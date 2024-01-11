use super::f4control::MovementValues;
use super::geocoord::*;
use crate::player::Player;
use bevy::{
    prelude::*,
    utils::tracing::{self, instrument},
};
use big_space::FloatingOriginSettings;
use std::{collections::HashMap, f32::consts::FRAC_PI_2};

#[derive(Resource)]
pub struct Views {
    map: HashMap<String, String>,
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
#[derive(Default, Debug, Clone, Copy)]
pub struct GeoView {
    pub geo_coord: GeoCoord, // lat/lon
    pub elevation: f32,
    pub direction: f32,
    pub up_view: f32,
    pub distance: f32, // todo: use distance and camera_fov (by ArcControl)
    pub camera_fov: f32,
}

impl GeoView {
    pub fn new() -> GeoView {
        GeoView {
            geo_coord: GeoCoord { lat: 0., lon: 0. },
            elevation: 0.,
            direction: 0.,
            up_view: 0.,
            distance: 0.,
            camera_fov: 0.,
        }
    }
    /**
     * Store self GeoView in a browser cookie
     * To restore it into your viewer, use [[GeoView]].[[restore]]
     * internal, util [[restore]] is called.
     * @param id  "name" of the cookie
     */
    pub fn store(&self, id: String, views: &mut HashMap<String, String>) {
        //                                      id la lo he di vi ra fo
        //t cookie = format!("OSM2World_GeoView_{}={} {} {} {} {} {} {};samesite=strict",  //  todo? {:.2}
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
        println!(">>> id: {} cookie: {}", id, cookie);

        // html/wasm: document.cookie = cookie;
        views.insert(id, cookie);
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

    pub fn set_camera_view(
        &self,
        space: &FloatingOriginSettings,
        player: &mut Player,
        movement_values: Option<&mut MovementValues>,
    ) {
        if let Some(movement_values) = movement_values {
            movement_values.view = *self;
        }

        // Position on Earth ground
        let mut starting_transform = self
            .geo_coord
            .to_cartesian()
            .to_galactic_transform_space(space);
        let directions = starting_transform.directions();

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

        if self.distance > 0.0 {
            //let beam_directon = starting_transform.transform.rotation; // quat
            //let (angle,_) = beam_directon.to_axis_angle();
            starting_transform.transform.translation += directions.west * self.distance;
            //starting_transform.transform.look_at(_camera_spot);
        }

        player.set_pos(starting_transform);
    }

    #[instrument(level = "debug", skip(space, player), ret)]
    pub fn get_camera_view(space: &FloatingOriginSettings, player: &Player) -> Self {
        let position = player.pos();

        let geo_coord = position.to_planetary_position().to_geocoord();
        let elevation =
            position.position_double(space).length() as f32 - crate::geocoord::EARTH_RADIUS;

        let forward = position.galactic_transform.transform.forward();
        let directions = position.directions();
        let up_view = (forward.angle_between(-directions.up) - FRAC_PI_2).to_degrees();

        // we have to "rotate back the up" before calculating delta north
        let flat_forward = directions
            .up // rotate back up ?    https://en.wikipedia.org/wiki/Cross_product   cross product or vector product
            .cross(position.galactic_transform.transform.right()); // now we have a vector pointing forward, but parallel to the ground.

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
            distance: 66.,
            camera_fov: 77.,
        }
    }
}

// System: If keys pressed, store and restore camera views
fn keys_ui(
    keys: Res<Input<KeyCode>>,
    mut player: Player,
    mut movement_values: ResMut<MovementValues>,
    mut views: ResMut<Views>,
    space: Res<FloatingOriginSettings>,
) {
    {
        for key in keys.get_just_pressed() {
            let key = *key;

            match key {
                // (>= KeyCode::Key0 & <=KeyCode::Key9) => {
                KeyCode::Key0
                | KeyCode::Key1
                | KeyCode::Key2
                | KeyCode::Key3
                | KeyCode::Key4
                | KeyCode::Key5
                | KeyCode::Key6
                | KeyCode::Key7
                | KeyCode::Key8
                | KeyCode::Key9 => {
                    let key_string = format!("{:?}", key).to_string();
                    if keys.pressed(KeyCode::ShiftRight) {
                        info!("*** KEY: {:?}", key_string);
                        if key != KeyCode::Key0 {
                            let geo_view = GeoView::get_camera_view(&space, &player);
                            geo_view.store(key_string, &mut views.map);
                        }
                    } else {
                        info!("*** key: {:?}", key_string);
                        let view3 = GeoView::restore(key_string, &mut views.map);
                        if let Some(view3) = view3 {
                            view3.set_camera_view(&space, &mut player, Some(&mut movement_values));
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
    mut player: Player,
    mut views: ResMut<Views>,
    space: Res<FloatingOriginSettings>,
) {
    starting_values
        .start_view
        .store("Key0".to_string(), &mut views.map);
    starting_values
        .start_view
        .set_camera_view(&space, &mut player, None);
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        // todo: Is there a OnKeyPressed instead of Update?
        // todo: the reaction is bad? Mayh be this helps: Pairing with bevy_framepace to smooth out input latency
        app.add_systems(Update, keys_ui);
        let map = HashMap::new();
        app.insert_resource(Views { map });
        app.add_systems(PostStartup, keys_ui_setup);
    }
}

// Dodo?: implement old code: pub fn to_camera_view(&self, osm_scene: &OsmScene) -> CameraView {
