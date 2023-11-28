// Taken from https://github.com/lizelive/bevy_http and modified.

use async_fs::File;
use bevy::{
    asset::{
        io::{
            AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader,
            VecReader,
        },
        AsyncReadExt, AsyncWriteExt,
    },
    prelude::*,
    utils::BoxedFuture,
};
use flate2::read::GzDecoder;
use std::{
    collections::HashSet,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

/// A custom asset reader implementation that wraps a given asset reader implementation
struct HttpAssetReader {
    #[cfg(target_arch = "wasm32")]
    base_url: String,
    #[cfg(not(target_arch = "wasm32"))]
    client: surf::Client,
    /// Whether to load tiles from this path
    tile: bool,
    /// Used to ensure the same asset doesn't get its cache file written twice at the same time,
    /// as that depends on the OS whether it succeeds (could result in broken cache files).
    sync: Arc<RwLock<HashSet<PathBuf>>>,
}

impl HttpAssetReader {
    /// Creates a new `HttpAssetReader`. The path provided will be used to build URLs to query for assets.
    fn new(base_url: &str, tile: bool, sync: Arc<RwLock<HashSet<PathBuf>>>) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let base_url = surf::Url::parse(base_url).expect("invalid base url");

            let client = surf::Config::new().set_timeout(Some(std::time::Duration::from_secs(5)));
            let client = client.set_base_url(base_url);

            let client = client.try_into().expect("could not create http client");
            Self { client, tile, sync }
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self {
                base_url: base_url.into(),
                tile,
                sync,
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn fetch_bytes<'a>(&self, path: &str) -> Result<Box<Reader<'a>>, AssetReaderError> {
        info!("downloading {path}");
        use js_sys::Uint8Array;
        use wasm_bindgen::JsCast;
        use wasm_bindgen_futures::JsFuture;
        use web_sys::Response;

        fn js_value_to_err<'a>(
            context: &'a str,
        ) -> impl FnOnce(wasm_bindgen::JsValue) -> std::io::Error + 'a {
            move |value| {
                let message = match js_sys::JSON::stringify(&value) {
                    Ok(js_str) => format!("Failed to {context}: {js_str}"),
                    Err(_) => {
                        format!(
                        "Failed to {context} and also failed to stringify the JSValue of the error"
                    )
                    }
                };

                std::io::Error::new(std::io::ErrorKind::Other, message)
            }
        }

        let window = web_sys::window().unwrap();
        let resp_value =
            JsFuture::from(window.fetch_with_str(&format!("{}/{path}", self.base_url)))
                .await
                .map_err(js_value_to_err("fetch path"))?;
        let resp = resp_value
            .dyn_into::<Response>()
            .map_err(js_value_to_err("convert fetch to Response"))?;
        match resp.status() {
            200 => {
                let data = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
                let bytes = Uint8Array::new(&data).to_vec();
                let reader: Box<Reader> = Box::new(VecReader::new(bytes));
                Ok(reader)
            }
            404 => Err(AssetReaderError::NotFound(path.into())),
            status => Err(AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Encountered unexpected HTTP status {status}"),
            ))),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn fetch_bytes<'a>(&self, path: &str) -> Result<Box<Reader<'a>>, AssetReaderError> {
        info!("downloading {path}");
        let resp = self.client.get(path).await;

        trace!("fetched {resp:?} ... ");
        let mut resp = resp.map_err(|e| {
            AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("error fetching {path}: {e}"),
            ))
        })?;

        let status = resp.status();

        if !status.is_success() {
            let err = match status {
                surf::StatusCode::NotFound => AssetReaderError::NotFound(path.into()),
                _ => AssetReaderError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("bad status code: {status}"),
                )),
            };
            return Err(err);
        };

        let bytes = resp.body_bytes().await.map_err(|e| {
            AssetReaderError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("error getting bytes for {path}: {e}"),
            ))
        })?;
        let reader = bevy::asset::io::VecReader::new(bytes);
        Ok(Box::new(reader))
    }
}

impl AssetReader for HttpAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let cache_path = directories::ProjectDirs::from("org", "osmeta", "OSMeta")
                .map(|dirs| dirs.cache_dir().join(path));
            // Load from cache if the asset exists there.
            if let Some(cache_path) = cache_path.clone() {
                if cache_path.exists() {
                    let file = File::open(&cache_path).await?;
                    return Ok(Box::new(file) as Box<Reader>);
                }
            }
            let path = path.display().to_string();

            let mut bytes = vec![];
            if self.tile {
                // `tile://` urls are special for now, because we can't use `/` in the tile paths,
                // as that will cause texture loading to be attempted in the subfolders instead of the root.
                let (x, rest) = path.split_once('_').unwrap();
                // The tile servers we're using have their files gzipped, so we download that and unzip it
                // transparently and act as if there's a .glb file there.
                let path = format!("lod1/15/{x}/{rest}.gz");
                let mut bytes_compressed = Vec::new();
                self.fetch_bytes(&path)
                    .await?
                    .read_to_end(&mut bytes_compressed)
                    .await?;

                let mut decoder = GzDecoder::new(bytes_compressed.as_slice());

                decoder.read_to_end(&mut bytes)?;
            } else {
                self.fetch_bytes(&path)
                    .await?
                    .read_to_end(&mut bytes)
                    .await?;
            };
            if let Some(cache_path) = cache_path {
                // Write asset to cache, but ensure only one HttpAssetReader writes at any given point in time
                if self.sync.write().unwrap().insert(cache_path.clone()) {
                    async_fs::create_dir_all(cache_path.parent().unwrap()).await?;
                    File::create(&cache_path).await?.write_all(&bytes).await?;
                }
            }
            Ok(Box::new(VecReader::new(bytes)) as Box<Reader<'static>>)
        })
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move { Err(AssetReaderError::NotFound(path.into())) })
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        Box::pin(async move { Err(AssetReaderError::NotFound(path.into())) })
    }

    fn is_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, std::result::Result<bool, AssetReaderError>> {
        Box::pin(async move { Err(AssetReaderError::NotFound(path.into())) })
    }
}

/// A plugins that registers the `HttpAssetReader` as an asset source.
pub struct HttpAssetReaderPlugin {
    pub base_url: String,
}

impl Plugin for HttpAssetReaderPlugin {
    fn build(&self, app: &mut App) {
        let base_url = self.base_url.clone();
        let sync = Arc::new(RwLock::new(HashSet::new()));
        let sync2 = sync.clone();
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(move || {
                Box::new(HttpAssetReader::new(&base_url, false, sync.clone()))
            }),
        );
        let base_url = self.base_url.clone();
        app.register_asset_source(
            AssetSourceId::Name("tile".into()),
            AssetSource::build().with_reader(move || {
                Box::new(HttpAssetReader::new(&base_url, true, sync2.clone()))
            }),
        );
    }
}
