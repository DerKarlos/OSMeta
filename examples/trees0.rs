use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })

        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}


fn _create_tree() -> Mesh {

    //  2__3
    //  | /|
    //  |/ |
    //  0--1

    let w = 2.0;
    let h = 6.0;
    let n = 0.0;
    let positions = vec![
    /*       x    y    z  */
    /*0:*/ [-w,  n,	 n ],
    /*1:*/ [ w,  n,  n ],
    /*2:*/ [-w,  h,  n ],
    /*3:*/ [ w,  h,  n ],
    /*4:*/ [ n,  n,  w ],
    /*5:*/ [ n,  n, -w ],
    /*6:*/ [ n,  h,  w ],
    /*7:*/ [ n,  h, -w ],
    ];

    let uvs = vec![
    /*       u   v  is related to  x  y in this case  */
    /*0:*/ [ 1., 1. ],
    /*1:*/ [ 0., 1. ],
    /*2:*/ [ 1., 0. ],
    /*3:*/ [ 0., 0. ],
    /*4:*/ [ 1., 1. ],
    /*5:*/ [ 0., 1. ],
    /*6:*/ [ 1., 0. ],
    /*7:*/ [ 0., 0. ],
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
    mesh.set_indices(Some(Indices::U32(indices)));
    // pub fn compute_flat_normals(&mut self)   Panics if Indices are set  ==>>  NOT set !!! todo: issue?

    /**** THIS (MORE VERTICES)
    // compute only works with duplicate!
    mesh.duplicate_vertices();   // ERROR bevy_pbr::material: Mesh is missing requested attribute: Vertex_Normal (MeshVertexAttributeId(1), pipeline type: Some("bevy_pbr::material::MaterialPipeline<bevy_pbr::pbr_material::StandardMaterial>"))
    mesh.compute_flat_normals(); // thread 'TaskPool (0)' panicked at 'assertion failed: `(left == right)`  //   left: `8`,  //  right: `6`: MeshVertexAttributeId(1) has a different vertex count (6) than other attributes (8) in this mesh.', /Users/karlos/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_render-0.7.0/src/mesh/mesh/mod.rs:208:17
    OR THAT: ****/

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ]);

    /*
    let a = 1.00;
    let c = 0.01;
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vec![
        [c, c, c, a],
        [c, c, c, a],
        [c, c, c, a],
        [c, c, c, a],
        [c, c, c, a],
        [c, c, c, a],
        [c, c, c, a],
        [c, c, c, a],
    ]);
     */

    mesh
}


/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });


    // tree(s)
    let texture_handle = asset_server.load("arbaro_tree_broad_leaved.png");

    const MAX: usize = 7;
    let max = MAX as f32;
    for n in 0..MAX {   // ein mal Quad 10000 war ok.  10000 macht 13 FPS statt <=30.  Äh!! Nur die sichtbaren zählen?!
        let m = n as f32;
        let c = (max-m)/max;
        //println!("c: {}",c);
        let d = [c,c,c,1.0];
        let e = vec![d;8];
        let mut mesh = _create_tree();
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, e);

        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(
                StandardMaterial {
                    base_color_texture: Some(texture_handle.clone() ),
                    alpha_mode: bevy::pbr::AlphaMode::Mask(0.5), // Opaque, Mask(0.5), Blend,

                    double_sided: true, // needed to have both sides equal lighted
                    cull_mode: None,  // No cull of the back side.  Default is: Some(bevy::render::render_resource::Face::Back),
                    ..default()
                }
            ),
            transform: Transform::from_xyz(0.0, 0.0, -m ),
            ..default()
        })
        ;
    }

    //// light ////
    // Shadows do not work correct on my Macbook Air native, but in the browser it is ok.
    let mut _shadows = true;
    #[cfg(not(target_arch = "wasm32"))]
    { _shadows = false; }
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: _shadows,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });


    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-8.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}

