use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;

pub type Position       = [f32;3];
pub type Uv             = [f32;2];

// Prepatation for O2W rendering
pub type RendererParameter1<'a, 'r> = bevy::prelude::Commands<'a, 'r>;
pub type RendererParameter2<'a> = bevy::prelude::ResMut<'a, Assets<Mesh>>;

pub fn bevy_render(
    positions: Vec<Position>,
    uvs:       Vec<Uv>,
    indices:   Vec<u32>,
    commands:  &mut Commands,
    meshes:    &mut ResMut<Assets<Mesh>>,
)
{

    // Create Object and calculate smooth normals based on vertices
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.duplicate_vertices();
    mesh.compute_flat_normals();
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        ..default()
    });
}


pub struct Object {
}

impl Object {
    pub fn new() -> Object {
        Object{
        }
    }

    // Render geometry, NO colors yet
    pub fn render( &mut self,
        positions: Vec<Position>,
        uvs:       Vec<Uv>,
        indices:   Vec<u32>,
        commands:  &mut RendererParameter1,
        meshes:    &mut RendererParameter2,
    ) {

        bevy_render(
            positions,
            uvs,
            indices,
            commands,
            meshes,
        );

    }
}



pub struct Example {
}

impl Example {

    pub fn new() -> Example {
        Example{

        }
    }

    // creates and renders 2 crossed rectangles. Default color pink
    pub fn render( &mut self,
        commands:  &mut RendererParameter1,
        meshes:    &mut RendererParameter2,
    ) {

        let w = 2.0; // width
        let h = 6.0; // height
        let n = 0.0; // null
        let positions = vec![
            [-w,  n,  n ],
            [ w,  n,  n ],
            [-w,  h,  n ],
            [ w,  h,  n ],

            [ n,  n,  w ],
            [ n,  n, -w ],
            [ n,  h,  w ],
            [ n,  h, -w ],
        ];

        let uvs = vec![
            [ 1., 1. ],
            [ 0., 1. ],
            [ 1., 0. ],
            [ 0., 0. ],

            [ 1., 1. ],
            [ 0., 1. ],
            [ 1., 0. ],
            [ 0., 0. ],
        ];

        let indices = vec![
        0,	 1,	 3,
        0,	 3,	 2,

        4,	 7,	 5,
        4,	 6,	 7,
        ];

        let mut object = Object::new();
        object.render(
            positions,
            uvs,
            indices,
            commands,
            meshes,
        );

    }

}




fn main() {

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

 
fn setup(
//  asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    /**** asset test
    load_extra_assets(
        &mut commands,
        asset_server,
    ); ****/

    // example
    let mut example = Example::new();
    example.render(
        &mut commands,
        &mut meshes,
    );

    // plane, colored
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 6.0 })),
        material: materials.add(bevy::prelude::Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });


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


    //// camera ////
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-8.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
