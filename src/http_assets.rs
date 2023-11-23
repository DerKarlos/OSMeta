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
use std::{
    io::Read,
    path::{Path, PathBuf},
};

use std::pin::Pin;
use std::task::Poll;

/// A custom asset reader implementation that wraps a given asset reader implementation
pub struct HttpAssetReader {
    client: surf::Client,
    /// Whether to load tiles from this path
    tile: bool,
}

impl HttpAssetReader {
    /// Creates a new `HttpAssetReader`. The path provided will be used to build URLs to query for assets.
    pub fn new(base_url: &str, tile: bool) -> Self {
        let base_url = surf::Url::parse(base_url).expect("invalid base url");

        let client = surf::Config::new().set_timeout(Some(std::time::Duration::from_secs(5)));
        let client = client.set_base_url(base_url);

        let client = client.try_into().expect("could not create http client");

        Self { client, tile }
    }

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

struct EmptyPathStream;

impl futures_core::Stream for EmptyPathStream {
    type Item = PathBuf;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        Poll::Ready(None)
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
        Box::pin(async move {
            let path = path.display().to_string();
            let meta_path = path + ".meta";
            self.fetch_bytes(&meta_path).await
        })
    }

    fn read_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        let stream: Box<PathStream> = Box::new(EmptyPathStream);
        error!("Reading directories is not supported with the HttpAssetReader");
        Box::pin(async move { Ok(stream) })
    }

    fn is_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, std::result::Result<bool, AssetReaderError>> {
        error!("Reading directories is not supported with the HttpAssetReader");
        Box::pin(async move { Ok(false) })
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
