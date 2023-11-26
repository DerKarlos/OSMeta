// Taken from https://github.com/lizelive/bevy_http and modified.

use bevy::{
    asset::{
        io::{
            AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader,
            VecReader,
        },
        AsyncReadExt,
    },
    prelude::*,
    utils::BoxedFuture,
};
use flate2::read::GzDecoder;
use std::{io::Read, path::Path};

/// A custom asset reader implementation that wraps a given asset reader implementation
pub struct HttpAssetReader {
    #[cfg(target_arch = "wasm32")]
    base_url: String,
    #[cfg(not(target_arch = "wasm32"))]
    client: surf::Client,
    /// Whether to load tiles from this path
    tile: bool,
}

impl HttpAssetReader {
    /// Creates a new `HttpAssetReader`. The path provided will be used to build URLs to query for assets.
    pub fn new(base_url: &str, tile: bool) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let base_url = surf::Url::parse(base_url).expect("invalid base url");

            let client = surf::Config::new().set_timeout(Some(std::time::Duration::from_secs(5)));
            let client = client.set_base_url(base_url);

            let client = client.try_into().expect("could not create http client");
            Self { client, tile }
        }
        #[cfg(target_arch = "wasm32")]
        {
            Self {
                base_url: base_url.into(),
                tile,
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    async fn fetch_bytes<'a>(&self, path: &str) -> Result<Box<Reader<'a>>, AssetReaderError> {
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
        let path = path.display().to_string();
        if self.tile {
            let (x, rest) = path.split_once('_').unwrap();
            let path = format!("lod1/15/{x}/{rest}.gz");
            Box::pin(async move {
                let mut bytes_compressed = Vec::new();
                self.fetch_bytes(&path)
                    .await?
                    .read_to_end(&mut bytes_compressed)
                    .await?;

                let mut decoder = GzDecoder::new(bytes_compressed.as_slice());

                let mut bytes_uncompressed = Vec::new();

                decoder.read_to_end(&mut bytes_uncompressed)?;

                Ok(Box::new(VecReader::new(bytes_uncompressed)) as Box<Reader<'static>>)
            })
        } else {
            Box::pin(async move { self.fetch_bytes(&path).await })
        }
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
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build()
                .with_reader(move || Box::new(HttpAssetReader::new(&base_url, false))),
        );
        let base_url = self.base_url.clone();
        app.register_asset_source(
            AssetSourceId::Name("tile".into()),
            AssetSource::build()
                .with_reader(move || Box::new(HttpAssetReader::new(&base_url, true))),
        );
    }
}
