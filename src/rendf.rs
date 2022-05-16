use std::fs::read;

use crate::pbftile::*;
use crate::rendf;
use crate::cars::*;
use crate::textures::*;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

// This was not easy to find out!:
pub type Renderer<'a, 'w, 's, 'x, 'y, 'z> = (
    &'a mut bevy::prelude::Commands<'w, 's>,
    &'a mut bevy::prelude::ResMut<'x, Assets<Mesh>>,
    &'a mut bevy::prelude::ResMut<'y, Assets<StandardMaterial>>,
    &'a     bevy::prelude::Res   <'z, AssetServer>,
);

pub type Color          = bevy::prelude::Color;
//b type MaterialHandle = bevy::asset::Handle<dyn bevy::pbr::prelude::Material<ExtractedAsset = Type, PreparedAsset = Type, Param = Type>>;
pub type Position       = [f32;3];
pub type Uv             = [f32;2];


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




// ----------------------------------------------------------------------


pub struct Object {
//  object_handle: Option<rend3::types::ObjectHandle>,
}

impl Object {
    pub fn new(
        vertex_positions: Vec<Position>,
        uv_positions: Vec<Uv>,
        index_data: Vec<u32>,
        material_handle: Handle<StandardMaterial>,
        cull: bool,
        _nr: usize,
        (commands,meshes,_materials, _asset_server):  &mut rendf::Renderer,
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
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: material_handle,
            ..default()
        });


        /*
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



pub fn pbr_material(
    //
    color: Option<Color>,
    url: &str,
    orm: &str,
    nor: &str,
    transparency: u8,
    _textures: &mut Textures,
    (_commands,_meshes,materials,asset_server): &mut Renderer,
) -> Handle<StandardMaterial>
{


    // Add PBR material with all defaults
    let mut material = StandardMaterial {
        // base_color_texture: Some(texture_handle.clone() ),
        // alpha_mode: bevy::pbr::AlphaMode::Mask(0.5), // Opaque, Mask(0.5), Blend,
        // double_sided: true, // needed to have both sides equal lighted
        // cull_mode: None,  // No cull of the back side.  Default is: Some(bevy::render::render_resource::Face::Back),
        ..default()
    };

    if let Some(color) = color {
        // println!("## create_object color: {:?}",         color);
        material.base_color = color;
    }


    if !url.is_empty()
    && !url.starts_with("data:")
    && !url.ends_with(".svg") { // url.len() > 0_usize 
        let texture_handle = asset_server.load(&format!("../../o2w/{}", url)); // Enum bevy::render::texture::ImageFormat
        material.base_color_texture = Some(texture_handle.clone() );
      //alpha_mode: bevy::pbr::AlphaMode::Mask(0.5), // Opaque, Mask(0.5), Blend,
      //double_sided: true, // needed to have both sides equal lighted
      //cull_mode: None,  // No cull of the back side.  Default is: Some(bevy::render::render_resource::Face::Back),

        if !orm.is_empty() {
            let texture_handle = asset_server.load(&format!("../../o2w/{}", orm)); // Enum bevy::render::texture::ImageFormat
            material.metallic_roughness_texture = Some(texture_handle.clone() ); // occlusion_texture ???
          //material.ao_factor = Some(1.0);
          //material.metallic_factor = Some(1.0);
          //material.roughness_factor = Some(-2.0);
        }

        if !nor.is_empty() {
            let texture_handle = asset_server.load(&format!("../../o2w/{}", nor)); // Enum bevy::render::texture::ImageFormat
            material.normal_map_texture = Some(texture_handle.clone() );
            // O2W needs Down, not Up
            // Tricomponent because in Gimp all 3 have values, Up because Down causes light colors on the shadow side
        }

    }

    /*

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
    ) -> OSM2World {

        // let _bytes = load_pbr_bytes( "../rend3/assets/4402/2828.o2w.pbf".to_string() ); // "../rend3/assets/{}/{}.o2w.pbf"
        let mut textures = Textures::new();
        let mut cars = Cars::new();

        let mut pbf_tile = PbfTile::new(4402, 2828);
        pbf_tile.load( &mut (commands, meshes, materials, asset_server), &mut textures, &mut cars );

        OSM2World{}
    }

}
