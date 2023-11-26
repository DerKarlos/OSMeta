use bevy::prelude::*;
use std::f32::consts::PI;

/**
 * Geo-position on the (OSM-) world map (GPS position)
 *
 * Does also calculations: tile_Name, the position is located in
 * and distance in meters from the tiles corner
 */

#[derive(Default, Debug, Clone, Copy)]
pub struct GeoPos {
    pub lat: f32,
    pub lon: f32,
}

impl GeoPos {
    /**
     * Convert GPS coordinates to tile coordinates.
     * We use the OSM naming for tiles:
     * https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames.
     * x and y relate to a lonitude and latitude position on Earth.
     * In OSM he two values are only used as part of the filename of a tile.
     * even if it is an x/y coordinate in numbers.
     * @param zoom  Zoom level of the OSM tile-name(x/y) system
     * @return coordinate in tile coordinates
     */
    pub fn to_tile_coordinates(&self, zoom: u8) -> Vec2 {
        let pow_zoom = 2_u32.pow(zoom.into()) as f32;

        // return
        Vec2 {
            // Longitude, (Längengrad) West/East "index"
            x: ((self.lon + 180.) / 360. * pow_zoom),

            // y: Latitude, (Breitengrad) Nort/South "index"
            y: ((1.
                - ((self.lat * PI / 180.).tan() + 1. / (self.lat * PI / 180.).cos()).ln() / PI)
                / 2.
                * pow_zoom),
            // The Nort/South y tile name part is not linear, the tiles gets stretched to the poles
            // to compensate the stretching if the stretching of the West/East projection
        }
    }
}
