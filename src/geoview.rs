use super::geocoord::*;
use super::GalacticTransformOwned;
use crate::player::{
    CamControlMode, ControlValues, PlayerGalacticTransform, PlayerQuery, OSM_LAT_LIMIT,
};
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};
use std::{collections::HashMap, f32::consts::FRAC_PI_2};

#[derive(Resource)]
pub struct Views {
    pub map: HashMap<String, String>,
}

/**
 * Geo coordinates on Earth and rotation at/abowe a GPU scene
 *
 * An instance of self [[GeoView]] serves
 * to define geo coordinates as a camera-position and view-angles,
 *
 * A crate user, by the lib API
 * may create an instance to define a (or find an existing) GPU scene
 * or get it from a "getGeoViewAtCamera" (todo) to store multible [[GeoView]]s
 *
 * The GPU scene uses it internal to read and set the browser url.
 */
#[derive(Debug, Clone, Copy)]
pub struct GeoView {
    pub geo_coord: GeoCoord, // lat/lon
    pub elevation: f32,
    pub direction: f32,
    pub up_view: f32,
    pub distance: f32, // todo: use distance and camera_fov (by ArcControl)
    pub camera_fov: f32,
}

impl Default for GeoView {
    fn default() -> Self {
        Self {
            geo_coord: GeoCoord { lat: 0., lon: 0. }, // todo: London?
            elevation: 1.4,
            direction: 0.0,
            up_view: 0.0,
            distance: 500.0,
            camera_fov: 42.0,
        }
    }
}

impl GeoView {
    pub fn limit(&mut self) {
        const ELEVATION_LIMIT: f32 = 20_000_000_000.0; // meter
        self.geo_coord.lat = self.geo_coord.lat.clamp(-OSM_LAT_LIMIT, OSM_LAT_LIMIT);
        self.up_view = self.up_view.clamp(-OSM_LAT_LIMIT, OSM_LAT_LIMIT);
        self.elevation = self.elevation.clamp(0.4, ELEVATION_LIMIT);
        self.distance = self.distance.clamp(0.4, ELEVATION_LIMIT);
        self.direction %= 360.0;
    }

    /**
     * Store self GeoView in a browser cookie
     * To restore it into your viewer, use [[GeoView]].[[restore]]
     * internal, util [[restore]] is called.
     * @param id  "name" of the cookie
     */
    pub fn store(&self, id: KeyCode, views_map: &mut HashMap<String, String>) {
        // todo: Add a name for the view
        //                                      id la lo he di vi ra fo
        //t cookie = format!("OSM2World_GeoView_{}={} {} {} {} {} {} {};samesite=strict",  //  todo? {:.2}
        let id_string = format!("{:?}", id).to_string();
        let cookie = format!(
            "{} {} {} {} {} {} {}",
            self.geo_coord.lat,
            self.geo_coord.lon,
            self.elevation,
            self.direction, // alpha, compas
            self.up_view,   // beta, headupdown
            self.distance,
            self.camera_fov,
        );
        println!(">>> id: {} cookie: {}", id_string, cookie);

        // html/wasm: document.cookie = cookie;
        views_map.insert(id_string, cookie);
    }

    /**
     * restore this geo pos from browser cookie
     * @param id  "name" of the cookie to restore it
     * @return restored GeoView
     */
    pub fn restore(id: String, views: &mut HashMap<String, String>) -> Option<GeoView> {
        let cookie = views.get(&id); //.unwrap();//_or(&or);

        if let Some(cookie) = cookie {
            println!("<<< id: {} cookie: {}", id, cookie);

            let floats: Vec<&str> = cookie.split(' ').collect();

            let geo_coord = GeoCoord {
                lat: (floats[0]).parse().unwrap(),
                lon: (floats[1]).parse().unwrap(),
            };

            Some(GeoView {
                geo_coord,
                elevation: (floats[2]).parse().unwrap(),
                direction: (floats[3]).parse().unwrap(),
                up_view: (floats[4]).parse().unwrap(),
                distance: (floats[5]).parse().unwrap(),
                camera_fov: (floats[6]).parse().unwrap(),
            })
        } else {
            None
        }
    }

    pub fn to_galactic_transform(self, use_distance: bool) -> GalacticTransformOwned {
        // Position on Earth ground
        let p_starting_transform: PlayerGalacticTransform =
            self.geo_coord.to_cartesian().to_galactic_transform_space();

        let directions = p_starting_transform.directions();

        let mut starting_transform: GalacticTransformOwned =
            p_starting_transform.galactic_transform;

        //  let directions = starting_transform.directions();

        // Add camera / player height above ground
        starting_transform.transform.translation += directions.up * self.elevation;
        let _camera_spot = starting_transform.transform.translation;
        // Look northwards (to Earth center)
        starting_transform
            .transform
            .look_to(directions.north, directions.up);

        // Rotate to west or east
        starting_transform
            .transform
            .rotate_axis(directions.up, self.direction.to_radians());
        // Pan up or down. We subtract 90° (FRAC_PI_2), because the up-view is an angle from looking
        // straight down. We don't default to looking down, as that doesn't guarantee us
        // that the forward direction is north.

        starting_transform
            .transform
            .rotate_local_x(self.up_view.to_radians());

        if use_distance {
            //let beam_directon = starting_transform.transform.rotation; // quat
            //let (angle,_) = beam_directon.to_axis_angle();
            let beam_directon = -starting_transform.transform.forward();
            starting_transform.transform.translation += beam_directon * self.distance;
            //starting_transform.transform.look_at(_camera_spot);
        }

        starting_transform
    }

    pub fn set_camera_view(&self, player: &mut PlayerQuery, control_values: &mut ControlValues) {
        control_values.view = *self; //
        let use_distance = control_values.cam_control_mode == CamControlMode::F4;
        let galactic_transform = self.to_galactic_transform(use_distance);

        player.set_pos(galactic_transform);
    }

    #[instrument(level = "debug", skip(player), ret)]
    pub fn from_player(player: &PlayerQuery) -> Self {
        let position = player.pos();

        let geo_coord = position.to_planetary_position().to_geocoord();
        let elevation = position.position_double().length() as f32 - crate::geocoord::EARTH_RADIUS;

        let forward = position.galactic_transform.transform.forward();
        let directions = position.directions();
        let up_view = (forward.angle_between(-directions.up) - FRAC_PI_2).to_degrees();

        // we have to "rotate back the up" before calculating delta north
        let flat_forward = directions
            .up // rotate back up ?    https://en.wikipedia.org/wiki/Cross_product   cross product or vector product
            .cross(*position.galactic_transform.transform.right()); // now we have a vector pointing forward, but parallel to the ground.

        // Cannot use `angle_between` naively, as that gives us a positive angle between 0 and 180 degrees
        let north_angle = flat_forward.angle_between(directions.north).to_degrees();
        let west_angle = flat_forward.angle_between(directions.west).to_degrees();
        // So we pick a positive or negative angle depending on how far away from west we are.
        let direction = if west_angle < FRAC_PI_2 {
            north_angle
        } else {
            -north_angle
        };

        Self {
            geo_coord,
            elevation,
            direction,
            up_view,
            distance: 0.0,
            camera_fov: 77.,
        }
    }
}

// System: If keys pressed, store and restore camera views
fn keys_ui(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: PlayerQuery,
    mut control_values: ResMut<ControlValues>,
    mut views: ResMut<Views>,
) {
    {
        for key in keys.get_just_pressed() {
            let key = *key;

            match key {
                // (>= KeyCode::Key0 & <=KeyCode::Key9) => {
                KeyCode::Digit0
                | KeyCode::Digit1
                | KeyCode::Digit2 // todo: shift-Key2 does not work. Not in keys 
                | KeyCode::Digit3
                | KeyCode::Digit4
                | KeyCode::Digit5
                | KeyCode::Digit6
                | KeyCode::Digit7
                | KeyCode::Digit8
                | KeyCode::Digit9 => {
                    let key_string = format!("{:?}", key).to_string();
                    if keys.pressed(KeyCode::ShiftRight) {
                        info!("*** KEY: {:?}", key_string);
                        if key != KeyCode::Digit0 {

                            let is_orbit_control = control_values.cam_control_mode == CamControlMode::F4;
                            let mut geo_view = if is_orbit_control {
                                control_values.view
                            } else {
                                GeoView::from_player(&player)
                            };
                            geo_view.distance = control_values.view.distance; // keep distance of orbid control
                            geo_view.store(key, &mut views.map);
                        }
                    } else {
                        info!("*** key: {:?}", key_string);
                        let view3 = GeoView::restore(key_string, &mut views.map);
                        if let Some(view3) = view3 {
                            view3.set_camera_view(&mut player, &mut control_values);
                        }
                    }
                }
                _ => (),
            };
        }
    }
}

fn keys_ui_setup(
    starting_values: Res<crate::StartingValues>,
    mut player: PlayerQuery,
    mut views: ResMut<Views>,
    mut control_values: ResMut<ControlValues>,
) {
    // The start view is placed for Key0 and
    // all controls are set here (also the camera)
    starting_values.view.store(KeyCode::Digit0, &mut views.map);
    starting_values
        .view
        .set_camera_view(&mut player, &mut control_values);
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        // todo: Is there a OnKeyPressed instead of Update?
        // todo: the reaction is bad? Mayh be this helps: Pairing with bevy_framepace to smooth out input latency
        app.add_systems(Update, keys_ui);
        let mut map = HashMap::new();

        // Test key 9: About the initial View at Munic
        GeoView {
            geo_coord: GeoCoord {
                lat: 48.1408,
                lon: 11.5577,
            },
            up_view: -30.0,
            elevation: 1.4,
            distance: 500.,
            ..Default::default()
        }
        .store(KeyCode::Digit9, &mut map);

        // Test key 8: View below the clouds at Munic
        GeoView {
            geo_coord: GeoCoord {
                lat: 48.1408,
                lon: 11.5577,
            },
            up_view: -OSM_LAT_LIMIT,
            elevation: 1.4,
            distance: 4000.0,
            ..Default::default()
        }
        .store(KeyCode::Digit8, &mut map);

        app.insert_resource(Views { map });
        app.add_systems(PostStartup, keys_ui_setup);
    }
}

// Dodo?: implement old code: pub fn to_camera_view(&self, osm_scene: &OsmScene) -> CameraView {
