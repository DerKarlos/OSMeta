use std::f32::consts::PI;

use crate::cam_map::TileName;

// import { OsmScene } from "./osmscene.js"


/**
 * Geo-position on the (OSM-) world map (GPS position)
 *
 * Does also calculations: tile_Name, the position is located in
 * and distance in meters from the tiles corner
 */

#[derive(Default, Debug,Clone,Copy)]
 pub struct GeoPos {
    pub lat: f32,
    pub lon: f32,
}

impl GeoPos {
    /**
     * Calculate the tile-name(x/y) of a tile at this position
     * @param zoom  Zoom level of the OSM tile-name(x/y) system
     * @return tile-name(x/y)
     */
    pub fn calc_tile_name(&self, zoom: u32) -> TileName {
        let pow_zoom = 2_u32.pow(zoom) as f32;

        // return
        TileName{

            // Longitude, (Längengrad) West/East "index"
            x: ((self.lon + 180.) / 360. * pow_zoom).floor() as i32,

            // y: Latitude, (Breitengrad) Nort/South "index"
            y: (
                (
                    1. - (
                        (self.lat * PI / 180.).tan() + 1. / (self.lat * PI / 180.).cos()
                    ).ln() / PI
                ) / 2. * pow_zoom
            ).floor() as i32,
            // The Nort/South y tile name part is not linear, the tiles gets stretched to the poles
            // to compensate the stretching if the stretching of the West/East projection

        }
    }
}
