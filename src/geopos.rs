use bevy::prelude::*;
use glam::DVec3;
use globe_rs::{CartesianPoint, GeographicPoint};
use std::f32::consts::PI;

use crate::tilemap::TileCoord;

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
    pub fn to_tile_coordinates(self, zoom: u8) -> TileCoord {
        let pow_zoom = 2_u32.pow(zoom.into()) as f32;

        let mut coord = TileCoord {
            pos: Vec2 {
                // Longitude, (Längengrad) West/East "index"
                x: ((self.lon + 180.) / 360. * pow_zoom).rem_euclid(pow_zoom),

                // y: Latitude, (Breitengrad) Nort/South "index"
                y: ((1.
                    - (self.lat.to_radians().tan() + 1. / self.lat.to_radians().cos()).ln() / PI)
                    / 2.
                    * pow_zoom),
                // The Nort/South y tile name part is not linear, the tiles gets stretched to the poles
                // to compensate the stretching if the stretching of the West/East projection
            },
            zoom,
        };
        // If out of bounds, wrap around the globe.
        // Note: only works if the gps coordinates weren't out of bounds enough to wrap around the planet beyond the equator.
        if coord.pos.y > pow_zoom {
            coord.pos.y = pow_zoom - coord.pos.y.rem_euclid(pow_zoom);
            coord.pos.x = (coord.pos.x + pow_zoom / 2.0).rem_euclid(pow_zoom);
        } else if coord.pos.y.is_sign_negative() {
            coord.pos.y = coord.pos.y.abs();
            coord.pos.x = (coord.pos.x + pow_zoom / 2.0).rem_euclid(pow_zoom);
        }
        if coord.pos.x > pow_zoom || coord.pos.y > pow_zoom {
            panic!("{self:?} @ zoom {zoom} -> {coord:?}");
        }
        coord
    }

    pub fn to_cartesian(self) -> DVec3 {
        let geo = GeographicPoint::new(
            (self.lon as f64).to_radians(),
            (self.lat as f64).to_radians(),
            EARTH_RADIUS as f64,
        );
        let cart = CartesianPoint::from_geographic(&geo);
        DVec3::new(-cart.x(), -cart.y(), cart.z())
    }

    pub fn from_cartesian(pos: DVec3) -> Self {
        let cart = CartesianPoint::new(-pos.x, -pos.y, pos.z);
        let geo = GeographicPoint::from_cartesian(&cart);
        GeoPos {
            lat: geo.latitude().to_degrees() as f32,
            lon: geo.longitude().to_degrees() as f32,
        }
    }

    /// Tile width and height in meters
    pub fn tile_size(self, zoom: u8) -> Vec2 {
        let coord = self.to_tile_coordinates(zoom);
        let pos = self.to_cartesian();
        let x = TileCoord {
            pos: coord.pos + Vec2::X,
            zoom,
        }
        .to_geo_pos()
        .to_cartesian()
        .distance(pos) as f32;
        let y = TileCoord {
            pos: coord.pos + Vec2::Y,
            zoom,
        }
        .to_geo_pos()
        .to_cartesian()
        .distance(pos) as f32;
        Vec2 { x, y }
    }
}

pub const EARTH_RADIUS: f32 = 6_378_000.;
pub const MOON_RADIUS: f32 = 1_737_400.;
pub const MOON_ORBIT: f32 = 384_400_000.;
