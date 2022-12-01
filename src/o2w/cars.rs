//use std::error::Error;
//use std::{path::Path, sync::Arc};
//use glam::{Mat4, Vec3};

//use rend3::Renderer;
//use rend3::types::MaterialHandle;
//use rend3_gltf::instance_loaded_scene;
//use rend3_gltf::GltfSceneInstance;
//use crate::rendf;

use super::utils::*;


pub struct Cars {
    //settings: rend3_gltf::GltfLoadSettings,
    //file: Option<gltf::Gltf>,
    //loaded: Option<rend3_gltf::LoadedGltfScene>,
    //material_handles: Vec<MaterialHandle>,
    //material_ids: Vec<u8>,
    //material_rand: Vec<u8>,
    _positions: Vec<ScenePos>,
    _directions: Vec<f32>,
    //instances: Vec<GltfSceneInstance>,
}

impl Cars {
    pub fn new() -> Self {
        //println!("Cars new");
        Self {
            //settings: rend3_gltf::GltfLoadSettings {
            //    enable_directional: false,
            //    ..Default::default()
            //},
            //file: None,
            //loaded: None,
            //material_handles: Vec::new(),
            //material_ids: Vec::new(),
            //material_rand: Vec::new(),
            _positions: Vec::new(),
            _directions: Vec::new(),
            //instances: Vec::new(),
        }
    }

    fn _clone_car(&mut self, _r: f32, _g: f32, _b: f32, _material_id: u8, _renderer: &super::Renderer) -> u8 {

        /*
        // PBR material with all defaults except a single color.
        let material = rend3_routine::pbr::PbrMaterial {
            albedo: rend3_routine::pbr::AlbedoComponent::Value(glam::Vec4::new(r,g,b, 1.0)),
            ..rend3_routine::pbr::PbrMaterial::default()
        };
        let material_handle = renderer.add_material(material);
        // println!("m_h {:?}",material_handle); // ResourceHandle { refcount: 2, idx: 17 }
        self.material_handles.push(material_handle);

        material_id // return
        */
        0
    }


    pub fn _load(&mut self, _renderer: &super::Renderer) {
        //println!("Cars load");

        //t path = Path::new("/Users/karlos/Public/rend3/examples/car/car2.glb");
        let _car_path = models_path("car2.glb");

        /*
        let path = Path::new( &car_path);
        //println!("car path: {:?}", &path);
        let parent_directory = path.parent().unwrap();

        let data = load_data(&car_path);
      //let data = std::fs::read(&path).unwrap();

        let mut file = { gltf::Gltf::from_slice_without_validation(&data).unwrap() };
        let loaded = pollster::block_on(
            rend3_gltf::load_gltf_data(renderer, &mut file, &self.settings, |uri| {
                rend3_gltf::filesystem_io_func(&parent_directory, uri)
            })
        ).unwrap();
        //println!("loaded: {:?}", loaded);

        let white = self.clone_car(1.0, 1.0, 1.0, 0, renderer);
        let grey  = self.clone_car(0.2, 0.2, 0.2, 1, renderer);
        let black = self.clone_car(0.0, 0.0, 0.0, 2, renderer);
        let blue  = self.clone_car(0.0, 0.0, 0.8, 3, renderer);
        let red   = self.clone_car(1.0, 0.0, 0.0, 4, renderer);
        let green = self.clone_car(0.0, 0.1, 0.0, 5, renderer);
        //lf.material_ids = vec![red, blue, blue, white, white, white, black, black, black, grey, grey, grey, grey];
        self.material_ids = vec![green, red, blue, white, white, black, grey, grey];
        //println!("self.material_ids: {:?}",self.material_ids);

        // ***** own after change
        self.file = Some(file);
        self.loaded = Some(loaded);
        */
    }


    pub fn _add(&mut self, pos: ScenePos, direction: f32)
    {
        if rand::random::<f32>() < 0.6 { // good luck: add new car to draw ist  todo: parameter
            self._positions.push(pos);
            self._directions.push(direction);
        }
    }


    pub fn _instantiate(&mut self, _renderer: &super::Renderer)
    {
        /*
        // println!("cars instantiate {}",self.positions.len() );
        let loaded = if let Some(loaded) = &mut self.loaded {
            loaded
        } else {
            return;
        };
        let file = if let Some(file) = &self.file { file } else { return };

        for (index, position) in self.positions.iter().enumerate() {
            // rotate the car +/- 8 degr
            let direction = self.directions[index] + rand::random::<f32>() * rad(16.) - rad(8.);

            let mut pos: ScenePos = *position;
            pos.x = -pos.x; // What? Why ???, well, it works :-/
            // positions randomly different 95-105 %
            pos.x = pos.x + rand::random::<f32>() * 0.1 - 0.05;
            pos.z = pos.z + rand::random::<f32>() * 0.1 - 0.05;

            // scale down randomly up to 80%
            let scale = self.settings.scale * (1. - rand::random::<f32>() * 0.2);

            // println!("{}# cars pos {:?}  dir {}",index, pos, direction );

            if self.material_rand.is_empty() {
                self.material_rand = self.material_ids.clone();
            }
            let rand = self.material_rand.len() as f32 * rand::random::<f32>();
            let material_id = self.material_rand.remove(rand as usize);
            let material_handle_idx = self.material_handles[material_id as usize].get_raw().idx;
            let resource_handle = rend3::types::ResourceHandle::new(material_handle_idx);
            //println!("xxx: {:?} {:?} {:?} {:?}",material_rand,material_id,material_handle_idx,&resource_handle);
            let labeled = rend3_gltf::Labeled::new(resource_handle, None); // move to fn load ???
            loaded.materials[0] = labeled;
            // println!("material0: {:?}", &loaded.materials[0]);

            let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, 0.0, -direction, 0.0);
            let view = glam::Mat4::from_translation(-pos) * view;  // order is relevant!

            let instance = instance_loaded_scene::<Arc<dyn Error>>(
                renderer,
                loaded,
                file.nodes().collect(),
                &self.settings, // scale not used inside
                view * Mat4::from_scale(Vec3::new(
                    scale,
                    scale,
                    if renderer.handedness == rend3::types::Handedness::Left {
                        -scale
                    } else {
                        scale
                    },
                )),
            )
            .expect("Loading gltf scene");

            self.instances.push(instance);

        };
        */
    }
}
