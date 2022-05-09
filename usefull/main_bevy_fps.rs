use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

/// This example provides a 2D benchmark.
///
/// Usage: spawn more entities by clicking on the screen.
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "BevyMark".to_string(),
            width: 800.,
            height: 600.,
            present_mode: PresentMode::Immediate,
            resizable: true,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .add_startup_system(setup)
        .add_system(counter_system)
        .run();
}


#[derive(Component)]
struct StatsText;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
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
        })
        .insert(StatsText);

}

fn counter_system(
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
