use std::fmt::Display;

use super::coord::TileCoord;
use crate::GalacticGrid;
use bevy::prelude::*;
use big_space::FloatingOriginSettings;

/// An x/y index of an OWM tile.
#[derive(Debug, Copy, Clone, Component, Hash, PartialEq, Eq)]
pub struct TileIndex {
    idx: UVec2,
    zoom: u8,
}

impl std::ops::Deref for TileIndex {
    type Target = UVec2;

    fn deref(&self) -> &Self::Target {
        &self.idx
    }
}

impl TileIndex {
    pub fn as_coord(self) -> TileCoord {
        self.into()
    }
    pub fn right(self) -> Self {
        Self {
            idx: self.idx + UVec2::X,
            ..self
        }
    }
    pub fn down(self) -> Self {
        Self {
            idx: self.idx + UVec2::Y,
            ..self
        }
    }

    pub fn distance_squared(&self, origin: TileIndex) -> u32 {
        assert_eq!(self.zoom, origin.zoom);
        let max_tiles = 2_u32.pow(self.zoom.into());
        let mut x = self.idx.x.abs_diff(origin.idx.x);
        x = x.min(max_tiles - x);
        let mut y = self.idx.y.abs_diff(origin.idx.y);
        y = y.min(max_tiles - y);
        x * x + y * y
    }

    pub fn offset(self, offset: IVec2) -> TileIndex {
        let max_tiles = 2_i32.pow(self.zoom.into());
        let mut idx = self.idx.as_ivec2() + offset;
        if idx.x < 0 {
            idx.x += max_tiles;
        }
        idx.x %= max_tiles;
        if idx.y < 0 {
            idx.y += max_tiles;
        }
        idx.y %= max_tiles;
        TileIndex {
            idx: idx.as_uvec2(),
            zoom: self.zoom,
        }
    }

    pub fn to_cartesian(self, space: &FloatingOriginSettings) -> (GalacticGrid, Transform) {
        let coord = self.as_coord().center();
        let pos = coord.to_geo_pos().to_cartesian();
        let up = pos.normalize().as_vec3();
        let next = coord.up().to_geo_pos().to_cartesian();
        let (grid, pos) = space.translation_to_grid(pos);
        let (grid_next, next) = space.translation_to_grid(next);
        let diff = grid_next - grid;
        let diff = Vec3 {
            x: diff.x as f32 * space.grid_edge_length(),
            y: diff.y as f32 * space.grid_edge_length(),
            z: diff.z as f32 * space.grid_edge_length(),
        };
        let next = next + diff;
        (
            grid,
            Transform::from_translation(pos).looking_to(next - pos, up),
        )
    }

    pub fn zoom(&self) -> u8 {
        self.zoom
    }

    pub fn from_coord_lossy(arg: TileCoord) -> TileIndex {
        Self {
            idx: arg.as_uvec2(),
            zoom: arg.zoom(),
        }
    }
}

impl Display for TileIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.idx.fmt(f)
    }
}
