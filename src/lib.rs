//! Loads and renders a glTF file as a scene.

use std::f32::consts::FRAC_PI_2;

use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    pbr::NotShadowCaster,
    prelude::*,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_flycam::{FlyCam, MovementSettings};
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;
use bevy_screen_diagnostics::{
    Aggregate, ScreenDiagnostics, ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin,
};
use big_space::{
    world_query::{
        GridTransform, GridTransformItem, GridTransformOwned, GridTransformReadOnlyItem,
    },
    FloatingOriginPlugin, FloatingOriginSettings, GridCell,
};
use geocoord::{GeoCoord, EARTH_RADIUS};
use geoview::GeoView;
use http_assets::HttpAssetReaderPlugin;
use player::PlanetaryPosition;
use tilemap::{TileIndex, TileMap, TILE_ZOOM};

mod flycam;
mod geocoord;
mod geoview;
mod http_assets;
mod sun;
mod tilemap;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
mod xr;

type GridPrecision = i64;
type GalacticGrid = GridCell<GridPrecision>;
type GalacticTransform = GridTransform<GridPrecision>;
type GalacticTransformOwned = GridTransformOwned<GridPrecision>;
#[allow(dead_code)]
type GalacticTransformReadOnlyItem<'a> = GridTransformReadOnlyItem<'a, GridPrecision>;
#[allow(dead_code)]
type GalacticTransformItem<'a> = GridTransformItem<'a, GridPrecision>;

#[derive(Resource)]
struct Args {
    starting_position: PlanetaryPosition,
    xr: bool,
}

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
        std::env::set_var("RUST_BACKTRACE", "1");
        args.extend(std::env::args().skip(1));
    }

    let mut geo_coord = GeoCoord {
        lat: 48.1408, // Germany, Munic, Main railway station
        lon: 11.5577,
    };
    let mut elevation: f32 = 300.0;

    // GeoView to city center, Marienplatz
    let mut direction: f32 = -105.0; // Compass view-direction to Oeast-Southeast. 0 = Nord, -90 = East Todo: Why minus?
    let mut up_view: f32 = 75.0; // Up-view slightly down. -90 = down, 0 = horizontal 90 = Up

    let mut xr = false;

    for arg in args {
        let (k, v) = arg
            .split_once('=')
            .expect("arguments must be `key=value` pairs");
        match k {
            "lat" => geo_coord.lat = v.parse().unwrap(),
            "lon" => geo_coord.lon = v.parse().unwrap(),
            "ele" => elevation = v.parse().unwrap(),
            "view" => up_view = v.parse().unwrap(),
            "dir" => direction = v.parse().unwrap(),
            "xr" => xr = v.parse().unwrap(),
            other => panic!("unknown key `{other}`"),
        }
    }

    let mut app = App::new();
    app.insert_resource(Args {
        starting_position: geo_coord.to_cartesian(),
        xr,
    });

    let _start_view = GeoView {
        geo_coord,
        elevation,
        direction,
        up_view,
        distance: 6.,
        camera_fov: 7.,
    };

    let start_view = GeoView {
        // test only
        geo_coord: GeoCoord { lat: 33., lon: 0. }, // up,dir
        elevation: 5000000.,
        direction: 0.,
        up_view: 0.02,
        distance: 6.,
        camera_fov: 7.,
    };

    app.insert_resource(ViewDistance(2000.0));
    app.add_plugins(HttpAssetReaderPlugin {
        base_url: "gltiles.osm2world.org/glb/".into(),
    });
    // Offer assets via `embedded://`
    app.add_plugins(EmbeddedAssetPlugin::default());
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
        .add_plugins(geoview::Plugin { start_view })
        .insert_resource(TileMap::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (
                    // After recomputing the view-distance from the FPS
                    recompute_view_distance,
                    (
                        // Hide tiles that are now beyond the view-distance
                        get_main_camera_position.pipe(TileMap::hide_faraway_tiles),
                        // And load tiles that are now within the view-distance
                        get_main_camera_position
                            .pipe(TileMap::load_next)
                            .pipe(TileMap::load),
                    ),
                )
                    .chain(),
                TileMap::update,
            ),
        )
        .add_systems(Update, update_camera_orientations)
        .add_systems(PostUpdate, reposition_compass)
        .run();
}

fn setup(
    mut diags: ResMut<ScreenDiagnostics>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    diags.modify("fps").aggregate(Aggregate::Average);
    let rot = Quat::from_axis_angle(Vec3::X, FRAC_PI_2);
    let transform =
        Transform::from_translation(Vec3::NEG_Z * EARTH_RADIUS * 1.5).with_rotation(rot);

    let material = materials.add(StandardMaterial {
        fog_enabled: false,
        ..default()
    });

    // Rotational axis
    let mesh = meshes.add(
        shape::Cylinder {
            radius: 1000.0,
            height: EARTH_RADIUS * 6.0,
            resolution: 16,
            segments: 1,
        }
        .into(),
    );
    commands.spawn((
        PbrBundle {
            mesh,
            transform,
            material: material.clone(),
            ..default()
        },
        GalacticGrid::ZERO,
    ));

    // Equator
    let mesh = meshes.add(
        shape::Cylinder {
            radius: EARTH_RADIUS + 1000.0,
            height: 1.0,
            resolution: 64,
            segments: 1,
        }
        .into(),
    );
    commands.spawn((
        PbrBundle {
            mesh,
            transform: Transform::from_rotation(rot),
            material,
            ..default()
        },
        GalacticGrid::ZERO,
    ));
}

#[cfg(not(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32")))))]
/// HACK: we can't attach `LocalPlayer` to the xr player yet, so we need
/// to access the OpenXRTrackingRoot, but that doesn't exist without the xr feature.
/// So we create a dummy that is not attached to anything on platforms without the XR player.
#[derive(Component)]
pub struct OpenXRTrackingRoot;

#[derive(Resource, Copy, Clone)]
pub struct ViewDistance(f32);

fn recompute_view_distance(
    diagnostics: Res<DiagnosticsStore>,
    mut view_distance: ResMut<ViewDistance>,
) {
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
}

fn get_main_camera_position(
    player: player::Player,
    view_distance: Res<ViewDistance>,
) -> (TileIndex, f32) {
    let player = player.pos();

    let pos = player.pos();
    let origin = GeoCoord::from_cartesian(pos);
    let tile_size = origin.tile_size(TILE_ZOOM);
    let radius = view_distance.0 + tile_size + 0.5;
    let origin = origin.to_tile_coordinates(TILE_ZOOM);

    (origin.as_tile_index(), radius)
}

#[derive(Component)]
struct Compass;

mod player;

fn reposition_compass(
    mut compass: Query<
        GalacticTransform,
        (With<Compass>, Without<FlyCam>, Without<OpenXRTrackingRoot>),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
    player: player::Player,
) {
    if let Ok(mut compass) = compass.get_single_mut() {
        let player = player.pos();
        let directions = player.directions();
        compass.transform.translation = player.transform.translation - directions.up * 5.;
        *compass.cell = player.cell;
        compass.transform.look_to(directions.north, directions.up)
    } else {
        let mesh = shape::Plane::default();
        let mesh = meshes.add(mesh.into());
        let image = server.load("embedded://compass.png");
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(image),
            unlit: true,
            cull_mode: None,
            perceptual_roughness: 1.0,
            fog_enabled: false,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh,
                material,
                ..default()
            },
            GalacticGrid::ZERO,
            Compass,
            NotShadowCaster,
        ));
    }
}

fn update_camera_orientations(
    mut movement_settings: ResMut<MovementSettings>,
    fly_cam: Query<GalacticTransform, With<FlyCam>>,
    space: Res<FloatingOriginSettings>,
) {
    movement_settings.up = fly_cam
        .single() // the only FlyCam's calactic position <grid,f32>
        .position_double(&space)
        .normalize() // direction from galactic NULL = from the Earth center
        .as_vec3();
}

fn pull_to_ground(
    time: Res<Time>,
    mut tracking_root_query: Query<GalacticTransform, With<OpenXRTrackingRoot>>,
    space: Res<FloatingOriginSettings>,
) {
    let Ok(mut root) = tracking_root_query.get_single_mut() else {
        return;
    };

    let adjustment_rate = (time.delta_seconds() * 10.0).min(1.0);

    // Lower player onto sphere
    let real_pos = root.position_double(&space);
    let up = real_pos.normalize();
    let diff = up * EARTH_RADIUS as f64 - real_pos;
    root.transform.translation += diff.as_vec3() * adjustment_rate;

    // Rotate player to be upright on sphere
    let angle_diff = Quat::from_rotation_arc(root.transform.up(), up.as_vec3());
    root.transform
        .rotate(Quat::IDENTITY.slerp(angle_diff, adjustment_rate));
}
