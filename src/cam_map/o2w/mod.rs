// rendf.rs => 02w/mod.rs

// This file is an adapter/wrapper to bevy for the other Rust sources in ths module.
// There are other wrappers for the Crate rend3 and tree-d
// There are some data types, variing for different renderer,
// the main class of the module and some helpers
// Important is the fn pbr_material to create a bevy material form tile data
// and the "class" Object to show 3D meshes in the GPU

// Todo ? Remane module to "map" ?
// mod osmscene;   // In TypeScript it manages all the tile loading. Place on Earth, shown in 3D. There could be more than one to teleport to.
// mod geopos;     // Position on Earth
// mod geoview;    // Position and viewangles
// mod cameraview; // geoview plus camera settings

// Todo ? Move in an extra sub-module "osm2world" ?
mod frontend;   // read the binary PBF file and greate Rust structs
mod pbftile;    // Main
mod instance_parameter; // nomen est omen
mod materialobject; // An mesh with geometry and material
mod textures;   // handle the textures for the material
mod cars;       // read car.glm and instantiate cars of different color, size and position
pub mod viewtile;   // a (sub-) square of the pbftile with all its material-objects
mod utils;      // nomen est omen
mod print;      // Debug outputs. Todo? use print trait ?


use std::fs::read;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

// Todo ? Move in an extra sub-module "osm2world" ?
// pub use viewer::*;
//use geopos::*;
//use geoview::*;
//pub(crate) mod viewer;     // canvas handler

use pbftile::*;
use textures::*;
use cars::*;

use crate::cam_map::TileName;


// This was not easy to find out!:
// A tubble(?) with all needed renderer variables ot draw an 3D object
pub type Renderer<'a, 's, 't, 'w, 'x, 'y, 'z> = (
    &'a mut bevy::prelude::Commands<'w, 's>,
    &'a mut bevy::prelude::ResMut<'t, Assets<Mesh>>,
    &'a mut bevy::prelude::ResMut<'x, Assets<StandardMaterial>>,
    &'a     bevy::prelude::Res   <'z, AssetServer>,
);

// Some data types, variing for different renderer,
pub type PbfColor       = bevy::prelude::Color;
//b type MaterialHandle = bevy::asset::Handle<dyn bevy::pbr::prelude::Material<ExtractedAsset = Type, PreparedAsset = Type, Param = Type>>;
pub type Position       = [f32;3];
pub type Uv             = [f32;2];


//// Some helper functions ////

// color: bevy::prelude::Color::rgb(0.0, 1.0, 1.0),
pub fn shape_color(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color::rgba(r,g,b,a)
}

pub fn shape_position(x: f32, y:f32, z:f32) -> Position {
    /* For rend3 an array is needed:
    ScenePos::new(
        /**/ x,
        /**/ y,
        /**/ z,
    )
    */
    // For bevy an array is needed:
    [
        /**/ x,
        /**/ y,
        /**/ z,
    ]
}

pub fn shape_uv(u: f32, v: f32) -> Uv {
    //  Vec2::new(0.0, 0.0));  // rend3
    [ u, v ]
}


// Get the 3D-vector tile and material textures local - or from a web-api
pub static LOCAL: bool = true;




pub fn load_pbr_bytes(pbf_url: String) -> Vec<u8> {
    println!("pbf_url: {:?}",pbf_url);

    if LOCAL {
        read(pbf_url.clone()).unwrap()
        // wasm browser message: "operation not supported on this platform"
    } else {
        /*
        let url = pbf_url;
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(3*60))
            .build().unwrap();
        let response = client.get(url).send().unwrap();
        response.bytes().unwrap().to_vec()
        // iii: progress bar only with .async => https://gist.github.com/giuliano-oliveira/4d11d6b3bb003dba3a1b53f43d81b30d
        */
        Vec::new()
    }

}



// bevy system: When a asset loading/changing or unloading is done, this system runs
// It just checks, what happend and may print it in the log. May be more is needed ??
// Not used, does not work?
pub fn _fixup_images(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    // if ev_asset.len() < 91 {  println!("ev_asset {:?}",ev_asset.len() );    }
    for (i,ev) in ev_asset.iter().enumerate() {
        match ev {
            AssetEvent::Created { handle } |
            AssetEvent::Modified { handle } => {
                // a texture was just loaded or changed!

                let _texture = assets.get_mut(handle).unwrap();
                // ^ unwrap is OK, because we know it is loaded now
                println!("CreaMod #{} {:?}",i,handle );

            }
            AssetEvent::Removed { handle } => {
                println!("Removed #{} {:?}",i,handle);
                // an image was unloaded
            }
        }
    }
}

// ----------------------------------------------------------------------

// A "class" Object to show 3D meshes in the GPU
#[derive(Debug)]
pub struct Object {
    //  object_handle: Option<rend3::types::ObjectHandle>,
}

impl Object {
    pub fn new(
        vertex_positions: Vec<Position>,
        uv_positions: Vec<Uv>,
        index_data: Vec<u32>,
        material_handle: Handle<StandardMaterial>,
        _cull: bool,
        _nr: usize,
        (commands,meshes,_materials, _asset_server):  &mut Renderer,
    ) -> Object {
        // logs(format!("{}# Object - poss: {:?}  indices: {:?}", _nr, vertex_positions.len(), index_data.len() ));

        // if vertex_positions.len() < 10 {
        //     logs(format!("pos: {:?}", vertex_positions ));
        //     logs(format!("ind: {:?}", index_data ));
        // }

        // create and NO return a instance:
        let this = Object {
            // object_handle: None
        };


        // Create Object and calculate smooth normals based on vertices
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertex_positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_positions);
        mesh.set_indices(Some(Indices::U32(index_data)));
        mesh.duplicate_vertices();
        mesh.compute_flat_normals();

        // commands.
        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: material_handle,
            ..default()
        });


        /* OLD rend3 code
        let mesh_builder = rend3::types::MeshBuilder::new(
            vertex_positions,
            rend3::types::Handedness::Right, // PBF data is ::Right not ::Left
        )
        .with_vertex_uv0(uv_positions.to_vec())  // only if texture?
        .with_indices(index_data);

        let mut mesh = mesh_builder.build().unwrap();

        if cull {
            mesh.double_side();
        }

        // Add mesh to renderer's world.
        // All handles are refcounted, so we only need to hang onto the handle until we make an object.
        let mesh_handle = renderer.add_mesh(mesh);

        let location = glam::Vec3::new(0.0, 0.0, 0.0);
        let view = glam::Mat4::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0);
        let view = view * glam::Mat4::from_translation(location);

        // Combine the mesh and the material with a location to give an object.
        let object = rend3::types::Object {
            mesh_kind: rend3::types::ObjectMeshKind::Static(mesh_handle),
            material: material_handle,
            transform: view, // glam::Mat4::IDENTITY,
        };
        // Creating an object will hold onto both the mesh and the material, even if they are deleted.
        // We need to keep the object handle alive.
        this.object_handle = Some(renderer.add_object(object));
        */

        // RETURN instance:
        this
    }

} // MaterialObject


// To preload main teextures: not used
pub fn load_texture( uri:String, asset_server: &Res<AssetServer> ) -> Handle<Image>{
        asset_server.load(&format!("../../o2w/{}", uri)) // "../../o2w/{}"
}

pub fn pbr_material(
    //
    color: Option<Color>,
    url: &str,
    _orm: &str,
    _nor: &str,
    transparency: u8,
    cull: bool,
    _textures: &mut Textures,
    (_commands,_meshes,materials,asset_server): &mut Renderer,
) -> Handle<StandardMaterial>
{
    // None: No cull of the back sides.  Default is cull/hide back sides: Some(bevy::render::render_resource::Face::Back),
    let cull_mode    = if cull {None} else {Some(bevy::render::render_resource::Face::Back)};
    let double_sided = if cull {true} else {false};

    // if transparency<3 {logn(transparency as f32)};
    let alpha_mode =
    match transparency {
        1 => bevy::pbr::AlphaMode::Blend,      // 1: TRUE,   windows!  (analog)
        2 => bevy::pbr::AlphaMode::Mask(0.5),  // 2: BINARY, trees (not analog)
        _ => bevy::pbr::AlphaMode::Opaque,     // 0, 3.. no tramsparency    // 3: FALSE   "undurchsichtig"
    };

    // Add PBR material with all defaults
    let mut material = StandardMaterial {
        unlit: false, // no ???
        alpha_mode,   //: bevy::pbr::AlphaMode::Mask(0.5), // Opaque, Mask(0.5), Blend,
        double_sided, // needed to have both sides equal lighted
        cull_mode,
        ..default()
    };

    if let Some(color) = color {
        // println!("## create_object color: {:?}",         color);
        material.base_color = color;
    }

    if !url.is_empty()
    && !url.starts_with("data:")
    && !url.ends_with(".svg") { // url.len() > 0_usize
        // println!("material _url: {:?}",url);
        let texture_handle = load_texture(url.to_string(), asset_server);
        material.base_color_texture = Some(texture_handle.clone() );

        if !_orm.is_empty() {
            //println!("orm: {}",_orm);
            let texture_handle = load_texture(_orm.to_string(), asset_server);
            material.metallic_roughness_texture = Some(texture_handle.clone() ); // metallic-Green and roughness-Blue. Not occlusion-Red  ???
            material.metallic    = 1.0; // dielectric (0.0, non-metal) or conductor (1.0, metal). needed???
            material.reflectance = 0.5; // Specular intensity for non-metals. The default is 0.5, or 4% reflectance.
          //from gltf-loader:
          //material.perceptual_roughness: pbr.roughness_factor(),
          //from rent3?
          //material.roughness_factor = Some(-2.0);
          //material.metallic_factor = Some(1.0);
          //material.ao_factor = Some(1.0);
        }

        if !_nor.is_empty() {
            //println!("nor {}",_nor);
            let texture_handle = load_texture(_nor.to_string(), asset_server);
            material.normal_map_texture = Some(texture_handle.clone() );
            // O2W needs Down, not Up
            // Tricomponent because in Gimp all 3 have values, Up because Down causes light colors on the shadow side
        }

        // println!("material: {:#?}", material);

/* OLD rend3 code

StandardMaterial
{
    base_color: Rgba { red: 180.0, green: 118.0, blue: 69.0, alpha: 255.0 },
    base_color_texture: Some(StrongHandle<Image>(AssetPathId(AssetPathId(SourcePathId(5004750218444946552), LabelId(6298619649789039366))))),
    emissive: Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 },
    emissive_texture: None,
    perceptual_roughness: 0.089,
    metallic: 0.01,
    metallic_roughness_texture: None, reflectance: 0.5,
    normal_map_texture: None,
    flip_normal_map_y: false,
    occlusion_texture: None,
    double_sided: false,
    cull_mode: Some(Back),
    unlit: false,
    alpha_mode: Opaque
}

StandardMaterial
{
    base_color: Rgba{ red: 180.0, green: 118.0, blue: 69.0, alpha: 255.0 },
    base_color_texture: Some(StrongHandle<Image>(AssetPathId(AssetPathId(SourcePathId(5004750218444946552), LabelId(6298619649789039366))))),
    emissive: Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 },
    emissive_texture: None,
    perceptual_roughness: 0.089,
    metallic: 1.0,
    metallic_roughness_texture: Some(StrongHandle<Image>(AssetPathId(AssetPathId(SourcePathId(12441045366667236010), LabelId(6298619649789039366))))), reflectance: 0.5,
    normal_map_texture: Some(StrongHandle<Image>(AssetPathId(AssetPathId(SourcePathId(18179905451368387593), LabelId(6298619649789039366))))),
    flip_normal_map_y: false,
    occlusion_texture: None,
    double_sided: true,
    cull_mode: Some(Back),
    unlit: false,
    alpha_mode: Opaque
}
*/

    }

    /*  OLD rend3 code

    if !url.is_empty() { // url.len() > 0_usize {

        ....

        // Todo??? dis.len

        // if transparency<3 {logn(transparency as f32)};

        match transparency {
            1 => {
                let _color = glam::Vec4::new(0.0, 0.0, 1.0, 1.0);
                material.transparency = rend3_routine::pbr::Transparency::Blend;
                material.reflectance = rend3_routine::pbr::MaterialComponent::Value(0.66);
            } // 1: TRUE, windows!    (analog)
            2 => material.transparency = rend3_routine::pbr::Transparency::Cutout { cutout: 0.5 }, // 2: BINARY, trees (not analog)
            _ => (), // 0, 3.. no tramsparency                                                 // 3: FALSE
        }

        /*******
        pub wrap_s: Wrapping, default = Wrapping::Repeat
        pub wrap_t: Wrapping,
        pub wrap_r: Wrapping,
        let wrap = pbf_material.textureLayer[nrTextur].wrap // REPEAT = 1;  CLAMP = 2;
        let addressmode = (wrap == 2) ? BABYLON.Texture.CLAMP_ADDRESSMODE : BABYLON.Texture.WRAP_ADDRESSMODE;
        texture.wrapU = addressmode; // 0 CLAMP_ADDRESSMODE, 1 WRAP_ADDRESSMODE  4 BORDER_ADDRESSMODE
        texture.wrapV = addressmode;
        ******/
    }

    renderer.add_material(material) // return
    */

    materials.add(material) // return handle

}



pub struct OSM2World {
}

impl OSM2World {

    pub fn new( commands:     &mut Commands,
                meshes:       &mut ResMut<Assets<Mesh>>,
                materials:    &mut ResMut<Assets<StandardMaterial>>,
                asset_server: &    Res<AssetServer>,
                name:         &    TileName,
                start_pos:    Vec3,
    ) -> OSM2World {

        // let _bytes = load_pbr_bytes( "../rend3/assets/4402/2828.o2w.pbf".to_string() ); // "../rend3/assets/{}/{}.o2w.pbf"
        let mut textures = Textures::new();
        let mut cars = Cars::new();

        let mut pbf_tile = PbfTile::new(name.x, name.y, start_pos);
        pbf_tile.load( &mut (commands, meshes, materials, asset_server), &mut textures, &mut cars );

        OSM2World{}
    }

}
