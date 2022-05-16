use std::*;
use crate::rendf;

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
    pub fn add(&mut self, _uri: String) {
        self.adds += 1;
    }

    // needed for user message:
    pub fn len(&self) -> usize {
        0//self.map.len()
    }


    pub fn load(&mut self, _renderer: &rendf::Renderer) {
        // println!("textures LOAD");
        eprintln!("\n");
    }

}
