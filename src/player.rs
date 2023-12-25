use crate::geocoord::GeoCoord;
use crate::GalacticTransform;
use crate::GalacticTransformOwned;

use super::Compass;
use super::OpenXRTrackingRoot;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use big_space::FloatingOriginSettings;
use glam::DVec3;

#[derive(SystemParam)]
/// A helper argument for bevy systems that obtains the main player's position.
pub struct Player<'w, 's> {
    pub(crate) xr_pos: Query<
        'w,
        's,
        GalacticTransform,
        (With<OpenXRTrackingRoot>, Without<FlyCam>, Without<Compass>),
    >,
    pub(crate) flycam_pos: Query<
        'w,
        's,
        GalacticTransform,
        (With<FlyCam>, Without<OpenXRTrackingRoot>, Without<Compass>),
    >,
    pub(crate) space: Res<'w, FloatingOriginSettings>,
}

/// A helper for working with positions relative to the planet center.
#[derive(Clone, Copy)]
pub struct PlanetaryPosition {
    // todo?  eliminate pos by (DVec3);
    pub pos: DVec3,
}

impl From<PlanetaryPosition> for DVec3 {
    fn from(value: PlanetaryPosition) -> Self {
        value.pos
    }
}

impl std::ops::Deref for PlanetaryPosition {
    type Target = DVec3;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl PlanetaryPosition {
    pub fn to_galactic_position(self, space: &FloatingOriginSettings) -> GalacticPosition<'_> {
        let (cell, pos) = space.translation_to_grid(self.pos);
        let transform = Transform::from_translation(pos);
        let pos = GalacticTransformOwned { transform, cell };
        GalacticPosition { pos, space }
    }

    pub fn directions(self) -> Directions {
        let up = self.pos.normalize().as_vec3();
        let west = Vec3::Z.cross(up);
        let north = up.cross(west);
        Directions { up, north, west }
    }

    pub fn to_geocoord(self) -> GeoCoord {
        GeoCoord::from_cartesian(self.pos)
    }
}

/// A helper for working with galactic positions.
#[derive(Copy, Clone)]
pub struct GalacticPosition<'a> {
    pub pos: GalacticTransformOwned,
    pub space: &'a FloatingOriginSettings,
}

impl<'a> std::ops::DerefMut for GalacticPosition<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pos
    }
}

impl<'a> std::ops::Deref for GalacticPosition<'a> {
    type Target = GalacticTransformOwned;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl GalacticPosition<'_> {
    /// Compute the cartesian coordinates by combining the grid cell and the position from within
    /// the grid.
    pub fn pos(&self) -> DVec3 {
        self.pos.position_double(self.space)
    }

    /// Calculates cardinal directions at any cartesian position.
    pub fn directions(&self) -> Directions {
        let up = self.pos().normalize().as_vec3();
        let west = Vec3::Z.cross(up);
        let north = up.cross(west);
        Directions { up, north, west }
    }

    pub fn to_planetary_position(self) -> PlanetaryPosition {
        let pos = self.pos();
        PlanetaryPosition { pos }
    }
}

/// A coordinate system where "forward" is north, "right" is west and "up" is away from the planet.
pub struct Directions {
    pub up: Vec3,
    pub north: Vec3,
    pub west: Vec3,
}

impl<'w, 's> Player<'w, 's> {
    /// Computes the galactic position of the main player (prefers XR player).
    pub fn pos(&self) -> GalacticPosition<'_> {
        let pos = if let Ok(xr_pos) = self.xr_pos.get_single() {
            xr_pos
        } else {
            self.flycam_pos.single()
        }
        .to_owned();
        GalacticPosition {
            pos,
            space: &self.space,
        }
    }

    pub fn set_pos(&mut self, new_pos: GalacticPosition<'_>) {
        let mut pos = if let Ok(xr_pos) = self.xr_pos.get_single_mut() {
            xr_pos
        } else {
            self.flycam_pos.single_mut()
        };
        *pos.cell = new_pos.cell;
        *pos.transform = new_pos.transform;
    }
}
