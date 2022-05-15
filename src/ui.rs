use bevy::prelude::{Component,PositionType};
use bevy::ecs::{query::With,system::{Res,Query}};
use bevy::text::{Text,TextStyle,TextSection};
use bevy::ui::{Val,Style,entity::TextBundle};
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::math::Rect;
use bevy::asset::AssetServer;
use bevy::utils::default;

#[derive(Component)]
pub struct StatsText;

// add this lines to main::new:
//   .add_plugin(FrameTimeDiagnosticsPlugin::default())
//   .add_system(ui_system)

// add     asset_server: Res<AssetServer>,   to setup-parameterrs
// add this lines to setup-code:
//      // UI with FPS
//      commands.spawn_bundle(UiCameraBundle::default());
//      commands.spawn_bundle(create_ui(&asset_server)).insert(StatsText);



pub fn create_ui(asset_server: &Res<AssetServer>) -> TextBundle {
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




pub fn ui_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<StatsText>>,
)
{
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[0].value = format!("FPS: {:.2}", average);
        }
    };
}
