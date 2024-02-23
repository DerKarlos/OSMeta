/*
    This code was done, starting with https://github.com/sburris0/bevy_flycam

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
All controls will have the resource type control later (now Control)
Maximal one control/plurgin/systems should run (may be none)

What about the PlayerQuery? Is it for Fly-Cam or for all controls


See also: https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

TODO: "like F4" reaction while elevate/distance+/-!
The UI of www.F4map.com is very simple:
Arrows left/right: rotate counter-/clockwise
Arrows up/down: tile/shift forward/backward
1st mouse: tile/shilft
2nd mouse: rotate
Mouse wheel: zoom

SAFARI does NOT show buildings propperly! FireFox does.

 */

use bevy::ecs::event::Events;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::geocoord::{GeoDir, GeoDirTrait};
use crate::player::{ControlValues, InputState, PlayerQuery};

/// Key configuration
#[derive(Resource)]
pub struct KeyBindings {
    pub move_forward: KeyCode,
    pub move_forward2: KeyCode,
    pub move_backward: KeyCode,
    pub move_backward2: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_ascend2: KeyCode,
    pub move_descend: KeyCode,
    pub move_descend2: KeyCode,
    //
    pub rotate_up: KeyCode,
    pub rotate_down: KeyCode,
    pub rotate_left: KeyCode,
    pub rotate_left2: KeyCode,
    pub rotate_right: KeyCode,
    pub rotate_right2: KeyCode,
    pub zoom_in: KeyCode,
    pub zoom_out: KeyCode,
    pub zoom_out2: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_forward2: KeyCode::ArrowUp, // F4
            move_backward: KeyCode::KeyS,
            move_backward2: KeyCode::ArrowDown, // F4
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_ascend: KeyCode::KeyT,
            move_ascend2: KeyCode::Backslash, // + on German Mac
            move_descend: KeyCode::KeyG,
            move_descend2: KeyCode::BracketRight, // # on German Mac
            //
            rotate_up: KeyCode::KeyF,
            rotate_down: KeyCode::KeyR,
            rotate_left: KeyCode::KeyQ,
            rotate_left2: KeyCode::ArrowLeft, // F4
            rotate_right: KeyCode::KeyE,
            rotate_right2: KeyCode::ArrowRight, // F4
            zoom_in: KeyCode::KeyH,
            zoom_out: KeyCode::KeyZ,
            zoom_out2: KeyCode::KeyY, // Z on german Keyboards
        }
    }
}

/// Handles keyboard input and movement
fn player_keys(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    time: Res<Time>,
    mut control_values: ResMut<ControlValues>,
    mut player: PlayerQuery,
) {
    if let Ok(_window) = primary_window.get_single() {
        const SPEED_DEGREE_PER_M: f32 = 1.0 / 200000.0;
        let speed = control_values.speed;
        let view = &mut control_values.view;
        let elevation_fakt = 1. + time.delta_seconds() / 1.0;
        let groundmove_fact_lat = speed * time.delta_seconds() * SPEED_DEGREE_PER_M;
        let groundmove_fact_lon = groundmove_fact_lat / view.geo_coord.lat.to_radians().sin();
        let groundmove_fact = Vec2::new(groundmove_fact_lon, groundmove_fact_lat);
        let rotation_fact = time.delta_seconds() * 20.0; // delta time * degrees per second = delta degrees

        let dir = view.direction.to_radians();
        let forward = GeoDir::forward(dir);
        let right = GeoDir::right(dir);
        let mut velocity = GeoDir::ZERO;

        let moved = keys.get_pressed().len() > 0;
        for key in keys.get_pressed() {
            // match key does not work with struct key_bindings
            let key = *key;
            //
            // forward/backward
            if key == key_bindings.move_forward || key == key_bindings.move_forward2 {
                velocity += forward;
            } else if key == key_bindings.move_backward || key == key_bindings.move_backward2 {
                velocity -= forward;
            //
            // sidewise
            } else if key == key_bindings.move_right {
                velocity += right;
            } else if key == key_bindings.move_left {
                velocity -= right;
            //
            // elevate
            } else if key == key_bindings.move_ascend || key == key_bindings.move_ascend2 {
                view.elevation *= elevation_fakt;
            } else if key == key_bindings.move_descend || key == key_bindings.move_descend2 {
                view.elevation /= elevation_fakt;
            //
            // rotate
            } else if key == key_bindings.rotate_right || key == key_bindings.rotate_right2 {
                view.direction -= rotation_fact;
            } else if key == key_bindings.rotate_left || key == key_bindings.rotate_left2 {
                view.direction += rotation_fact;
            } else if key == key_bindings.rotate_up {
                view.up_view += rotation_fact;
            } else if key == key_bindings.rotate_down {
                view.up_view -= rotation_fact;
            //
            // zoom
            } else if key == key_bindings.zoom_out || key == key_bindings.zoom_out2 {
                view.distance *= elevation_fakt;
            } else if key == key_bindings.zoom_in {
                view.distance /= elevation_fakt;
            }
        }

        if moved {
            view.geo_coord.add_move(velocity * groundmove_fact);

            view.limit();
            let galactic_transform = view.to_galactic_transform(true);
            player.set_pos(galactic_transform);
        }
    } else {
        warn!("Primary window not found for `player_keys`!");
    }
}

/// Handles moving around if 1st key is pressed, looking around if 2nd key is pressed
fn player_mouse(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_motion: Res<Events<MouseMotion>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut input_state: ResMut<InputState>,
    mut control_values: ResMut<ControlValues>,
    mut player: PlayerQuery, // The PlayerQuery includes the query
) {
    if let Ok(window) = primary_window.get_single() {
        let speed = control_values.speed;
        let sensitivity = control_values.sensitivity;
        let view = &mut control_values.view;
        let mut moved = false;
        //for mut _transform in query.iter_mut()
        {
            for ev in scroll_events.read() {
                moved = true;
                let factor = 1. + ev.x / 1000.0;
                view.distance /= factor;
            }

            for ev in input_state.reader_motion.read(&mouse_motion) {
                let mut yaw = 0.0;
                let mut pitch = 0.0;

                let dir = view.direction.to_radians();
                let forward = GeoDir::forward(dir);
                let right = GeoDir::right(dir);

                //t (left, forward) = dir.sin_cos();
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                pitch -= (sensitivity * ev.delta.y * window_scale).to_radians();
                yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();

                if mouse_input.pressed(MouseButton::Right) {
                    moved = true;
                    view.up_view += pitch * 50.; // todo: F4 needs more senivity ???
                    view.direction += yaw * 50.;
                }

                if mouse_input.pressed(MouseButton::Left) {
                    moved = true;
                    let groundmove_fact_lat = speed / 500000.0;
                    let groundmove_fact_lon =
                        groundmove_fact_lat / view.geo_coord.lat.to_radians().sin();
                    let groundmove_fact = Vec2::new(groundmove_fact_lon, groundmove_fact_lat);

                    let velocity = forward * -pitch + right * yaw;
                    view.geo_coord.add_move(velocity * groundmove_fact);
                }
            }
        }

        if moved {
            view.limit();
            // Todo: Crossing a pole by up_view makes the rotation very low and stucking.
            let galactic_transform = view.to_galactic_transform(true);
            player.set_pos(galactic_transform);
        }
    } else {
        warn!("Primary window not found for `player_mouse`!");
    }
}

fn setup(mut control_values: ResMut<ControlValues>, starting_values: Res<crate::StartingValues>) {
    // set up accroding to lat/lon relative to Earth center
    control_values.speed = 100.0;
    control_values.view = starting_values.view;
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .init_resource::<KeyBindings>()
            .add_systems(Update, player_keys)
            .add_systems(Update, player_mouse);
    }
}
