//! This module contains everything about the controls and rendering related
//! to the non-VR "player". Todo: update

// A Copy of "https://github.com/oli-obk/bevy_flycam", branch = "arbitrary_up", a
// branch of "https://github.com/sburris0/bevy_flycam"

use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::window::{CursorGrabMode, PrimaryWindow};

use big_space::{FloatingOrigin, FloatingOriginSettings};

use crate::geocoord::EARTH_RADIUS;
use crate::{GalacticGrid, GalacticTransform};

pub mod prelude {
    pub use crate::*;
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub up: Vec3,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
            up: Vec3::Y,
        }
    }
}

/// Key configuration
#[derive(Resource)]
//todo: KeyBindings ALL at once, also for Save/store F4Camm???
struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    pub toggle_grab_cursor: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ShiftLeft,
            toggle_grab_cursor: KeyCode::Escape,
        }
    }
}

/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/// Spawns the `Camera3dBundle` to be controlled  Todo: delete
fn _setup_player(mut commands: Commands, settings: Res<MovementSettings>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, settings.up),
            ..Default::default()
        },
        FlyCam,
    ));
}

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&FlyCam, &mut Transform)>, //    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for (_camera, mut transform) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let forward = transform.forward();
            let right = transform.right();

            for key in keys.get_pressed() {
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;
                        if key == key_bindings.move_forward {
                            velocity += forward;
                        } else if key == key_bindings.move_backward {
                            velocity -= forward;
                        } else if key == key_bindings.move_left {
                            velocity -= right;
                        } else if key == key_bindings.move_right {
                            velocity += right;
                        } else if key == key_bindings.move_ascend {
                            velocity += settings.up;
                        } else if key == key_bindings.move_descend {
                            velocity -= settings.up;
                        }
                    }
                }

                velocity = velocity.normalize_or_zero();

                transform.translation += velocity * time.delta_seconds() * settings.speed
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state.reader_motion.read(&motion) {
                let mut yaw = 0.0;
                let mut pitch = 0.0;
                match window.cursor.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                // Order is important to prevent unintended roll
                transform.rotation = Quat::from_axis_angle(settings.up, yaw)
                    * transform.rotation
                    * Quat::from_axis_angle(Vec3::X, pitch);

                let up = transform.up();
                let right = transform.right();
                let pitch = settings.up.cross(up).dot(right).atan2(settings.up.dot(up));
                let restricted_pitch = pitch.clamp(-1.54, 1.54);
                let diff = restricted_pitch - pitch;
                transform.rotate_axis(right, diff);

                // Eliminate accumulated roll and ensure we don't flip onto our heads.
                let forward = transform.forward();
                transform.look_to(forward, settings.up);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(key_bindings.toggle_grab_cursor) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

// Grab cursor when an entity with FlyCam is added
fn initial_grab_on_flycam_spawn(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    query_added: Query<Entity, Added<FlyCam>>,
) {
    if query_added.is_empty() {
        return;
    }

    if let Ok(window) = &mut primary_window.get_single_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

///////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_camera_speed)
            .add_systems(Update, grab_cursor)
            // no_camera_player_plugin
            // Contains everything needed to add first-person fly camera behavior to your game, but does not spawn a camera
            .init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Startup, initial_grab_on_flycam_spawn)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, cursor_grab);
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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut movement_settings: ResMut<MovementSettings>,
    mut keys: ResMut<KeyBindings>,
    starting_values: Res<crate::StartingValues>,
    space: Res<FloatingOriginSettings>,
) {
    // set up accroding to lat/lon relative to Earth center
    movement_settings.up = starting_values.planetary_position.normalize().as_vec3();
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
        FlyCam,
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

    movement_settings.speed = 100.0;
    // Don't use ESC for grabbing/releasing the cursor. That's what browsers use, too, so it gets grabbed by bevy and released by the browser at the same time.
    keys.toggle_grab_cursor = KeyCode::G;
}

fn update_camera_speed(
    mut movement_settings: ResMut<MovementSettings>,
    fly_cam: Query<GalacticTransform, With<FlyCam>>,
    space: Res<FloatingOriginSettings>,
) {
    let elevation = fly_cam.single().position_double(&space).length() as f32;
    let speed = (1. * (elevation - crate::geocoord::EARTH_RADIUS - 300.0)).max(100.0);
    movement_settings.speed = speed;
}

// Todo ? Merge both to fn update? To many different parameters?

fn grab_cursor(
    mut windows: Query<&mut Window>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        if window.cursor.visible {
            // for a game that doesn't use the cursor (like a shooter):
            // use `Locked` mode to keep the cursor in one place
            window.cursor.grab_mode = CursorGrabMode::Locked;
            // also hide the cursor
            window.cursor.visible = false;
        } else {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub fn update_camera_orientations(
    mut movement_settings: ResMut<MovementSettings>,
    mut fly_cam: Query<GalacticTransform, With<FlyCam>>,
    space: Res<FloatingOriginSettings>,
) {
    // the only FlyCam's GalacticTransform <grid,f32>
    let mut fly_cam = fly_cam.single_mut();

    let up = fly_cam
        .position_double(&space)
        .normalize() // direction from galactic NULL = from the Earth center
        .as_vec3();
    movement_settings.up = up;

    // Reorient "up" axis without introducing other rotations.
    let forward = fly_cam.transform.forward();
    fly_cam.transform.look_to(forward, up);
}
