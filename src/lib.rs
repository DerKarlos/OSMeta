//! Loads and renders a glTF file as a scene.

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_flycam::{FlyCam, MovementSettings};
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;
use bevy_screen_diagnostics::{
    Aggregate, ScreenDiagnostics, ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin,
};
use big_space::{FloatingOriginPlugin, FloatingOriginSettings, GridCell};
use geopos::{GeoPos, EARTH_RADIUS};
use glam::DVec3;
use http_assets::HttpAssetReaderPlugin;
use tilemap::{TileMap, TILE_ZOOM};

mod flycam;
mod geopos;
mod http_assets;
mod sun;
mod tilemap;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
mod xr;

type GridPrecision = i64;
type GalacticGrid = GridCell<GridPrecision>;

#[derive(Resource)]
struct Args {
    starting_position: DVec3,
    xr: bool,
}

#[bevy_main]
pub fn main() {
    let mut args: Vec<String> = vec![];

    std::env::set_var("RUST_BACKTRACE", "1");

    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().expect("no window exists");
        let document = window.document().expect("no global document exist");
        let location = document.location().expect("no location exists");
        let raw_search = location.search().expect("no search exists");
        info!(?location);
        if let Some(addr) = raw_search.strip_prefix('?') {
            args.extend(addr.split('&').map(Into::into));
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        args.extend(std::env::args().skip(1));
    }

    let mut pos = GeoPos {
        lat: 48.14077,
        lon: 11.55741,
    };

    let mut xr = false;

    for arg in args {
        let (k, v) = arg
            .split_once('=')
            .expect("arguments must be `key=value` pairs");
        match k {
            "lat" => pos.lat = v.parse().unwrap(),
            "lon" => pos.lon = v.parse().unwrap(),
            "xr" => xr = v.parse().unwrap(),
            other => panic!("unknown key `{other}`"),
        }
    }

    let mut app = App::new();
    app.insert_resource(Args {
        starting_position: pos.to_cartesian(),
        xr,
    });
    app.insert_resource(ViewDistance(2000.0));
    app.add_plugins(HttpAssetReaderPlugin {
        base_url: "gltiles.osm2world.org/glb/".into(),
    });
    app.add_plugins(bevy_web_asset::WebAssetPlugin {
        user_agent: Some("osmeta 0.1.0".into()),
    });
    if xr {
        #[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
        app.add_plugins(xr::Plugin);
        app.add_systems(Update, pull_to_ground);
    } else {
        app.add_plugins(DefaultPlugins.build().disable::<TransformPlugin>());
    }
    app.add_plugins(FloatingOriginPlugin::<GridPrecision>::default());
    app.insert_resource(Msaa::Sample4) // Msaa::Sample4  Msaa::default()   -- Todo: tut nichts?
        .add_plugins(ScreenDiagnosticsPlugin {
            timestep: 1.0,
            ..default()
        })
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(sun::Plugin)
        .add_plugins(flycam::Plugin)
        .insert_resource(TileMap::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (load_next_tile, TileMap::update))
        .add_systems(Update, update_camera_orientations)
        .run();
}

fn setup(mut diags: ResMut<ScreenDiagnostics>) {
    diags.modify("fps").aggregate(Aggregate::Average);
}

#[cfg(not(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32")))))]
/// HACK: we can't attach `LocalPlayer` to the xr player yet, so we need
/// to access the OpenXRTrackingRoot, but that doesn't exist without the xr feature.
/// So we create a dummy that is not attached to anything on platforms without the XR player.
#[derive(Component)]
pub struct OpenXRTrackingRoot;

#[derive(Resource)]
pub struct ViewDistance(f32);

fn load_next_tile(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut tilemap: ResMut<TileMap>,
    xr_pos: Query<(&Transform, &GalacticGrid), (With<OpenXRTrackingRoot>, Without<FlyCam>)>,
    flycam_pos: Query<(&Transform, &GalacticGrid), (With<FlyCam>, Without<OpenXRTrackingRoot>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    diagnostics: Res<DiagnosticsStore>,
    mut view_distance: ResMut<ViewDistance>,
    space: Res<FloatingOriginSettings>,
) {
    let (pos, grid) = if let Ok(xr_pos) = xr_pos.get_single() {
        xr_pos
    } else {
        flycam_pos.single()
    };

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps.smoothed() {
            if fps < 40.0 {
                view_distance.0 *= 0.99;
            } else if fps > 59.5 {
                view_distance.0 *= 1.01;
            }
            view_distance.0 = view_distance.0.clamp(1000.0, 10000.0);
        }
    }

    let pos = space.grid_position_double(grid, pos);
    let origin = GeoPos::from_cartesian(pos);
    let tile_size = origin.tile_size(TILE_ZOOM);
    let radius = view_distance.0 / tile_size + 0.5;
    let origin = origin.to_tile_coordinates(TILE_ZOOM);

    tilemap.load_next(
        &mut commands,
        &server,
        &mut meshes,
        &space,
        // FIXME: Maybe use https://crates.io/crates/big_space in order to be able to remove
        // the translation from the tilemap and instead just use its real coordinates.
        origin,
        radius,
    );
}

fn update_camera_orientations(
    mut movement_settings: ResMut<MovementSettings>,
    fly_cam: Query<(&Transform, &GalacticGrid), With<FlyCam>>,
    space: Res<FloatingOriginSettings>,
) {
    let (transform, grid) = fly_cam.single();
    movement_settings.up = space
        .grid_position_double(grid, transform)
        .normalize()
        .as_vec3();
}

fn pull_to_ground(
    time: Res<Time>,
    mut tracking_root_query: Query<(&mut Transform, &GalacticGrid), With<OpenXRTrackingRoot>>,
    space: Res<FloatingOriginSettings>,
) {
    let Ok((mut root, grid)) = tracking_root_query.get_single_mut() else {
        return;
    };

    let adjustment_rate = (time.delta_seconds() * 10.0).min(1.0);

    // Lower player onto sphere
    let real_pos = space.grid_position_double(grid, &root);
    let up = real_pos.normalize();
    let diff = up * EARTH_RADIUS as f64 - real_pos;
    root.translation += diff.as_vec3() * adjustment_rate;

    // Rotate player to be upright on sphere
    let angle_diff = Quat::from_rotation_arc(root.up(), up.as_vec3());
    root.rotate(Quat::IDENTITY.slerp(angle_diff, adjustment_rate));
}
