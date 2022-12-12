// geoview.ts

use super::osmscene::*;
use super::geopos::*;
use super::cameraview::*;
use super::utils::{
    DEFAULT_DIR,
    DEFAULT_VIEW,
    DEFAULT_RADIUS,
    DEFAULT_HEIGHT,
    DEFAULT_FOV,
    rad
};

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
#[derive(Debug,Clone,Copy)]
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

     
    pub fn new(geo_pos: GeoPos) -> GeoView {
        GeoView{
            geo_pos, //GeoPos::new(),
            height: DEFAULT_HEIGHT,
            dir: DEFAULT_DIR,
            view: DEFAULT_VIEW,
            radius: DEFAULT_RADIUS,
            fov: DEFAULT_FOV,
        }
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
            alpha: (self.dir  - 90.).to_radians(), //  API dir  0 degrees = nord        becomes BJS alpha -90 rad = nord
            beta:  (self.view + 90.).to_radians(), // API view 0 degrees = horizontal  becomes BJS beta  +90 rad = horizontal
            radius: self.radius,
            fov: (self.fov).to_radians()
        }
    }


    /**
     * Store self geo view in a browser cookie
     * To restore it into your viewer, use [[Viewer]].[[restoreGeoView]]
     * internal, util [[restoreGeoView]] is called.
     * @param id  "name" of the cookie
     */

    fn store_cookie(&self, id: String) {

        //                                      id la lo he di vi ra fo
        let cookie = format!("OSM2World_GeoView_{}={} {} {} {} {} {} {};samesite=strict",  // {:.2}
            id,
            self.geo_pos.lat,
            self.geo_pos.lon,
            self.height,
            self.dir, // alpha
            self.view, // beta
            self.radius,
            self.fov,
        );
        // html/wasm: document.cookie = cookie;
        println!(">>> geo {}", cookie);
    }

    /*  Cookie ... will be soon ...:
        Click the hamburger menu in the top right, then, Options->Privacy & Security
        From here, scroll down about half-way and find Cookies and Site Data. Click Manage Data.
        Then, search for the site you are having the notices on, highlight it, and Remove Selected  */

}
