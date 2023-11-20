//! Loads and renders a glTF file as a scene.

use bevy::prelude::*;
use tilemap::TileMap;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
use xr::XRPlugin;

#[cfg(all(feature = "xr", not(target_os = "macos")))]
use bevy_oxr::DefaultXrPlugins;

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
        app.add_plugins(DefaultXrPlugins).add_plugins(XRPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    sun::init(&mut app);
    flycam::init(&mut app);
    app.insert_resource(Msaa::Sample4) // Msaa::Sample4  Msaa::default()   -- Todo: tut nichts?
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_active_tile_zone, tilemap::update))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(TileMap::new(&mut meshes));
}

fn update_active_tile_zone(mut commands: Commands, mut tilemap: Query<&mut TileMap>) {
    let mut tilemap = tilemap.single_mut();
    tilemap.load(&mut commands, 17429, 11369);
    tilemap.load(&mut commands, 17429, 11370);
    tilemap.load(&mut commands, 17429, 11371);

    tilemap.load(&mut commands, 17430, 11369);
    tilemap.load(&mut commands, 17430, 11370);
    tilemap.load(&mut commands, 17430, 11371);

    tilemap.load(&mut commands, 17431, 11369);
    tilemap.load(&mut commands, 17431, 11370);
    tilemap.load(&mut commands, 17431, 11371);
}
