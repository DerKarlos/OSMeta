use bevy::ecs::event::ManualEventReader;
use bevy::ecs::system::SystemParam;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
//use gl am::DVec3;
use bevy::math::DVec3;

#[cfg(not(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32")))))]
use crate::compass::OpenXRTrackingRoot;
#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;

use crate::big_space::{FloatingOrigin, Space};
use crate::compass::Compass;
use crate::sky::Galactica;
use crate::geocoord::{GeoCoord, EARTH_RADIUS};
use crate::geoview::GeoView;
use crate::GalacticGrid;
use crate::GalacticTransform;
use crate::GalacticTransformOwned;

/// Keeps track of mouse motion events
#[derive(Resource, Default)]
pub struct InputState {
    pub reader_motion: ManualEventReader<MouseMotion>, // todo? add to ControlValues ???
}

#[derive(SystemParam)]
/// A helper argument for bevy systems that obtains the main player's position.
pub struct PlayerQuery<'w, 's> {
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
    //removed: pub(crate) space: Res<'w, FloatingOriginSettings>,
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
    pub fn to_galactic_transform_space(self) -> PlayerGalacticTransform {
        let (cell, pos) = Space::translation_to_grid(self.pos);
        let transform = Transform::from_translation(pos);
        let galactic_transform = GalacticTransformOwned { transform, cell };
        PlayerGalacticTransform { galactic_transform }
    }

    pub fn to_galactic_transform(self) -> GalacticTransformOwned {
        let (cell, pos) = Space::translation_to_grid(self.pos);
        let transform = Transform::from_translation(pos);
        GalacticTransformOwned { transform, cell }
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
pub struct PlayerGalacticTransform {
    pub galactic_transform: GalacticTransformOwned,
}

impl std::ops::DerefMut for PlayerGalacticTransform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.galactic_transform
    }
}

impl std::ops::Deref for PlayerGalacticTransform {
    type Target = GalacticTransformOwned;

    fn deref(&self) -> &Self::Target {
        &self.galactic_transform
    }
}

impl PlayerGalacticTransform {
    /// Compute the cartesian coordinates by combining the grid cell and the position from within
    /// the grid.
    pub fn pos(&self) -> DVec3 {
        self.galactic_transform.position_double()
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

impl<'w, 's> PlayerQuery<'w, 's> {
    /// Computes the galactic position of the main player (prefers XR player).
    pub fn pos(&self) -> PlayerGalacticTransform {
        let galactic_transform = if let Ok(xr_pos) = self.xr_pos.get_single() {
            xr_pos
        } else {
            self.comtrol_pos.single()
        }
        .to_owned();
        PlayerGalacticTransform { galactic_transform }
    }

    pub fn set_pos(&mut self, new_pos: GalacticTransformOwned) {
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

#[derive(Clone, Copy, PartialEq)]
pub enum CamControlMode {
    F4,
    Fly,
    // todo: more to come
}

/// Used by all controlers: Mouse sensitivity and movement speed, up vector and set view
#[derive(Resource)]
pub struct ControlValues {
    pub sensitivity: f32,
    pub speed: f32,
    pub up: Vec3,
    pub view: GeoView,
    pub cam_control_mode: CamControlMode,
}

impl Default for ControlValues {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
            up: Vec3::Y,
            view: GeoView::default(),
            cam_control_mode: CamControlMode::F4,
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
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,        
    )
}

fn update_camera_speed(
    mut control_values: ResMut<ControlValues>,
    fly_cam:    Query<GalacticTransform, (With<Control  >, Without<Galactica>)>,
    galacticas: Query<GalacticTransform, (With<Galactica>, Without<Control  >)>,
) {
    // Distance to Earth surface
    let pos_cam = fly_cam.single().position_double();
    let elevation = pos_cam.length() as f32;
    let mut distance_to_focus = elevation - crate::geocoord::EARTH_RADIUS;
    if control_values.cam_control_mode == CamControlMode::F4 {
        distance_to_focus += control_values.view.distance
    };

    // Use Calactica if more close
    if let Ok(galactica) = galacticas.get_single() {
        let pos_galactica = galactica.position_double();
        let dis_galactica = (pos_cam - pos_galactica).length() as f32;
        distance_to_focus = dis_galactica.min(distance_to_focus);
    }

    let speed = (distance_to_focus * 2. - 300.0).max(100.0);
    control_values.speed = speed;
}

pub fn setup_player_controls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    starting_values: Res<crate::StartingValues>,
    mut control_values: ResMut<ControlValues>,
) {
    control_values.cam_control_mode = starting_values.cam_control_mode;

    // set up accroding to lat/lon relative to Earth center
    control_values.up = starting_values.planetary_position.normalize().as_vec3();

    let (grid, _): (GalacticGrid, _) =
        Space::translation_to_grid(starting_values.planetary_position);

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let mesh = meshes.add(Sphere::new(1.0).mesh().ico(10).unwrap());
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
            .init_resource::<InputState>()
            .add_systems(Startup, setup_player_controls)
            .add_systems(Update, update_camera_speed);
    }
}

pub const OSM_LAT_LIMIT: f32 = 85.0511; // degrees
