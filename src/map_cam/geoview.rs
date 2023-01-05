// geoview.ts

// mod super::fly_control;
// use bevy::prelude::*;

use super::osmscene::*;
use super::geopos::*;
use super::cameraview::*;
use super::utils::{
    DEFAULT_DIR,
    DEFAULT_VIEW,
    DEFAULT_RADIUS,
    DEFAULT_HEIGHT,
    DEFAULT_FOV,
};

use std::collections::HashMap;  // or??? use bevy::utils::HashMap;

/**
 * Geo position on Earth and rotation at/abowe a [[OsmScene]]
 *
 * An instance of self [[GeoView]] serves
 * to define a geo Position and a camera position and view angles,
 *
 * A O2W-lib user, by the lib API
 * may create an instance to define a (or find an existing) [[OsmScene]]
 * or get it from a [[Viewer]].[[getGeoViewAtCamera]] to store multible [[GeoView]]s
 *
 * The [[OsmScene]] uses it internal to read and set the browser url.
 *
 * TODO?:
 * The geo position will be checked if it is in range of an existing OsmScenes.
 * Or a new OsmScene will be created.
 */
#[derive(Default, Debug,Clone,Copy)]
pub struct GeoView {
    pub geo_pos: GeoPos,
    pub height: f32,
    pub dir: f32,
    pub view: f32,
    pub radius: f32,
    pub fov: f32,
    }

impl GeoView {

    /**
     * View constructor
     * @param geoPos  (lat,lon) requested position on Earth (lat,lon)
     * @param height  (meter)   height in the [[OsmScene]], default is 1.6
     * @param dir     (degrees) the longitudinal, compas-direction, -90/0/90 equals west/nord=default/east. Becomes the alpha of the camera
     * @param view    (degrees) the latitudinal, up/down-direction, 0=horizontal. Default is slightly down. Becomes the beta of the camera
     * @param radius  (meter)   the camera distance from the target position
     * @param fov     (degrees) the camera view angle / zoom
     */

     
    pub fn default(geo_pos: GeoPos) -> GeoView {
        GeoView{
            geo_pos, //GeoPos::new(),
            height: DEFAULT_HEIGHT,
            dir:    DEFAULT_DIR,
            view:   DEFAULT_VIEW,
            radius: DEFAULT_RADIUS,
            fov:    DEFAULT_FOV,
        }
    }

    /**
     * restore this geo pos from browser cookie
     * @param id  "name" of the cookie to restore it
     * @return restored geo view
     */
    pub fn restore(id: String, cookies: &mut HashMap<String,String>) -> Option<GeoView> {

        // todo: far jump calcualtes wrong pbf-tile ? (wind=>passau)
        // https://192.168.3.141:8080/o2w/tiles/13/4385/2827.o2w.pbf = wind
        // https://192.168.3.141:8080/o2w/tiles/13/4385/2829.o2w.pbf = BAD 9!
        // https://192.168.3.141:8080/o2w/tiles/13/4402/2828.o2w.pbf = default passau

        // if (document.cookie.indexOf('OSM2World_GeoView_' + id) == -1) return undefined;   // cookie does not exists
        //t cookie = self.getCookie("OSM2World_GeoView_".to_string() + &id);
        let cookie = cookies.get( &id ).unwrap();//_or(&or);

        println!("<<< id: {} cookie: {}", id, cookie);

        let floats: Vec<&str> = cookie.split(' ').collect();

        let geo_pos = GeoPos{
            lat: (floats[0]).parse().unwrap(),
            lon: (floats[1]).parse().unwrap(),
        };

        Some(GeoView{
            geo_pos,
            height:  (floats[2]).parse().unwrap(),
            dir:     (floats[3]).parse().unwrap(),
            view:    (floats[4]).parse().unwrap(),
            radius:  (floats[5]).parse().unwrap(),
            fov:     (floats[6]).parse().unwrap(),
        })
    }


    /**
     * (lib-internal) Convert self [[GeoView]] instance to a new [[CameraView]] instance
     * @param osmScene  scene, needed to calculate meters from lat/lon
     * @return a new camera view
     */
    pub fn to_camera_view(&self, osm_scene: &OsmScene) -> CameraView {

        let mut scene_pos = self.geo_pos.calc_scene_pos(osm_scene); //.subtract(osmScene.loadCenter);
        scene_pos.y = self.height;

        //return
        CameraView{
            scene_pos,
            dir:   (self.dir ).to_radians(), // - 90.), // API dir  0 degrees = nord        becomes BJS alpha -90 rad = nord
            view:  (self.view).to_radians(), // + 90.), // API view 0 degrees = horizontal  becomes BJS beta  +90 rad = horizontal
            radius: self.radius,
            fov:   (self.fov.to_radians())
        }
    }


    /**
     * Store self geo view in a browser cookie
     * To restore it into your viewer, use [[GeoView]].[[restore]]
     * internal, util [[restore]] is called.
     * @param id  "name" of the cookie
     */

    pub fn store(&self, id: String, cookies: &mut HashMap<String,String>) {

        //                                      id la lo he di vi ra fo
        //t cookie = format!("OSM2World_GeoView_{}={} {} {} {} {} {} {};samesite=strict",  // {:.2}
        let cookie = format!(                  "{} {} {} {} {} {} {}",
        //  id,
            self.geo_pos.lat,
            self.geo_pos.lon,
            self.height,
            self.dir, // alpha, compas
            self.view, // beta, headupdown
            self.radius,
            self.fov,
        );
        println!(">>> id: {} cookie: {}", id, cookie);

        // html/wasm: document.cookie = cookie;
        cookies.insert(id, cookie );

    }

    /*  Cookie ... will be soon ...:
        Click the hamburger menu in the top right, then, Options->Privacy & Security
        From here, scroll down about half-way and find Cookies and Site Data. Click Manage Data.
        Then, search for the site you are having the notices on, highlight it, and Remove Selected  */

}
