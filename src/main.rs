use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::PrimitiveTopology;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

//e bevy::app::ScheduleRunnerSettings;
//e bevy::utils::Duration;
//e bevy::core::FixedTimestep;

use rendf::*;

mod rendf;
mod pbftile;
mod frontend;
mod viewtile;
mod instance_parameter;
mod materialobject;
mod o2w_utils;
mod print;
mod textures;
mod cars;

fn main() {

    App::new()
    //  .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64( 1.0 / 20.0, )))  // no different
    //  .with_run_criteria(FixedTimestep::step( (1.0/60.0) as f64))
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
                        color: bevy::prelude::Color::rgb(0.0, 1.0, 1.0),
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

    // OpenStreetMap !!!
    let _osm2world = OSM2World::new( &mut commands, &mut meshes );

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
    let x = 100.;
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-8.0*x, 10.0*x, 20.0*x).looking_at(Vec3::ZERO, Vec3::Y),
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
