use super::TileIndex;
use crate::geopos::GeoPos;
use bevy::prelude::*;
use std::f32::consts::PI;

/// A coordinate in the OWM tile coordinate system.
/// We use floats instead of integers so we can specify positions of objects
/// within a tile. E.g. (0.5, 0.5) is the position in the middle of tile (0, 0).
#[derive(Debug, Copy, Clone)]
pub struct TileCoord {
    pos: Vec2,
    zoom: u8,
}

impl std::ops::Deref for TileCoord {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl From<TileIndex> for TileCoord {
    fn from(value: TileIndex) -> Self {
        TileCoord {
            pos: value.as_vec2(),
            zoom: value.zoom(),
        }
    }
}

impl TileCoord {
    pub fn to_geo_pos(self) -> GeoPos {
        let pow_zoom = 2_u32.pow(self.zoom.into()) as f32;

        let lon = self.x / pow_zoom * 360.0 - 180.0;
        let lat_rad = (PI * (1. - 2. * self.y / pow_zoom)).sinh().atan();
        let lat = lat_rad.to_degrees();
        GeoPos { lat, lon }
    }

    /// Offset this position by half a tile size. If you started out with a left upper
    /// corner position, you'll now be in the middle of the tile.
    pub fn center(self) -> Self {
        Self {
            pos: self.pos + 0.5,
            zoom: self.zoom,
        }
    }

    pub fn up(self) -> Self {
        Self {
            pos: self.pos - Vec2::Y,
            ..self
        }
    }

    pub fn right(self) -> Self {
        Self {
            pos: self.pos + Vec2::X,
            ..self
        }
    }

    pub fn down(self) -> Self {
        Self {
            pos: self.pos + Vec2::Y,
            ..self
        }
    }

    pub fn as_tile_index(self) -> TileIndex {
        TileIndex::from_coord_lossy(self)
    }

    pub fn zoom(self) -> u8 {
        self.zoom
    }

    pub fn new(pos: Vec2, zoom: u8) -> TileCoord {
        Self { pos, zoom }
    }
}
