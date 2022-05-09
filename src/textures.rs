use image::GenericImageView;
use std::collections::HashMap;
use std::*;

// SVG!
#[cfg(not(target_arch = "wasm32"))]
use resvg::render;
#[cfg(not(target_arch = "wasm32"))]
use tiny_skia::Pixmap;
#[cfg(not(target_arch = "wasm32"))]
use usvg::FitTo;
#[cfg(not(target_arch = "wasm32"))]
use usvg::Options;
#[cfg(not(target_arch = "wasm32"))]
use usvg::Tree;


//use rend3::types::*;
//use rend3::*;

use crate::rendf;
use crate::o2w_utils::*;

//// Texture-Handler ////
pub struct Textures {
//  map: HashMap<String, bevy::asset::Handle<bevy::render::render_resource::Texture>>,
    pub adds: u32,
}

impl Textures {
    pub fn new() -> Textures {
        Textures {
        // map: HashMap::new(),
            adds: 0,
        }
    }

    // Add the texture URI, load it later!
    pub fn add(&mut self, uri: String) {
        self.adds += 1;
        let len = cmp::min(uri.len(), 77);

        if len == 0 {
            return;
        }

        //logs(format!(
        //    "{} textures.add uri: {}",
        //    self.map.len(),
        //    &uri[0..len]
        //));

        let null: usize = 0;
    //  let texture_handle_null: ResourceHandle<Texture> = ResourceHandle::<Texture>::new(null);
    //  let _old = self.map.insert(uri, texture_handle_null);
        // if let Some(_) = _old { println!("   texture old: {}", uri); }
    }

    // needed for user message:
    pub fn len(&self) -> usize {
        0//self.map.len()
    }

    /***  Load Assert:
    file or www: osm_utils => rend3 (option, "http*:")
    WASM or native-paht: rend3::assert_loader.get_assert (read/request/ndk_qlue-andorid)
    data: nur in rend3_gltf.filesystem_io_func
    .svg: -/-
    ***/

    pub fn load(&mut self, renderer: &rendf::Renderer) {
        // println!("textures LOAD");

        /*
        let mut keys = Vec::new();
        for key in self.map.keys() {
            keys.push(key.clone());
        }

        for key in keys {
            // println!("key: {} ", key);

            let octet_stream_header = "data:";
            let bytes = if let Some(base64_data) = key.strip_prefix(octet_stream_header) {
                let (_mime, rest) = base64_data.split_once(';').unwrap();
                let (encoding, data) = rest.split_once(',').unwrap();
                assert_eq!(encoding, "base64");
                base64::decode(data).unwrap()
            } else {
                // LOAD !!!!! todo: parallel by async/await
                load_texture(&key)
            };

            let texture_checker = if key.ends_with(".svg") {
                self.svg(bytes)
            } else {
                // Add texture to renderer's world.
                let image_checker = image::load_from_memory(&bytes.to_vec()).expect("Failed to load image from memory");
                let image_checker_rgba8 = image_checker.to_rgba8();

                /*
                rend3::types::Texture {
                    label: Option::None,
                    data: image_checker_rgba8.to_vec(),
                    format: rend3::types::TextureFormat::Rgba8UnormSrgb,
                    size: glam::UVec2::new(image_checker.dimensions().0, image_checker.dimensions().1),
                    mip_count: rend3::types::MipmapCount::ONE,
                    mip_source: rend3::types::MipmapSource::Uploaded,
                }
                */
            };

            let texture_handle = renderer.add_texture_2d(texture_checker);
            let k2 = key.to_string();
            self.map.insert(k2, texture_handle);

            eprint!("."); // e = error = imidiately visible, no waiting for new line
        }
        */

        eprintln!("\n");
    }

    /*
    fn svg(&self, svg_data: Vec<u8>) -> bevy::render::render_resource::Texture {
        let options = Options::default();
        let svg_rtree = Tree::from_data(&svg_data, &options).unwrap(); // csv grafic tree (?)
        let pixmap_size = svg_rtree.svg_node().size.to_screen_size(); // reads the height/width="nnn" of the CSV-File/bytes
        let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        // read svg-tree and render into pixELmap
        render(&svg_rtree, FitTo::Original, pixmap.as_mut()).unwrap();

        //let texture_checker =
        rend3::types::Texture {
            label: Option::None,
            data: pixmap.data().to_vec(),
            format: rend3::types::TextureFormat::Rgba8UnormSrgb,
            size: glam::UVec2::new(pixmap.width(), pixmap.height()),
            mip_count: rend3::types::MipmapCount::ONE,
            mip_source: rend3::types::MipmapSource::Uploaded,
        }
    }
    */

    /*
    pub fn get(&self, uri: &str) -> bevy::asset::Handle<bevy::render::render_resource::Texture> {
        //println!("get {}", uri);

        // return
        self.map.get(uri).unwrap().clone()
    }
    */
}
