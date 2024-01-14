use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use big_space::{FloatingOrigin, FloatingOriginSettings};
use glam::DVec3;

#[cfg(not(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32")))))]
use crate::compass::OpenXRTrackingRoot;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;

use crate::compass::Compass;
use crate::geocoord::{GeoCoord, EARTH_RADIUS};
use crate::geoview::GeoView;
use crate::GalacticGrid;
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
    pub(crate) comtrol_pos: Query<
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
            self.comtrol_pos.single()
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
            self.comtrol_pos.single_mut()
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

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

pub fn setup_player_controls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    space: Res<FloatingOriginSettings>,
    starting_values: Res<crate::StartingValues>,
) {
    let (grid, _): (GalacticGrid, _) =
        space.translation_to_grid(starting_values.planetary_position);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 1.0,
            subdivisions: 10,
        }
        .try_into()
        .unwrap(),
    );
    let sphere = commands
        .spawn(PbrBundle {
            mesh,
            material,
            ..default()
        })
        .id();

    let mut camera = commands.spawn((
        Camera3dBundle { ..default() },
        InheritedVisibility::default(),
        Control,
        grid,
        FogSettings {
            color: Color::rgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                EARTH_RADIUS * 2.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    ));
    camera.add_child(sphere);
    if !starting_values.xr {
        camera.insert(FloatingOrigin);
    }
    // FIXME: attach the camera bundle to the world, so when we move the world, the player is automatically moved with it.
    // We'll need this when the player moves very far or teleports to another place, as we need to ensure we don't go into
    // regions where the floating point numbers become imprecise.
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlValues>()
            .add_systems(Startup, setup_player_controls);
    }
}
