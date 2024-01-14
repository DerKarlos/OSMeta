//! Loads and renders a glTF file as a scene.

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
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
use flycontrol::update_camera_orientations;
use geocoord::GeoCoord;
use geoview::GeoView;
use http_assets::HttpAssetReaderPlugin;
use player::{ControlValues, PlanetaryPosition};
use tilemap::TileMap;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use xr::pull_to_ground;

mod compass;
mod f4control;
mod flycontrol;
mod geocoord;
mod geoview;
mod http_assets;
mod player;
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

pub enum CamControlMode {
    F4,
    Fly,
    // todo: more to come
}

#[derive(Resource)]
struct StartingValues {
    // was Args
    _args: Vec<String>, // Todo: You never know where you may need it
    planetary_position: PlanetaryPosition,
    start_view: GeoView,
    _cam_control_mode: CamControlMode,
    xr: bool,
}

#[bevy_main]
pub fn main() {
    // todo: info! warn! error! NOT VISIBLE! WHY?
    // FOR TEST, USE: panic!("The last ouptut of the app");  OR assert_eq!( up_view, -30. );

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

    let mut _cam_control_mode = CamControlMode::F4; // default: F4,  test: Fly

    let mut geo_coord = GeoCoord {
        lat: 48.1408, // Germany, Munic, Main railway station
        lon: 11.5577,
    };
    let mut elevation: f32 = 50.0; // todo: default 1.4 for f4control

    // GeoView to city center, Marienplatz
    let mut direction: f32 = -105.0; // Compass view-direction to Oeast-Southeast. 0 = Nord, -90 = East Todo: Why minus?
    let mut up_view: f32 = -30.0; // Up-view slightly down. -90 = down, 0 = horizontal 90 = Up
    let mut distance: f32 = 500.; // radius of the sphere, the arc rotate camera rotates on
    let mut camera_fov: f32 = 30.; // todo: default?  field of view, the angle widht of the world, the camera is showing

    let mut xr = false;

    for arg in &args {
        if arg.is_empty() {continue;}; // to skipp unneeded & in the browser URL
        let (k, v) = arg
            .split_once('=')
            .expect("arguments must be `key=value` pairs");
        match k {
            "con" => {
                let arg: String = v.parse().unwrap();
                let arg: &str = arg.as_str(); // todo: better rust?
                match arg {
                    "fly" => _cam_control_mode = CamControlMode::Fly,
                    "ufo" => _cam_control_mode = CamControlMode::Fly,
                    _ => _cam_control_mode = CamControlMode::F4,
                }
            }
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
        .add_plugins(space::Plugin)
        .add_plugins(player::Plugin)
        //.add_systems(Update, init_controls);
        .init_resource::<ControlValues>();

    match _cam_control_mode {
        CamControlMode::F4 => {
            app.add_plugins(f4control::Plugin);
        }
        CamControlMode::Fly => {
            app.add_plugins(flycontrol::Plugin)
                .add_systems(Update, update_camera_orientations)
                .add_systems(PostUpdate, compass::reposition_compass);
        }
    }

    app.insert_resource(StartingValues {
        _args: args,
        planetary_position: geo_coord.to_cartesian(),
        start_view,
        xr,
        _cam_control_mode,
    });

    app.add_plugins(geoview::Plugin)
        .insert_resource(TileMap::default())
        .add_systems(Startup, setup)
        .add_plugins(tilemap::Plugin)
        .run();
}

fn setup(mut diags: ResMut<ScreenDiagnostics>) {
    diags.modify("fps").aggregate(Aggregate::Average);
}

#[derive(Resource, Copy, Clone)]
pub struct ViewDistance(f32);
