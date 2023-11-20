//! Loads and renders a glTF file as a scene.

use bevy::prelude::*;

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

fn update_active_tile_zone(mut commands: Commands, mut tilemap: Query<(Entity, &mut TileMap)>) {
    let (id, mut tilemap) = tilemap.single_mut();
    tilemap.load(id, &mut commands, 17429, 11369);
    tilemap.load(id, &mut commands, 17429, 11370);
    tilemap.load(id, &mut commands, 17429, 11371);

    tilemap.load(id, &mut commands, 17430, 11369);
    tilemap.load(id, &mut commands, 17430, 11370);
    tilemap.load(id, &mut commands, 17430, 11371);

    tilemap.load(id, &mut commands, 17431, 11369);
    tilemap.load(id, &mut commands, 17431, 11370);
    tilemap.load(id, &mut commands, 17431, 11371);
}
