/*
    To welcome our users, we offer a camera control as it is used with OpenStreetMap 3D rendering
    The first was www.f4map.com . We offer the same key, mouse and wheel (todo: touch)
    But we extend it with more keys for all mouse moves too

    Overview:

    'KeyA', 'ArrowLeft', // cursor keys
    'KeyD', 'ArrowRight',
    'KeyW', 'ArrowUp',
    'KeyS', 'ArrowDown',

    'KeyQ', 'KeyE', // rotate
    'KeyR', 'KeyF', // nick
    'KeyG', 'KeyT', // elevate

    'KeyY', 'KeyH', // zoom (Y=Z at German keyboard) Mind the Compas! ???

    'PageUp', 'PageDown',
    'Backslash', 'BracketRight', // Left of "Enter"; UK or US keyboard: ] and \ German keypbard: + and #

    'OSLeft', 'OSRight',
    'metaKey',  // Chrome OSkey
    'shiftKey', // 'ShiftLeft', 'ShiftRight',


We start with an argumente to select one control and lager switch dynamically.
All controls will have the resource type control later (now FlyCam)
Maximal one control/plurgin/systems should run (may be none)

What about the Player? Is it for FlyCam or for all controls

 */

use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::GalacticGrid;
use big_space::{FloatingOrigin, FloatingOriginSettings};

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
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::W,
            move_backward: KeyCode::S,
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_ascend: KeyCode::T,  // TODO use + and PgUp also
            move_descend: KeyCode::G, // TODO use # and PgDown also
        }
    }
}

/// Used in queries when you want f4controls and not other cameras
/// A marker component used in queries when you want f4controls and not other cameras
// #[derive(Component)]
// pub struct FlyCam;
use bevy_flycam::FlyCam; // not F4Control  Todo: name it CamControl for the just running control  --

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    settings: Res<MovementSettings>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&FlyCam, &mut Transform)>, //    mut query: Query<&mut Transform, With<F4Control>>,
) {
    if let Ok(_window) = primary_window.get_single() {
        for (_camera, mut transform) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let forward = transform.forward();
            let right = transform.right();

            for key in keys.get_pressed() {
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

/// Same as [`PlayerPlugin`] but does not spawn a camera
pub struct NoCameraPlayerPlugin;
impl bevy::prelude::Plugin for NoCameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .init_resource::<KeyBindings>()
            .add_systems(Update, player_move)
            .add_systems(Update, player_look);
    }
}

fn setup(
    mut commands: Commands,
    mut movement_settings: ResMut<MovementSettings>,
    args: Res<crate::Args>,
    space: Res<FloatingOriginSettings>,
) {
    // set up accroding to lat/lon relative to Earth center
    movement_settings.up = args.starting_position.normalize().as_vec3();
    let (grid, _): (GalacticGrid, _) = space.translation_to_grid(args.starting_position);

    let mut camera = commands.spawn((
        Camera3dBundle { ..default() },
        InheritedVisibility::default(),
        FlyCam,
        grid,
    ));
    camera.insert(FloatingOrigin);
    movement_settings.speed = 100.0;
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app .add_systems(Startup, setup)
            .add_plugins(NoCameraPlayerPlugin) // https://github.com/sburris0/bevy_flycam (bevy_config_cam dies not work wiht Bevy 12)
            ;
    }
}
