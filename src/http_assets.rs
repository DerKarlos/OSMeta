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
    base_url: String,
    /// Whether to load tiles from this path
    tile: bool,
}

impl HttpAssetReader {
    /// Creates a new `HttpAssetReader`. The path provided will be used to build URLs to query for assets.
    pub fn new(base_url: &str, tile: bool) -> Self {
        Self {
            base_url: base_url.into(),
            tile,
        }
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
                let mut bytes_compressed = vec![];
                let path = format!("{}{}", self.base_url, path);
                bevy_web_asset::WebAssetReader::Https
                    .read(Path::new(&path))
                    .await?
                    .read_to_end(&mut bytes_compressed)
                    .await?;

                let mut decoder = GzDecoder::new(bytes_compressed.as_slice());

                let mut bytes_uncompressed = vec![];

                decoder.read_to_end(&mut bytes_uncompressed)?;

                Ok(Box::new(VecReader::new(bytes_uncompressed)) as Box<Reader<'static>>)
            })
        } else {
            Box::pin(async move {
                let path = format!("{}{}", self.base_url, path);
                let mut bytes = vec![];
                bevy_web_asset::WebAssetReader::Https
                    .read(Path::new(&path))
                    .await?
                    .read_to_end(&mut bytes)
                    .await?;

                Ok(Box::new(VecReader::new(bytes)) as Box<Reader<'static>>)
            })
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
