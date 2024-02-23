//! A helper query argument that ensures you don't forget to handle
//! the [`GridCell`] when you work with a [`Transform`].

use bevy::ecs::query::QueryData;
use bevy::math::DVec3;
use bevy::prelude::*;

use super::precision::GridPrecision;
use super::{GridCell, Space};

#[derive(QueryData)]
#[query_data(mutable)]
/// A convenience query argument that groups a [`Transform`] with its [`GridCell`].
/// If you only want to read from the position, use [`GridTransformReadOnly`] instead,
/// as this will allow the bevy ECS to run multiple queries using [`GridTransformReadOnly`]
/// at the same time (just like multiple queries with `&Transform` are fine).
pub struct GridTransform<P: GridPrecision> {
    /// Grid local transform
    pub transform: &'static mut Transform,
    /// The grid to which `transform` is relative to.
    pub cell: &'static mut GridCell<P>,
}

impl<'w, P: GridPrecision> GridTransformItem<'w, P> {
    /// Compute the global position with double precision.
    pub fn position_double(&self) -> DVec3 {
        Space::grid_position_double(&self.cell, &self.transform)
    }

    /// Compute the global position.
    pub fn _position(&self) -> Vec3 {
        Space::grid_position(&self.cell, &self.transform)
    }

    /// Get a copy of the fields to work with.
    pub fn _to_owned(&self) -> GridTransformOwned<P> {
        GridTransformOwned {
            transform: *self.transform,
            cell: *self.cell,
        }
    }
}

impl<'w, P: GridPrecision> GridTransformReadOnlyItem<'w, P> {
    /// Compute the global position with double precision.
    pub fn position_double(&self) -> DVec3 {
        Space::grid_position_double(self.cell, self.transform)
    }

    /// Compute the global position.
    pub fn _position(&self) -> Vec3 {
        Space::grid_position(self.cell, self.transform)
    }

    /// Get a copy of the fields to work with.
    pub fn to_owned(&self) -> GridTransformOwned<P> {
        GridTransformOwned {
            transform: *self.transform,
            cell: *self.cell,
        }
    }
}

/// A convenience wrapper that allows working with grid and transform easily
#[derive(Copy, Clone)]
pub struct GridTransformOwned<P: GridPrecision> {
    /// Grid local transform
    pub transform: Transform,
    /// The grid to which `transform` is relative to.
    pub cell: GridCell<P>,
}

/**/
/// A coordinate system where "forward" is north, "right" is west and "up" is away from the planet.
pub struct Directions {
    pub up: Vec3,
    pub north: Vec3,
    pub west: Vec3,
}

impl<P: GridPrecision> GridTransformOwned<P> {
    /// Calculates cardinal directions at any cartesian position.
    pub fn directions(&self) -> Directions {
        let up = self.transform.translation.normalize();
        let west = Vec3::Z.cross(up);
        let north = up.cross(west);
        Directions { up, north, west }
    }
}
/**/

impl<P: GridPrecision> std::ops::Sub for GridTransformOwned<P> {
    type Output = Self;

    /// Compute a new transform that maps from `source` to `self`.
    fn sub(mut self, source: Self) -> Self {
        self.cell -= source.cell;
        self.transform.translation -= source.transform.translation;
        self.transform.scale /= source.transform.scale;
        self.transform.rotation *= source.transform.rotation.inverse();
        self
    }
}

impl<P: GridPrecision> std::ops::Add for GridTransformOwned<P> {
    type Output = Self;

    /// Compute a new transform that shifts, scales and rotates `self` by `diff`.
    fn add(mut self, diff: Self) -> Self {
        self.cell += diff.cell;
        self.transform.translation += diff.transform.translation;
        self.transform.scale *= diff.transform.scale;
        self.transform.rotation *= diff.transform.rotation;
        self
    }
}

impl<P: GridPrecision> GridTransformOwned<P> {
    /// Compute the global position with double precision.
    pub fn position_double(&self) -> DVec3 {
        Space::grid_position_double(&self.cell, &self.transform)
    }

    /// Compute the global position.
    pub fn position(&self) -> Vec3 {
        Space::grid_position(&self.cell, &self.transform)
    }
}
