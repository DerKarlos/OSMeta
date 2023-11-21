//! Loads and renders a glTF file as a scene.

#![allow(clippy::type_complexity)]

use bevy::prelude::*;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;

type TileMap = tilemap::TileMap<8145>;

mod flycam;
mod sun;
mod tilemap;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
mod xr;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    if std::env::args().any(|arg| arg == "xr") {
        #[cfg(all(feature = "xr", not(target_os = "macos")))]
        app.add_plugins(xr::Plugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.insert_resource(Msaa::Sample4) // Msaa::Sample4  Msaa::default()   -- Todo: tut nichts?
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(sun::Plugin)
        .add_plugins(flycam::Plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_active_tile_zone, TileMap::update))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Just for testing:
    const X0: i32 = 17430;
    const Y0: i32 = 11370;
    commands.spawn((
        TileMap::new(&mut meshes),
        SpatialBundle {
            transform: Transform::from_xyz(
                -X0 as f32 * TileMap::TILE_SIZE,
                0.,
                -Y0 as f32 * TileMap::TILE_SIZE,
            ),
            ..default()
        },
    ));
}

#[derive(Component)]
/// A dummy struct connected to all players local to this computer.
/// Used to make sure all players have a map to walk on.
pub struct LocalPlayer;

#[cfg(not(all(feature = "xr", not(target_os = "macos"))))]
/// HACK: we can't attach `LocalPlayer` to the xr player yet, so we need
/// to access the OpenXRTrackingRoot, but that doesn't exist without the xr feature
type OpenXRTrackingRoot = LocalPlayer;

fn update_active_tile_zone(
    mut commands: Commands,
    mut tilemap: Query<
        (Entity, &mut TileMap, &Transform),
        (Without<OpenXRTrackingRoot>, Without<LocalPlayer>),
    >,
    player_pos: Query<&Transform, Or<(With<OpenXRTrackingRoot>, With<LocalPlayer>)>>,
) {
    let (id, mut tilemap, transform) = tilemap.single_mut();
    for pos in player_pos.iter() {
        tilemap.load_nearest(id, &mut commands, pos.translation - transform.translation);
    }
}
