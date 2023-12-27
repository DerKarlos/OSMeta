//! Loads and renders a glTF file as a scene.

use bevy::{pbr::NotShadowCaster, prelude::*};
use bevy_embedded_assets::EmbeddedAssetPlugin;
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
    FloatingOriginPlugin, GridCell,
};
use flycam::update_camera_orientations;
use geocoord::GeoCoord;
use geoview::GeoView;
use http_assets::HttpAssetReaderPlugin;
use player::PlanetaryPosition;
use tilemap::TileMap;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use xr::pull_to_ground;

mod f4control;
mod flycam;
mod geocoord;
mod geoview;
mod http_assets;
mod space;
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
struct StartingValues {
    // was Args
    _args: Vec<String>, // Todo: You never know where you may need it
    planetary_position: PlanetaryPosition,
    start_view: GeoView,
    xr: bool,
}

enum CamControlMode {
    F4,
    Fly,
    // todo: more to come
}

#[bevy_main]
pub fn main() {

    // todo: NOT VISIBLE! WHY?
    error!("Main START ++++++++++++++++++++++++++++++++++++++");
    warn!( "Main START ++++++++++++++++++++++++++++++++++++++");
    info!( "Main START ++++++++++++++++++++++++++++++++++++++");
    // THIS WORKS:
    // panic!("The last ouptut of the app");
    // assert_eq!( up_view, -30. );
  
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

    let mut cam_control_mode = CamControlMode::F4;

    let mut geo_coord = GeoCoord {
        lat: 48.1408, // Germany, Munic, Main railway station
        lon: 11.5577,
    };
    let mut elevation: f32 = 300.0;

    // GeoView to city center, Marienplatz
    let mut direction: f32 = -105.0; // Compass view-direction to Oeast-Southeast. 0 = Nord, -90 = East Todo: Why minus?
    let mut up_view: f32 = -30.0; // Up-view slightly down. -90 = down, 0 = horizontal 90 = Up
    let mut distance: f32 = 300.; // radius of the sphere, the arc rotate camera rotates on
    let mut camera_fov: f32 = 30.; // todo: default?  field of view, the angle widht of the world, the camera is showing

    let mut xr = false;

    for arg in &args {
        let (k, v) = arg.split_once('=').expect("arguments must be `key=value` pairs");
            match k {
            "con" => {
                let arg: String = v.parse().unwrap();
                let arg: &str = arg.as_str(); // todo: better rust?
                match arg {
                    "fly" => cam_control_mode = CamControlMode::Fly,
                    "ufo" => cam_control_mode = CamControlMode::Fly,
                    _ => (), // F4 is default
                }
            },
            "lat" => geo_coord.lat = v.parse().unwrap(),
            "lon" => geo_coord.lon = v.parse().unwrap(),
            "ele" => elevation = v.parse().unwrap(),
            "view" => up_view = v.parse().unwrap(),
            "dir" => direction = v.parse().unwrap(),
            "dist" => distance = v.parse().unwrap(),
            "fov" => camera_fov = v.parse().unwrap(),

            "xr" => xr = v.parse().unwrap(),
            other => panic!("unknown key `{other}`"),
        }
    }

    let start_view = GeoView {
        geo_coord,
        elevation,
        direction,
        up_view,
        distance,
        camera_fov,
    };

    let _start_view = GeoView {
        // test only
        geo_coord, //: GeoCoord { lat: 33., lon: 0. }, // up,dir
        elevation: 1000.,
        direction: 0.,
        up_view: 0.,
        distance: 1000.,
        camera_fov: 44.,
    };

    let mut app = App::new();
    app.insert_resource(StartingValues {
        _args: args,
        planetary_position: geo_coord.to_cartesian(),
        start_view,
        xr,
    });

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
        {
            app.add_plugins(xr::Plugin);
            app.add_systems(Update, pull_to_ground);
        }
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
        .add_plugins(space::Plugin);

    match cam_control_mode {
        CamControlMode::F4 => {
            app.add_plugins(f4control::Plugin);
        }
        CamControlMode::Fly => {
            app.add_plugins(flycam::Plugin)
                .add_systems(Update, update_camera_orientations)
                .add_systems(PostUpdate, reposition_compass);
        }
    }

    app.add_plugins(geoview::Plugin)
        .insert_resource(TileMap::default())
        .add_systems(Startup, setup)
        .add_plugins(tilemap::Plugin)
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

#[derive(Resource, Copy, Clone)]
pub struct ViewDistance(f32);

// Compass   Todo: move out of lib.rs
#[derive(Component)]
struct Compass;

mod player;

fn reposition_compass(
    mut compass: Query<
        GalacticTransform,
        (
            With<Compass>,
            Without<bevy_flycam::FlyCam>,
            Without<OpenXRTrackingRoot>,
        ),
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
