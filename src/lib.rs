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
use geopos::{GeoPos, EARTH_RADIUS};
use http_assets::HttpAssetReaderPlugin;
use sun::Sky;
use tilemap::{TileMap, TILE_ZOOM};

mod flycam;
mod geopos;
mod http_assets;
mod sun;
mod tilemap;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
mod xr;

#[derive(Resource)]
struct StartingPosition(Vec3);

#[bevy_main]
pub fn main() {
    let mut args: Vec<String> = vec![];

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
    app.insert_resource(StartingPosition(pos.to_cartesian()));
    app.add_plugins(HttpAssetReaderPlugin {
        base_url: "gltiles.osm2world.org/glb/".into(),
    });
    if xr {
        #[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
        app.add_plugins(xr::Plugin);
        app.add_systems(Update, pull_to_ground);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.insert_resource(Msaa::Sample4) // Msaa::Sample4  Msaa::default()   -- Todo: tut nichts?
        .add_plugins(ScreenDiagnosticsPlugin {
            timestep: 1.0,
            ..default()
        })
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(sun::Plugin)
        .add_plugins(flycam::Plugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (load_next_tile, TileMap::update))
        .add_systems(Update, update_camera_orientations)
        .run();
}

fn setup(mut commands: Commands, mut diags: ResMut<ScreenDiagnostics>, pos: Res<StartingPosition>) {
    diags.modify("fps").aggregate(Aggregate::Average);

    commands.spawn((
        TileMap::default(),
        SpatialBundle {
            transform: Transform::from_translation(-pos.0),
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
            Without<Sky>,
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
    mut sky: Query<
        &mut Transform,
        (
            With<Sky>,
            Without<OpenXRTrackingRoot>,
            Without<TileMap>,
            Without<FlyCam>,
        ),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    diagnostics: Res<DiagnosticsStore>,
    mut fog: Query<&mut FogSettings>,
) {
    let (id, mut tilemap, transform) = tilemap.single_mut();
    let pos = if let Ok(xr_pos) = xr_pos.get_single() {
        xr_pos.translation
    } else {
        flycam_pos.single().translation
    };
    let mut sky = sky.single_mut();
    sky.translation = pos;

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps) = fps.smoothed() {
            if fps < 40.0 {
                sky.scale = Vec3::splat(sky.scale.x * 0.99)
            } else if fps > 59.5 {
                sky.scale = Vec3::splat(sky.scale.x * 1.01)
            }
            sky.scale = Vec3::splat(sky.scale.x.clamp(1000.0, 10000.0));
            fog.single_mut().falloff = FogFalloff::from_visibility_colors(
                sky.scale.x, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            );
        }
    }

    let origin = GeoPos::from_cartesian(pos - transform.translation);
    let tile_size = origin.tile_size(TILE_ZOOM);
    let radius = sky.scale.x / tile_size + 0.5;
    let origin = origin.to_tile_coordinates(TILE_ZOOM);

    tilemap.load_next(
        id,
        &mut commands,
        &server,
        &mut meshes,
        // FIXME: Maybe use https://crates.io/crates/big_space in order to be able to remove
        // the translation from the tilemap and instead just use its real coordinates.
        origin,
        radius,
    );
}

fn update_camera_orientations(
    mut movement_settings: ResMut<MovementSettings>,
    fly_cam: Query<&Transform, (With<FlyCam>, Without<TileMap>)>,
    tilemap: Query<&Transform, (With<TileMap>, Without<FlyCam>)>,
) {
    movement_settings.up =
        (fly_cam.single().translation - tilemap.single().translation).normalize();
}

fn pull_to_ground(
    time: Res<Time>,
    mut tracking_root_query: Query<&mut Transform, (With<OpenXRTrackingRoot>, Without<TileMap>)>,
    tilemap: Query<&Transform, (With<TileMap>, Without<OpenXRTrackingRoot>)>,
) {
    let Ok(mut root) = tracking_root_query.get_single_mut() else {
        return;
    };
    let tilemap = tilemap.single();

    let adjustment_rate = (time.delta_seconds() * 10.0).min(1.0);

    // Lower player onto sphere
    let real_pos = root.translation.as_dvec3() - tilemap.translation.as_dvec3();
    let up = real_pos.normalize();
    let diff = up * EARTH_RADIUS as f64 - real_pos;
    root.translation += diff.as_vec3() * adjustment_rate;

    // Rotate player to be upright on sphere
    let angle_diff = Quat::from_rotation_arc(root.up(), up.as_vec3());
    root.rotate(Quat::IDENTITY.slerp(angle_diff, adjustment_rate));
}
