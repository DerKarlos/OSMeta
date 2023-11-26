//! Loads and renders a glTF file as a scene.

use bevy::prelude::*;
use bevy_flycam::FlyCam;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;
use http_assets::HttpAssetReaderPlugin;
use sun::Sky;

type TileMap = tilemap::TileMap<8145>;

mod flycam;
mod http_assets;
mod sun;
mod tilemap;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
mod xr;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(HttpAssetReaderPlugin {
        base_url: "gltiles.osm2world.org/glb/".into(),
    });
    if std::env::args().any(|arg| arg == "xr") {
        #[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
        app.add_plugins(xr::Plugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.insert_resource(Msaa::Sample4) // Msaa::Sample4  Msaa::default()   -- Todo: tut nichts?
    //  .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
    //  .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(sun::Plugin)
        .add_plugins(flycam::Plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (load_next_tile, TileMap::update))
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Just for testing:
    #[allow(unused_mut)]
    let mut x: i32 = 17437;
    #[allow(unused_mut)]
    let mut y: i32 = 11371;

    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().expect("no window exists");
        let document = window.document().expect("no global document exist");
        let location = document.location().expect("no location exists");
        let raw_search = location.search().expect("no search exists");
        info!(?location);
        if let Some(addr) = raw_search.strip_prefix('?') {
            let (l, r) = addr.split_once(',').unwrap();
            x = l.parse().unwrap();
            y = r.parse().unwrap();
        }
    }

    commands.spawn((
        TileMap::new(&mut meshes),
        SpatialBundle {
            transform: Transform::from_xyz(
                -x as f32 * TileMap::TILE_SIZE,
                0.,
                -y as f32 * TileMap::TILE_SIZE,
            ),
            ..default()
        },
    ));
}

#[cfg(not(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32")))))]
/// HACK: we can't attach `LocalPlayer` to the xr player yet, so we need
/// to access the OpenXRTrackingRoot, but that doesn't exist without the xr feature.
/// So we create a dummy that is not attached to anything on platforms without the XR player.
#[derive(Component)]
pub struct OpenXRTrackingRoot;

fn load_next_tile(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut tilemap: Query<
        (Entity, &mut TileMap, &Transform),
        (Without<OpenXRTrackingRoot>, Without<FlyCam>, Without<Sky>),
    >,
    xr_pos: Query<
        &Transform,
        (
            With<OpenXRTrackingRoot>,
            Without<OpenXRTrackingRoot>,
            Without<TileMap>,
            Without<FlyCam>,
        ),
    >,
    flycam_pos: Query<
        &Transform,
        (
            With<FlyCam>,
            Without<OpenXRTrackingRoot>,
            Without<TileMap>,
            Without<Sky>,
        ),
    >,
    mut sky_pos: Query<
        &mut Transform,
        (
            With<Sky>,
            Without<OpenXRTrackingRoot>,
            Without<TileMap>,
            Without<FlyCam>,
        ),
    >,
) {
    let (id, mut tilemap, transform) = tilemap.single_mut();
    let pos = if let Ok(xr_pos) = xr_pos.get_single() {
        xr_pos.translation
    } else {
        flycam_pos.single().translation
    };
    sky_pos.single_mut().translation = pos;

    tilemap.load_next(
        id,
        &mut commands,
        &server,
        // FIXME: Maybe use https://crates.io/crates/big_space in order to be able to remove
        // the translation from the tilemap and instead just use its real coordinates.
        pos - transform.translation,
        sky_pos.single().scale.x,
    );
}
