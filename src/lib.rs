//! Loads and renders a glTF file as a scene.

use bevy::prelude::*;
use bevy_http::HttpAssetReaderPlugin;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;
use gz::{GzAsset, GzAssetLoader};

type TileMap = tilemap::TileMap<8145>;

mod flycam;
mod gz;
mod sun;
mod tilemap;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
mod xr;

#[bevy_main]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    app.add_plugins(HttpAssetReaderPlugin {
        id: "osm2world".into(),
        base_url: "https://gltiles.osm2world.org/glb/lod1/15/".into(),
        fake_slash: "NOT_A_DIR_SEPARATOR".into(),
    });
    if std::env::args().any(|arg| arg == "xr") {
        #[cfg(all(feature = "xr", not(target_os = "macos")))]
        app.add_plugins(xr::Plugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.init_asset::<GzAsset>()
        .init_asset_loader::<GzAssetLoader>();
    app.insert_resource(Msaa::Sample4) // Msaa::Sample4  Msaa::default()   -- Todo: tut nichts?
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(sun::Plugin)
        .add_plugins(flycam::Plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (load_next_tile, TileMap::update))
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

fn load_next_tile(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut tilemap: Query<
        (Entity, &mut TileMap, &Transform),
        (Without<OpenXRTrackingRoot>, Without<LocalPlayer>),
    >,
    player_pos: Query<&Transform, Or<(With<OpenXRTrackingRoot>, With<LocalPlayer>)>>,
) {
    let (id, mut tilemap, transform) = tilemap.single_mut();
    for pos in player_pos.iter() {
        tilemap.load_next(
            id,
            &mut commands,
            &server,
            // FIXME: Maybe use https://crates.io/crates/big_space in order to be able to remove
            // the translation from the tilemap and instead just use its real coordinates.
            pos.translation - transform.translation,
        );
    }
}
