use super::geocoord::*;
use crate::player::Player;
//use bevy::prelude::*;
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
 * Geo position on Earth and rotation at/abowe a GPU scene
 *
 * An instance of self [[GeoView]] serves
 * to define a geo Position and a camera position and view angles,
 *
 * A crate user, by the lib API
 * may create an instance to define a (or find an existing) GPU scene
 * or get it from a "getGeoViewAtCamera" (todo) to store multible [[GeoView]]s
 *
 * The GPU scene uses it internal to read and set the browser url.
 */
#[derive(Default, Debug, Clone, Copy)]
pub struct GeoView {
    geo_coord: GeoCoord, // lat/lon
    height: f32,
    dir: f32,
    view: f32,
    radius: f32, // todo: use fov and radius(by ArcControl)
    fov: f32,
}

impl GeoView {
    /**
     * Store self geo view in a browser cookie
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
            self.height,
            self.dir,  // alpha, compas
            self.view, // beta, headupdown
            self.radius,
            self.fov,
        );
        println!(">>> id: {} cookie: {}", id, cookie);

        // html/wasm: document.cookie = cookie;
        views.insert(id, cookie);
    }

    /**
     * restore this geo pos from browser cookie
     * @param id  "name" of the cookie to restore it
     * @return restored geo view
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
                height: (floats[2]).parse().unwrap(),
                dir: (floats[3]).parse().unwrap(),
                view: (floats[4]).parse().unwrap(),
                radius: (floats[5]).parse().unwrap(),
                fov: (floats[6]).parse().unwrap(),
            })
        } else {
            None
        }
    }

    pub fn set_camera_view(
        &self,
        space: &FloatingOriginSettings,
        //movement_settings: &mut ResMut<'_, MovementSettings>,
        player: &mut Player,
    ) {
        let mut starting_position = self.geo_coord.to_cartesian().to_galactic_position(space);
        let directions = starting_position.directions();

        starting_position.transform.translation += directions.up * self.height;
        // Look northwards
        starting_position
            .transform
            .look_to(directions.north, directions.up);

        // Rotate to west or east
        starting_position
            .transform
            .rotate_axis(directions.up, self.dir.to_radians());
        // Pan up or down. We subtract 90°, because the view is an angle from looking
        // straight down. We don't default to looking down, as that doesn't guarantee us
        // that the forward direction is north.
        starting_position
            .transform
            .rotate_local_x(self.view.to_radians() - FRAC_PI_2);
        player.set_pos(starting_position);
    }

    #[instrument(level = "debug", skip(space, player), ret)]
    pub fn get_camera_view(space: &FloatingOriginSettings, player: &Player) -> Self {
        let position = player.pos();

        let geo_coord = position.to_planetary_position().to_geocoord();
        //info!(?geo_coord);
        let height =
            position.position_double(space).length() as f32 - crate::geocoord::EARTH_RADIUS;
        //info!(?height);

        /*
        let mut transform = player.pos().transform;

        // Un-Pan up or down.
        transform.rotate_local_x(geo_coord.lat.to_radians() + FRAC_PI_2);
        // Un-Rotate to west or east
        transform.rotate_local_z(geo_coord.lon.to_radians());
        let view = transform.rotation.x.to_degrees(); // lat
        let dir =  transform.rotation.z.to_degrees(); // llon
        */

        let forward = position.pos.transform.forward();
        let directions = position.directions();
        let view = forward.angle_between(-directions.up).to_degrees();
        let dir = forward
            .cross(directions.up)
            .cross(position.pos.transform.right())
            .angle_between(directions.north)
            .to_degrees();

        Self {
            geo_coord,
            height,
            dir,
            view,
            radius: 6.,
            fov: 7.,
        }
    }
}

// System: If keys pressed, store and restore camera views
fn keys_ui(
    keys: Res<Input<KeyCode>>,
    mut player: Player,
    mut views: ResMut<Views>,
    args: Res<crate::Args>,
    space: Res<FloatingOriginSettings>,
) {
    {
        for key in keys.get_just_pressed() {
            let key = *key;

            match key {
                KeyCode::Key0 => {
                    info!("*** Key: {:?}", key);
                    // Set camera form Args
                    let geo_coord = GeoCoord {
                        lat: 48.1408,
                        lon: 11.5577,
                    };
                    let start_view = GeoView {
                        geo_coord,
                        height: args.height,
                        dir: args.direction,
                        view: args.view,
                        radius: 6.,
                        fov: 7.,
                    };
                    start_view.store("start".to_string(), &mut views.map);
                    start_view.set_camera_view(&space, &mut player);
                    // todo: set "start" while setup/build by args. And read "start" here
                }

                // (>= KeyCode::Key1 & <=KeyCode::Key9) => {
                KeyCode::Key1
                | KeyCode::Key2
                | KeyCode::Key3
                | KeyCode::Key4
                | KeyCode::Key5
                | KeyCode::Key6
                | KeyCode::Key7
                | KeyCode::Key8
                | KeyCode::Key9 => {
                    let key = format!("{:?}", key);
                    if keys.pressed(KeyCode::ShiftRight) {
                        info!("*** KEY: {:?}", key);
                        let view = GeoView::get_camera_view(&space, &player);
                        view.store(key.to_string(), &mut views.map);
                    } else {
                        info!("*** key: {:?}", key);
                        let view3 = GeoView::restore(key.to_string(), &mut views.map);
                        if let Some(view3) = view3 {
                            info!("*** out: {:?}", view3);
                            view3.set_camera_view(&space, &mut player);
                        }
                    }
                }
                _ => (),
            };
        }
    }
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        // todo: Is there a OnKeyPressed instead of Update?
        // todo: the reaction is bad? Mayh be this helps: Pairing with bevy_framepace to smooth out input latency
        app.add_systems(Update, keys_ui);
        let map = HashMap::new();
        app.insert_resource(Views { map });
    }
}

// Dodo?: implement old code: pub fn to_camera_view(&self, osm_scene: &OsmScene) -> CameraView {
