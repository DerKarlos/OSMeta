//! Implements loader for a Gzip compressed asset.
// Adjusted from the example at https://github.com/bevyengine/bevy/blob/208ecb53dc5aed4baefc5ca5dc73bfdfee16a6e6/examples/asset/asset_decompression.rs

use bevy::gltf::{Gltf, GltfError, GltfLoader};
use bevy::{
    asset::{
        io::{Reader, VecReader},
        AssetLoader, AsyncReadExt, ErasedLoadedAsset, LoadContext,
    },
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use flate2::read::GzDecoder;
use std::io::prelude::*;

#[derive(Asset, TypePath)]
pub struct GzAsset {
    pub uncompressed: ErasedLoadedAsset,
}

#[derive(Default)]
pub struct GzAssetLoader;

impl AssetLoader for GzAssetLoader {
    type Asset = Gltf;
    type Settings = ();
    type Error = GltfError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes_compressed = Vec::new();
            warn!("decompressing glb.gz file");

            reader.read_to_end(&mut bytes_compressed).await?;

            let mut decoder = GzDecoder::new(bytes_compressed.as_slice());

            let mut bytes_uncompressed = Vec::new();

            decoder.read_to_end(&mut bytes_uncompressed)?;

            // Now that we have decompressed the asset, let's pass it back to the
            // context to continue loading

            let mut reader = VecReader::new(bytes_uncompressed);

            GltfLoader {
                supported_compressed_formats: default(),
                custom_vertex_attributes: default(),
            }
            .load(&mut reader, settings, load_context)
            .await
        })
    }

    fn extensions(&self) -> &[&str] {
        &["glb.gz"]
    }
}
