use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use big_space::FloatingOriginSettings;
use glam::DVec3;

#[cfg(not(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32")))))]
use crate::compass::OpenXRTrackingRoot;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;

use crate::compass::Compass;
use crate::geocoord::GeoCoord;
use crate::geoview::GeoView;
use crate::GalacticTransform;
use crate::GalacticTransformOwned;

#[derive(SystemParam)]
/// A helper argument for bevy systems that obtains the main player's position.
pub struct Player<'w, 's> {
    pub(crate) xr_pos: Query<
        'w,
        's,
        GalacticTransform,
        (With<OpenXRTrackingRoot>, Without<Control>, Without<Compass>),
    >,
    pub(crate) flycam_pos: Query<
        'w,
        's,
        GalacticTransform,
        (With<Control>, Without<OpenXRTrackingRoot>, Without<Compass>),
    >,
    pub(crate) space: Res<'w, FloatingOriginSettings>,
}

/// A helper for working with positions relative to the planet center.
#[derive(Clone, Copy)]
pub struct PlanetaryPosition {
    // todo?  eliminate membername pos by unnamed struct with (DVec3);
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
    pub fn to_galactic_transform_space(
        self,
        space: &FloatingOriginSettings,
    ) -> GalacticTransformSpace<'_> {
        let (cell, pos) = space.translation_to_grid(self.pos);
        let transform = Transform::from_translation(pos);
        let galactic_transform = GalacticTransformOwned { transform, cell };
        GalacticTransformSpace {
            galactic_transform,
            space,
        }
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
pub struct GalacticTransformSpace<'a> {
    pub galactic_transform: GalacticTransformOwned,
    pub space: &'a FloatingOriginSettings,
}

impl<'a> std::ops::DerefMut for GalacticTransformSpace<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.galactic_transform
    }
}

impl<'a> std::ops::Deref for GalacticTransformSpace<'a> {
    type Target = GalacticTransformOwned;

    fn deref(&self) -> &Self::Target {
        &self.galactic_transform
    }
}

impl GalacticTransformSpace<'_> {
    /// Compute the cartesian coordinates by combining the grid cell and the position from within
    /// the grid.
    pub fn pos(&self) -> DVec3 {
        self.galactic_transform.position_double(self.space)
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
    pub fn pos(&self) -> GalacticTransformSpace<'_> {
        let galactic_transform = if let Ok(xr_pos) = self.xr_pos.get_single() {
            xr_pos
        } else {
            self.flycam_pos.single()
        }
        .to_owned();
        GalacticTransformSpace {
            galactic_transform,
            space: &self.space,
        }
    }

    pub fn set_pos(&mut self, new_pos: GalacticTransformSpace<'_>) {
        let mut pos = if let Ok(xr_pos) = self.xr_pos.get_single_mut() {
            xr_pos
        } else {
            self.flycam_pos.single_mut()
        };
        *pos.cell = new_pos.cell;
        *pos.transform = new_pos.transform;
    }
}

/// A marker component used in queries when you want camera-controls
#[derive(Component)]
pub struct Control;

/// Used by all controlers: Mouse sensitivity and movement speed, up vector and set view
#[derive(Resource)]
pub struct ControlValues {
    pub sensitivity: f32,
    pub speed: f32,
    pub up: Vec3,
    pub view: GeoView,
}

impl Default for ControlValues {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
            up: Vec3::Y,
            view: GeoView::new(),
        }
    }
}

/*  plugin?
pub fn init_controls(&mut App) {
        app
            .init_resource::<MovementValues>()
}
 */
