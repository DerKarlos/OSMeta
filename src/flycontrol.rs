//! This module contains everything about the fly-controls and rendering related
//! to the non-VR "player". Todo: update
//!
//! A Copy of "https://github.com/oli-obk/bevy_flycam", branch = "arbitrary_up", a
//! branch of "https://github.com/sburris0/bevy_flycam"

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::player::{Control, ControlValues, InputState};
use crate::GalacticTransform;

pub mod prelude {
    pub use crate::*;
}

// Todo? enum intead of struct???
pub const _KEY_MOVE_FORWARD: KeyCode = KeyCode::W;
// Todo: KeyBindings ALL at once, also for Save/store F4Camm???
/// Key configuration
#[derive(Resource)]
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

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    control_values: Res<ControlValues>,
    key_bindings: Res<KeyBindings>,
    mut query: Query<(&Control, &mut Transform)>, //    mut query: Query<&mut Transform, With<Control>>,
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

                        //  match key {
                        //      KEY_MOVE_FORWARD => {velocity += forward;},
                        //      _ => (),
                        //  };

                        if key == key_bindings.move_forward {
                            velocity += forward;
                        } else if key == key_bindings.move_backward {
                            velocity -= forward;
                        } else if key == key_bindings.move_left {
                            velocity -= right;
                        } else if key == key_bindings.move_right {
                            velocity += right;
                        } else if key == key_bindings.move_ascend {
                            velocity += control_values.up;
                        } else if key == key_bindings.move_descend {
                            velocity -= control_values.up;
                        }
                    }
                }

                velocity = velocity.normalize_or_zero();

                transform.translation += velocity * time.delta_seconds() * control_values.speed
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<ControlValues>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<Control>>,
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

// Grab cursor when an entity with Fly-Cam is added
fn initial_grab_on_fly_control_spawn(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    query_added: Query<Entity, Added<Control>>,
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
            .add_systems(Update, grab_cursor)
            // This was the "no_camera_player_plugin"
            // Contains everything needed to add first-person fly camera behavior to your game, but does not spawn a camera
            .init_resource::<InputState>()
            .init_resource::<ControlValues>()
            .init_resource::<KeyBindings>()
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(Startup, initial_grab_on_fly_control_spawn)
            .add_systems(Update, player_move)
            .add_systems(Update, player_look)
            .add_systems(Update, cursor_grab);
    }
}

fn setup(mut keys: ResMut<KeyBindings>) {
    // Don't use ESC for grabbing/releasing the cursor. That's what browsers use, too, so it gets grabbed by bevy and released by the browser at the same time.
    keys.toggle_grab_cursor = KeyCode::G;
}

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
    mut control_values: ResMut<ControlValues>,
    mut fly_cam: Query<GalacticTransform, With<Control>>,
) {
    // the only controled camera's GalacticTransform <grid,f32>
    let mut fly_cam = fly_cam.single_mut();

    let up = fly_cam
        .position_double()
        .normalize() // direction from galactic NULL = from the Earth center
        .as_vec3();
    control_values.up = up;

    // Reorient "up" axis without introducing other rotations.
    let forward = fly_cam.transform.forward();
    fly_cam.transform.look_to(forward, up);
}
