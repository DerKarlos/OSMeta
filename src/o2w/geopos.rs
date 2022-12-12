// geopos.ts

use super::utils::{ScenePos, DEFAULT_LAT, DEFAULT_LON, PI, LAT_FAKT};
use super::osmscene::*;

// import { OsmScene } from "./osmscene.js"


/**
 * Geo-position on the (OSM-) world map (GPS position)
 *
 * Does also calculations: tile_Name, the position is located in
 * and distance in meters from the tiles corner
 */

#[derive(Debug,Clone,Copy)]
 pub struct GeoPos {
    pub lat: f32,
    pub lon: f32,
}

impl GeoPos {

    /**
     * Creats a new positon or the default position.
     * @param lat Latitude, (Breitengrad) Nort/South position
     * @param lon Longitude, (Längengrad) West/East position
     */
    pub fn new() -> GeoPos {
        GeoPos{
            lat: DEFAULT_LAT,
            lon: DEFAULT_LON,
        }
    }


    /**
     * Calculate the tile-name(x/y) of a tile at this position
     * @param zoom  Zoom level of the OSM tile-name(x/y) system
     * @return tile-name(x/y)
     */
    fn calc_tile_name(&self, zoom: u8) -> glam::Vec2 {
        let zoom = zoom as f32;

        // return
        glam::Vec2{

            // Longitude, (Längengrad) West/East "index"
            x: ((self.lon + 180.) / 360. * zoom.powf(2.)).floor(),

            // y: Latitude, (Breitengrad) Nort/South "index"
            y: (
                (
                    1. - (
                        (self.lat * PI / 180.).tan() + 1. / (self.lat * PI / 180.).cos()
                    ).ln() / PI
                ) / 2. * zoom.powf(2.)
            ).floor(),
            // The Nort/South y tile name part is not linear, the tiles gets stretched to the poles
            // to compensate the stretching if the stretching of the West/East projection

        }
    }


    /**
     * Calculate the meter distances of this geo position
     * relative to the position of the center of the [[OsmScene]].
     *
     * Because the null center of the [[OsmScene]] is the center of the first loaded pbf-tile,
     * this function is used calculate the initial positon of the camera etc.
     *
     * @param osmScene  The OsmScene-instance, the geo-position will be used in
     * @return x/0/z position in meter inside the Scene (ScenePos/Vector3)
     */
    pub fn calc_scene_pos(&self, osm_scene: &OsmScene) -> ScenePos {
        let pos_relative_to_corner = self.calc_meters_to_other_geo_pos(osm_scene.null_corner_geo_pos);
        let mut scene_pos = pos_relative_to_corner + osm_scene.pbf_corner_to_center; // center to corner ???
        scene_pos.x = (scene_pos.x * 100.).floor() / 100.; // cm is acurate in this case
        scene_pos.z = (scene_pos.z * 100.).floor() / 100.;
        /*return*/ scene_pos // gps-degrees plus/nord = z-meter plus/behind "In the backround = north"
    }


    /**
     * Calculate the meter distances of this geo position
     * relative to the other geo position
     *
     * Used in [[OsmScene]] to calculate the pbf-tile size and in calcScenePos
     *
     * @param other The other geo position
     * @return x/0/z position delta in meter inside the Scene (ScenePos/Vector3)
     */
    fn calc_meters_to_other_geo_pos(&self, other: GeoPos) -> ScenePos {
        // the closer to the pole, the smaller the tiles size in meters get
        let lon_fakt = LAT_FAKT * ((self.lat / 180. * PI).abs()).cos(); // Longitude(Längengrad) West/East factor
        // actual geo pos - other geo pos = relative geo/meter scene pos
        let x = (self.lon - other.lon) * lon_fakt;
        let z = (self.lat - other.lat) * LAT_FAKT;
        /*return*/ ScenePos{x, y:0., z}
    }

}
