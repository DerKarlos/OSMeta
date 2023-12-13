use super::geopos::*;
use crate::GalacticGrid;
use bevy::prelude::*;
use big_space::{FloatingOriginSettings, GridCell};
use std::collections::HashMap;

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
    geo_pos: GeoPos, // lat/lon
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
            self.geo_pos.lat,
            self.geo_pos.lon,
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

            let geo_pos = GeoPos {
                lat: (floats[0]).parse().unwrap(),
                lon: (floats[1]).parse().unwrap(),
            };

            Some(GeoView {
                geo_pos,
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
        space: &Res<'_, FloatingOriginSettings>,
        //movement_settings: &mut ResMut<'_, MovementSettings>,
        transform: &mut Transform,
    ) {
        // Todo: @Oli? This code does not handle big_space yet and does not work, if the player is far form GPU::ZERRO  SEE #64
        //let galactic_position = args.starting_position.to_galactic_position(space).pos().as_vec3();
        //transform.translation = galactic_position + directions.up * (args.height + _test);
        //transform.look_at(galactic_position, Vec3::Z);

        let starting_position = self.geo_pos.to_cartesian();
        //t _pos = Vec3::new(self.geo_pos.lat, self.geo_pos.lon, 0.);
        let direction: Vec3 = starting_position.normalize().as_vec3();

        let starting_position = self.geo_pos.to_cartesian();
        let (_grid, subgrid): (GalacticGrid, _) = space.translation_to_grid(starting_position);

        let rotation = Quat::from_axis_angle(Vec3::Z, self.dir.to_radians())  // Todo: change ::Z to ::Y
            * Quat::from_axis_angle(Vec3::X, self.view.to_radians());

        // First to positon and looking down, head to nord
        //info!("*** grid {:?} h: {:?}", _grid, args.height + _test);
        transform.translation = subgrid + direction * self.height;
        transform.look_at(subgrid, Vec3::Z);
        // Next rotate to up and to west or east
        // bad *transform = *transform * Transform::from_rotation(rotation);
        transform.rotation = transform.rotation * rotation;
    }

    // Todo: This does not work yet. @Oli?
    pub fn get_camera_view(space: &Res<FloatingOriginSettings>, transform: &Transform) -> Self {
        let translation = transform.translation; // GPU-translation = ? Not Earth, Galaqctic?
        info!("translation: {:?}", translation);
        let (grid, in_grid_pos): (GridCell<i64>, Vec3) = space.translation_to_grid(translation);
        info!("grid: {:?} in_grid_pos: {:?}", grid, in_grid_pos);
        let g = GeoPos::from_cartesian(in_grid_pos.as_dvec3());
        info!("g: {:?}", g); // wrong!: lat: 31.906904, lon: 93.580765
        let lat = g.lat;
        let lon = g.lon;
        info!("lat/lon: {:?}/{:?}", lat, lon);
        let height = translation.length() as f32; // - crate::geopos::EARTH_RADIUS; ??? // f32 = 6_378_000.
        info!("height: {:?}", height); // 897.622 ???

        let geo_pos = GeoPos {
            lat: lat as f32,
            lon: lon as f32,
        };
        let view = Self {
            geo_pos,
            height,
            dir: 0.,
            view: 0.,
            radius: 6.,
            fov: 7.,
        };
        view
    }
}

// System: If keys pessed, store and restore camera views
fn keys_ui(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &GalacticGrid), With<bevy_flycam::FlyCam>>,
    mut views: ResMut<Views>,
    args: Res<crate::Args>,
    space: Res<FloatingOriginSettings>,
) {
    let (mut transform, _grid) = query.single_mut();
    {
        for key in keys.get_just_pressed() {
            let key = *key;

            match key {
                KeyCode::Key0 => {
                    info!("*** Key: {:?}", key);
                    // Set camera form Args
                    let geo_pos = GeoPos {
                        lat: 48.1408,
                        lon: 11.5577,
                    };
                    let start_view = GeoView {
                        geo_pos,
                        height: args.height,
                        dir: args.direction,
                        view: args.view,
                        radius: 6.,
                        fov: 7.,
                    };
                    start_view.store("start".to_string(), &mut views.map);
                    start_view.set_camera_view(&space, &mut transform);
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
                        let view = GeoView::get_camera_view(&space, &transform);
                        view.store(key.to_string(), &mut views.map);
                    } else {
                        info!("*** key: {:?}", key);
                        let view3 = GeoView::restore(key.to_string(), &mut views.map);
                        if let Some(view3) = view3 {
                            info!("*** out: {:?}", view3);
                            view3.set_camera_view(&space, &mut transform);
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
