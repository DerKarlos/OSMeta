////////////// main_pbf.rs ==>  example pbr.rs (or test.rs) //////////
use bevy::prelude::*;


#[derive(Component)]
struct Movable;



/// This example shows various ways to configure texture materials in 3D
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_flycam::NoCameraPlayerPlugin) // https://github.com/sburris0/bevy_flycam  ##  https://github.com/BlackPhlox/bevy_config_cam
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

/// sets up a scene with textured entities
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
    });


    // point light
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            point_light: PointLight {
                intensity: 10.0, // 1600 lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::WHITE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });


    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    })
     // NO: .insert(bevy_flycam::FlyCam)  // The camera is NOT moved by the fly-controls,
    ;// we want to see the light effects if the PBR rotates


    // load a texture and retrieve its aspect ratio
    let _normals_handle:   Handle<Image> = asset_server.load("ManholeCover004_Normal.jpg"); // jpg!!!
    let _texture_handle:   Handle<Image> = asset_server.load("ManholeCover004_Color.png");
    //   AoMRTextures Combined: ao/m/r_texture:  Texture with Ambient Occlusion in R, Metallic in G, and Roughness in B
    let _metallic_handle:  Handle<Image> = asset_server.load("ManholeCover004_ORM-G.png"); // Green
    let _occlusion_handle: Handle<Image> = asset_server.load("ManholeCover004_ORM-R.png"); // Red
    //t _roughness_handle: Handle<Image> = asset_server.load("ManholeCover004_ORM-B.png"); // Blue
    let _orm_o_orm_handle: Handle<Image> = asset_server.load("ManholeCover004_ORM.jpg");   // ALL

    let _texture_window:   Handle<Image> = asset_server.load("MarekCompositeWall00001_transparent.png");



    let aspect = 1.0;
    // create a new quad mesh. this is what we will apply the texture to
    let quad_width = 1.5;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    )))); // new Vec2, new shape, Mesh::from, meshes.add



    //// 1. Texturlayer [0]  ///////////
    let material_handle = materials.add(StandardMaterial {
        unlit: false, // no ???
    //  alpha_mode: AlphaMode::Blend, // transparency Blend: "analog" from base_color_texture alpha  FOR  T R E E

        base_color: Color::rgba( 1.0, 1.0, 1.0,   1.0, ),  // rgb  -- alpha 1=visible 0=transparent
        base_color_texture: Some(_texture_handle.clone()),

        // ROUGHNESS&/glossiness: perceived smoothness (0.0 exact reflection) or roughness (1.0 rough diffuse = default).

        metallic: 1.0, // dielectric (0.0, non-metal) or conductor (1.0, metal).
        metallic_roughness_texture: Some(_orm_o_orm_handle.clone()), // Green (and Blue?)
        reflectance: 0.5, // Specular intensity for non-metals. The default is 0.5, or 4% reflectance.


      //occlusion_texture: Some(occlusion_handle.clone()), // Red: Reaction to Hemispheric/ambient Light (heightmap) (white=1)
      // emissive:Color::rgb( 0.0, 0.0, 1.0, ), // self iluminated material
      // emissive_texture: Option<Handle<Image>>
        normal_map_texture: Some(_normals_handle.clone()),
        // flip_normal_map_y: true,
        ..default()
    });

    commands.    
    spawn(PbrBundle {
        mesh: quad_handle.clone(),
        material: material_handle,
        transform: Transform {
        //  translation: Vec3::new(0.0, 0.0, 0.0),
        //  rotation: Quat::from_rotation_x(-std::f32::consts::PI / 5.0),
            ..default()
        },
        ..default()
    })

//  .insert(bevy_flycam::FlyCam)    // Square moved by Fly-(Camera)-controls  (the square, not the camera is moved)
    .insert(Movable)    // square moves itselves (if fly-control is not used)
    ;



    //// 2. Texturlayer [1]  ///////////    
    let material_window = materials.add(StandardMaterial {
        unlit: false, // no ???
        alpha_mode: AlphaMode::Blend, // transparency Blend: "analog" from base_color_texture alpha
        base_color: Color::rgba( 1.0, 1.0, 1.0,   1.0, ),  // rgb  -- alpha 1=visible 0=transparent
        base_color_texture: Some(_texture_window.clone()),
        ..default()
    });

    // Intance a PBR-square
    commands.spawn(PbrBundle {
        mesh: quad_handle.clone(),
            material: material_window,
            ..default()
        })
    .insert(Movable);    // square moves itselves 


/* NOTES from the glft-loader:
LoadedAsset::new(StandardMaterial {
    base_color: Color::rgba(color[0], color[1], color[2], color[3]),
    base_color_texture,
    perceptual_roughness: pbr.roughness_factor(),
    metallic: pbr.metallic_factor(),
    metallic_roughness_texture,
    normal_map_texture,
    double_sided: material.double_sided(),
    cull_mode: if material.double_sided() {
        None
    } else {
        Some(Face::Back)
    },
    occlusion_texture,
    emissive: Color::rgba(emissive[0], emissive[1], emissive[2], 1.0),
    emissive_texture,
    unlit: material.unlit(),
    alpha_mode: alpha_mode(material),
    ..Default::default()
}),
*/




}


// temporal movement (not fly control)
fn movement(
    _input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in query.iter_mut() {
        //println!("xx {:?}", transform);
        let delta_n = 0.02*time.delta_seconds();
        let delta_rotation = Quat::from_euler(EulerRot::ZYX, 0.0, delta_n, 0.0,);
        transform.rotation *= delta_rotation; // multiply! means addition
    }
}
