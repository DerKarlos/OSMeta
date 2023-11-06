// project uses
use super::frontend::Material;
use super::utils::*;

use super::textures::*;
//e crate::pbftile::*;
//e crate::frontend::Material_TextureLayer;

// extern and system uses
use std::*;

// no use crate::frontend   Do not mix it ub with the material of tree_d, frontend is used inside the code only

// use std::path::*;

// All shapes with the same pbf-material are merged into a geometry and model
// for it needs 1 draw-call per material (and there are no material-groups in a model yet)
// this may get sprlit into LoD and 400m tiles

#[derive(Debug)]
pub struct MaterialObject {
    positions: Vec<super::Position>, // For bevy an array is needed:  positions: Vec<[f32;3]>,
    uvs: Vec<super::Uv>, // For bevy an array is needed:  uvs: Vec<[f32;2]>,
    indices: Vec<u32>,
    url: String, // ??? option or >0 ???
    orm: String,
    nor: String,
    transp: u8,           // get_transparency
    depth_bias: f32,
    color: Option<super::PbfColor>, // rgb alpha
    cull: bool, //  model.cull = Cull::Back; //??? crulle/hide:  None(all there?)  Front Back FrontAndBack(all gone!)
    object: Option<super::Object>, // model: Option<Model>,
    _pbf_material_index: usize,  // debug output only
}

impl MaterialObject {

    pub fn get_lens(&self) -> (usize, usize) {
        (   self.positions.len(),
            self.indices.len()    )
    }

    pub fn new(pbf_material: &Material, _pbf_material_index: usize, texture_layer_index: usize) -> MaterialObject {

        let cull = pbf_material.doubleSided(); // false = Cull::None  true = Cull::Back

        // color: bevy::prelude::Color::rgb(0.0, 1.0, 1.0),
        let mut color: Option<super::PbfColor> = Some(super::shape_color(
            pbf_material.baseColorR() as f32,
            pbf_material.baseColorG() as f32,
            pbf_material.baseColorB() as f32,
            255.0,
        ));

        //println!("{} MaterialObject color: {:?}",_pbf_material_index,color);

        let mut url = "".to_string(); // tested;ok ääää
        let mut orm = "".to_string();
        let mut nor = "".to_string();
        let mut transp = 0;

        let texture_layers = &pbf_material.textureLayer;

        /***** T E S T  O N L Y: check if unsupported textures are used
        if texture_layers.len() > 1 {
            let texture_layer = &mut texture_layers[0];
            if texture_layer.get_emissiveTextureURI().len() > 0     { log("emissiveTextureURI"    ); }
            if texture_layer.get_reflectivityTextureURI().len() > 0 { log("reflectivityTextureURI"); }
            // texture_layer.get_displacementTextureURI().len() > 0 { log("displacementTextureURI"); }
        }
        if texture_layers.len() > 1 {
            let texture_layer = &mut texture_layers[1];
            if texture_layer.take_emissiveTextureURI().len()     > 0 { log("2.layer emissiveTextureURI"    ); }
            if texture_layer.take_reflectivityTextureURI().len() > 0 { log("2.layer reflectivityTextureURI"); }
            if texture_layer.take_ormTextureURI().len()          > 0 { log("2.layer ormTextureURI"         ); }
            if texture_layer.take_normalTextureURI().len()       > 0 { log("2.layer normalTextureURI"      ); }
            if texture_layer.take_displacementTextureURI().len() > 0 { log("2.layer displacementTextureURI"); }
        }
        ********/

        if texture_layers.len() > texture_layer_index {
            let texture_layer = &texture_layers[texture_layer_index];

            if !texture_layer.colorable() {
                color = None;
            };

            url = texture_layer.baseColorTextureURI().to_string();
            orm = texture_layer.ormTextureURI().to_string();
            nor = texture_layer.normalTextureURI().to_string();

            transp = pbf_material.transparency() as u8;
            if texture_layer_index > 0 {
                transp = 1
            }; // 1: used by windows etc.! (analog)

            // if url.len() < 77 { logs(format!( "{} create_object url: {:?}  orm: {:?}", this._pbf_material_index, url, orm )); }
        }; // todo: eliminate this double from pbftile

        // create and return a instance:
        let test = MaterialObject {
            positions: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            url,
            orm,
            nor,
            transp,
            depth_bias: texture_layer_index as f32, // https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html#structfield.depth_bias
            color,
            cull,
            object: None, // model
            _pbf_material_index,
        };
        //println!("MaterialObject: {:#?}", test);
        test
    }




    pub fn push_object(&mut self,
        positions: &Vec<ScenePos>, // in meter
        uvs: &mut Vec<super::Uv>,
        _merge: bool, // todo
    ) {

        for position in positions.iter() {
            // Todo:  Merge equal positions and use the same index

            // Indices

            self.indices.push(self.positions.len() as u32);
            // Positions
            self.positions.push( super::shape_position(
                /**/  position.x,
                /**/  position.y,
                /**/ -position.z, // +PBF => -Z ???
            ));
        }

        // Texture Coordinates
        for uv in uvs.iter() {
            self.uvs.push( *uv );
        }


    }



    ////  finish
    pub fn create_object(&mut self, textures: &mut Textures, renderer: &mut super::Renderer, test_index: usize) -> bool {

        if self.positions.is_empty() {
            //logs(format!("--- No positions for material {:?}", test_index));
            return false
        }


    //if true // ttt 3
    //&& self._pbf_material_index != 69 // Gibel
    //&& self._pbf_material_index != 71 // Wand  0 ODER 1 ???
    //&& self._pbf_material_index != 80 // Dach
    //{return false};
      
      //if &self.url == "textures/Plaster002_Color.jpg"  {return false};
      //if &self.url == "textures/Plaster002_Color.jpg"  {self.url = "textures/MarekCompositeWall00001_transparent.png".to_string() };
      //if &self.url == "textures/MarekCompositeWall00001_transparent.png"  {self.url = "textures/Plaster002_Color.jpg".to_string() };

        // println!("159 _pbf_material_index: {} {:#?}",self._pbf_material_index,&self); // .url



        let material_handle = super::pbr_material(
            self.color,
            &self.url,
            &self.orm,
            &self.nor,
            self.transp,
            self.depth_bias,
            self.cull, // some renderer need it at the material, some at the mesh
            textures,
            renderer,
        );

        // let texture = material_handle.
        // self.width =

        // If no textures: uses the color in the material, not in the object
        if self.url.is_empty() {
            self.color = None;
        }

        if self.positions.len() != self.uvs.len() {
            logs(format!(
                "!!!!!!!!!!!!! pos/uvs {}/{}",
                self.positions.len(),
                self.uvs.len()
            ));
        }

        // create a CPU-Object
        let object = super::Object::new(
            self.positions.clone(),
            self.uvs.clone(),
            self.indices.clone(),
            material_handle,
            self.cull, // some renderer need it at the material, some at the mesh
            test_index,
            renderer,
        ); // todo: no clone or "delete"
        self.object = Some(object);
        true

        //self.positions.clear(); todo
        //self.indices.clear();
        //self.uvs.clear();
    }
} // MaterialObject
