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
    pub base_url: String,
    /// Whether to load tiles from this path
    pub tile: bool,
    /// Used to ensure the same asset doesn't get its cache file written twice at the same time,
    /// as that depends on the OS whether it succeeds (could result in broken cache files).
    pub sync: Arc<RwLock<HashSet<PathBuf>>>,
    pub cache_path: Option<PathBuf>,
}
impl HttpAssetReader {}

impl AssetReader for HttpAssetReader {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let cache_path = self.cache_path.as_ref().map(|p| p.join(path));
            // Load from cache if the asset exists there.
            if let Some(cache_path) = &cache_path {
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
                let path = format!("{}lod1/15/{x}/{rest}.gz", self.base_url);
                let mut bytes_compressed = Vec::new();
                bevy_web_asset::WebAssetReader::Https
                    .read(Path::new(&path))
                    .await?
                    .read_to_end(&mut bytes_compressed)
                    .await?;

                let mut decoder = GzDecoder::new(bytes_compressed.as_slice());

                decoder.read_to_end(&mut bytes)?;
            } else {
                let path = format!("{}{path}", self.base_url);
                bevy_web_asset::WebAssetReader::Https
                    .read(Path::new(&path))
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
        let cache_path = directories::ProjectDirs::from("org", "osmeta", "OSMeta")
            .map(|dirs| dirs.cache_dir().to_owned());
        let cache_path2 = cache_path.clone();
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build().with_reader(move || {
                Box::new(HttpAssetReader {
                    base_url: base_url.clone(),
                    tile: false,
                    sync: sync.clone(),
                    cache_path: cache_path2.clone(),
                })
            }),
        );
        let base_url = self.base_url.clone();
        app.register_asset_source(
            AssetSourceId::Name("tile".into()),
            AssetSource::build().with_reader(move || {
                info!(?cache_path);
                Box::new(HttpAssetReader {
                    base_url: base_url.clone(),
                    tile: true,
                    sync: sync2.clone(),
                    cache_path: cache_path.clone(),
                })
            }),
        );
    }
}
