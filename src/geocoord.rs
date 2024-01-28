use crate::{player::PlanetaryPosition, tilemap::TileCoord};
use bevy::prelude::*;
use glam::DVec3;
use globe_rs::{CartesianPoint, GeographicPoint};
use std::f32::consts::PI;

/**
 * Geo-coordinates on the (OSM-) world map (GPS position)
 *
 * Does also calculations: tile_Name, the coordinates aer located in
 * and distance in meters from the tiles corner
 */

#[derive(Default, Debug, Clone, Copy)]
pub struct GeoCoord {
    pub lat: f32,
    pub lon: f32,
}

impl GeoCoord {
    /**
     * Convert GPS coordinates to tile coordinates.
     * We use the OSM naming for tiles:
     * https://wiki.openstreetmap.org/wiki/Slippy_map_tilenames.
     * x and y relate to a lonitude and latitude-position on Earth.
     * In OSM he two values are only used as part of the filename of a tile.
     * even if it is an x/y coordinate in numbers.
     * @param zoom  Zoom level of the OSM tile-name(x/y) system
     * @return coordinate in tile coordinates
     */
    pub fn to_tile_coordinates(self, zoom: u8) -> TileCoord {
        let pow_zoom = 2_u32.pow(zoom.into()) as f32;

        // Longitude, (Längengrad) West/East "index"
        let mut x = ((self.lon + 180.) / 360. * pow_zoom).rem_euclid(pow_zoom);
        // y: Latitude, (Breitengrad) Nort/South "index"
        let mut y =
            (1. - (self.lat.to_radians().tan() + 1. / self.lat.to_radians().cos()).ln() / PI) / 2.
                * pow_zoom;
        // The Nort/South y tile name part is not linear, the tiles gets stretched to the poles
        // to compensate the stretching if the stretching of the West/East projection

        // If out of bounds, wrap around the globe.
        // Note: only works if the gps coordinates weren't out of bounds enough to wrap around the planet beyond the equator.
        if y > pow_zoom {
            y = pow_zoom - y.rem_euclid(pow_zoom);
            x = (x + pow_zoom / 2.0).rem_euclid(pow_zoom);
        } else if y.is_sign_negative() {
            y = y.abs();
            x = (x + pow_zoom / 2.0).rem_euclid(pow_zoom);
        }
        if x > pow_zoom || y > pow_zoom {
            panic!("{self:?} @ zoom {zoom} -> {x},{y}");
        }
        TileCoord::new(Vec2 { x, y }, zoom)
    }

    /// Compute the planet-position on the surface.
    pub fn to_cartesian(self) -> PlanetaryPosition {
        let geo = GeographicPoint::new(
            (self.lon as f64).to_radians(),
            (self.lat as f64).to_radians(),
            EARTH_RADIUS as f64,
        );
        let cart = CartesianPoint::from_geographic(&geo);
        let pos = DVec3::new(-cart.x(), -cart.y(), cart.z());
        PlanetaryPosition { pos }
    }

    pub fn from_cartesian(pos: DVec3) -> Self {
        let cart = CartesianPoint::new(-pos.x, -pos.y, pos.z);
        let geo = GeographicPoint::from_cartesian(&cart);
        GeoCoord {
            lat: geo.latitude().to_degrees() as f32,
            lon: geo.longitude().to_degrees() as f32,
        }
    }

    /// Tile width and height in meters (are equal)
    pub fn tile_size(self, zoom: u8) -> f32 {
        let coord = self.to_tile_coordinates(zoom);
        let pos = self.to_cartesian();
        coord.right().to_geo_coord().to_cartesian().distance(*pos) as f32
    }

    /// Add a displacement
    pub fn add_move(&mut self, moved: GeoDir) {
        self.lat += moved.y;
        self.lon += moved.x;

    }
}



pub type GeoDir = Vec2;

pub trait GeoDirTrait {
    fn forward(dir: f32) -> Vec2;
    fn right(dir: f32) -> Vec2;
}


impl GeoDirTrait for GeoDir {

    fn forward(dir: f32) -> Vec2 {
        let (sin, cos) = dir.sin_cos();
        Vec2{x: -sin, y: cos}
    }

    fn right(dir: f32) -> Vec2 {
        let (sin, cos) = dir.sin_cos();
        Vec2{x: cos, y: sin}
    }
}




pub const EARTH_RADIUS: f32 = 6_378_000.;
pub const MOON_RADIUS: f32 = 01_737_400.;
pub const MOON_ORBIT: f32 = 384_400_000.;
