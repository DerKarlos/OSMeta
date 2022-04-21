use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)

        // Show Framerate in Console
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .add_startup_system(setup)
        .add_system(movement)
        .add_system(ui_system)
        .run();
}

#[derive(Component)]
struct Movable;

#[derive(Component)]
struct StatsText;


fn create_ui(asset_server: &Res<AssetServer>) -> TextBundle {
    //
    TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 1.0, 1.0),
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }

}

fn _create_tree() -> Mesh {

    //  2__3
    //  | /|
    //  |/ |
    //  0--1

    let positions = vec![
    /*       x    y    z  */
    /*0:*/ [-3.,	0.,	 0. ],
    /*1:*/ [ 3.,  0.,  0. ],
    /*2:*/ [-3.,  6.,  0. ],
    /*3:*/ [ 3.,  6.,  0. ],
    /*4:*/ [ 0.,  0., -3. ],
    /*5:*/ [ 0.,  0.,  3. ],
    /*6:*/ [ 0.,  6., -3. ],
    /*7:*/ [ 0.,  6.,  3. ],
    ];

    let uvs = vec![
    /*       u   v  is related to  x  y in this case  */
    /*3:*/ [ 1., 1. ],
    /*2:*/ [ 0., 1. ],
    /*1:*/ [ 1., 0. ],
    /*0:*/ [ 0., 0. ],
    /*3:*/ [ 1., 1. ],
    /*2:*/ [ 0., 1. ],
    /*1:*/ [ 1., 0. ],
    /*0:*/ [ 0., 0. ],
    ];

    let indices = vec![
    0,	 1,	 3,
    0,	 3,	 2,
    4,	 5,	 7,
    4,	 7,	 6,
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);


    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    // TODO: Crossing squares are not supported? Issue? no fallback by ATTRIBUTE_NORMAL? (ok, see: loader.js)
    //sh.duplicate_vertices(); // ERROR bevy_pbr::material: Mesh is missing requested attribute: Vertex_Normal (MeshVertexAttributeId(1), pipeline type: Some("bevy_pbr::material::MaterialPipeline<bevy_pbr::pbr_material::StandardMaterial>"))
    //sh.compute_flat_normals(); // thread 'TaskPool (0)' panicked at 'assertion failed: `(left == right)`  //   left: `8`,  //  right: `6`: MeshVertexAttributeId(1) has a different vertex count (6) than other attributes (8) in this mesh.', /Users/karlos/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_render-0.7.0/src/mesh/mesh/mod.rs:208:17
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0], // ?????
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ]);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh

}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // UI with FPS
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(create_ui(&asset_server)).insert(StatsText);


    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });


    // tree(s)
    let texture_handle = asset_server.load("arbaro_tree_broad_leaved.png");

    for n in 0..10 {   // ein mal Quad 10000 war ok
        //  mesh: meshes.add(create_tree()),
        let size = 3.0;

        // 1st side
        commands.spawn_bundle(PbrBundle {

            mesh: meshes.add(Mesh::from(shape::Quad { size: bevy::math::vec2(size, size), flip: false })),
            material: materials.add(
                StandardMaterial {
                    base_color_texture: Some(texture_handle.clone() ),
                    alpha_mode: bevy::pbr::AlphaMode::Mask(0.5), // Opaque, Mask(0.5), Blend,

                    double_sided: true, // needed to have both sides equal lighted
                    cull_mode: None,  // No cull of the back side.  Default is: Some(bevy::render::render_resource::Face::Back),
                    ..default()
                }
            ),
            transform: Transform::from_xyz(0.0, size/2., -n as f32),
            ..default()
        })
        .insert(Movable)
        ;

        // 2nd side crossed
        let rotation = Quat::from_euler(EulerRot::ZYX, 0.0, std::f32::consts::FRAC_PI_2, 0.0);
        commands.spawn_bundle(PbrBundle {

            mesh: meshes.add(Mesh::from(shape::Quad { size: bevy::math::vec2(size, size), flip: false })),
            material: materials.add(
                StandardMaterial {
                    base_color_texture: Some(texture_handle.clone() ),
                    alpha_mode: bevy::pbr::AlphaMode::Mask(0.5), // Opaque, Mask(0.5), Blend,

                    double_sided: true, // needed to have both sides equal lighted
                    cull_mode: None,  // No cull of the back side.  Default is: Some(bevy::render::render_resource::Face::Back),
                    ..default()
                }
            ),
            transform:
                Transform::from_xyz(0.0, size/2., -n as f32) *
                Transform::from_rotation(rotation),
            ..default()
        })
        .insert(Movable)
        ;

    }



    //// light ////
    // Shadows do not work correct on my Macbook Air native, but in the browser it is ok.
    let mut _shadows = true;
    #[cfg(not(target_arch = "wasm32"))]
    { _shadows = false; }
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: _shadows,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });


    //// camera ////
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-8.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}



fn movement(
    _input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in query.iter_mut() {
        //println!("xx {:?}", transform);
        let delta_y = 1.00*time.delta_seconds();
        let delta_rotation = Quat::from_euler(EulerRot::ZYX, 0.0, delta_y, 0.0);
        transform.rotation *= delta_rotation; // multiply! means addition
        let scale = transform.scale.x * (1.-0.02*time.delta_seconds());  // just for fun
        transform.scale = Vec3::new(scale,scale,scale);
    }
}


fn ui_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[0].value = format!("FPS: {:.2}", average);
        }
    };
}
