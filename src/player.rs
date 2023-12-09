use super::Compass;
use super::GalacticGrid;
use super::OpenXRTrackingRoot;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_flycam::FlyCam;
use big_space::FloatingOriginSettings;
use glam::DVec3;

#[derive(SystemParam)]
pub struct Player<'w, 's> {
    pub(crate) xr_pos: Query<
        'w,
        's,
        (&'static Transform, &'static GalacticGrid),
        (With<OpenXRTrackingRoot>, Without<FlyCam>, Without<Compass>),
    >,
    pub(crate) flycam_pos: Query<
        'w,
        's,
        (&'static Transform, &'static GalacticGrid),
        (With<FlyCam>, Without<OpenXRTrackingRoot>, Without<Compass>),
    >,
    pub(crate) space: Res<'w, FloatingOriginSettings>,
}

pub struct PlayerPosition<'a> {
    pub transform: Transform,
    pub grid: GalacticGrid,
    pub space: &'a FloatingOriginSettings,
}

impl PlayerPosition<'_> {
    pub fn pos(&self) -> DVec3 {
        self.space.grid_position_double(&self.grid, &self.transform)
    }
    pub fn directions(&self) -> Directions {
        let up = self.pos().normalize().as_vec3();
        let west = Vec3::Z.cross(up);
        let north = up.cross(west);
        Directions { up, north, west }
    }
}

pub struct Directions {
    pub up: Vec3,
    pub north: Vec3,
    pub west: Vec3,
}

impl<'w, 's> Player<'w, 's> {
    pub fn pos(&self) -> PlayerPosition<'_> {
        let (&transform, &grid) = if let Ok(xr_pos) = self.xr_pos.get_single() {
            xr_pos
        } else {
            self.flycam_pos.single()
        };
        PlayerPosition {
            transform,
            grid,
            space: &self.space,
        }
    }
}
