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

TODO: "like F4" !
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

pub mod prelude {
    pub use crate::*;
}

use crate::player::{ControlValues, InputState, PlayerQuery};

/// Used in queries when you want f4controls and not other cameras
/// A marker component used in queries when you want f4controls and not other cameras
//use crate::player::Control; // not F4Control  Todo: name it CamControl for the just running control  --

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
            move_forward: KeyCode::W,
            move_forward2: KeyCode::Up, // F4
            move_backward: KeyCode::S,
            move_backward2: KeyCode::Down, // F4
            move_left: KeyCode::A,
            move_right: KeyCode::D,
            move_ascend: KeyCode::T,
            move_ascend2: KeyCode::Backslash, // + on German Mac
            move_descend: KeyCode::G,
            move_descend2: KeyCode::BracketRight, // # on German Mac
            //
            rotate_up: KeyCode::F,
            rotate_down: KeyCode::R,
            rotate_left: KeyCode::Q,
            rotate_left2: KeyCode::Left, // F4
            rotate_right: KeyCode::E,
            rotate_right2: KeyCode::Right, // F4
            zoom_in: KeyCode::H,
            zoom_out: KeyCode::Z,
            zoom_out2: KeyCode::Y, // Z on german Keyboards
        }
    }
}

/// Handles keyboard input and movement
fn player_move(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    time: Res<Time>,
    mut control_values: ResMut<ControlValues>,
    mut player: PlayerQuery,
) {
    if let Ok(_window) = primary_window.get_single() {
        //let speed = (1. * (control_values.view.elevation - 300.0)).max(100.0); // TODO !!!!!!!! real camera height, including distance
        //control_values.speed = speed;

        const SPEED_DEGREE_PER_M: f32 = 1.0 / 200000.0;
        let speed = control_values.speed;
        let view = &mut control_values.view;
        let elevation_fakt = 1. + time.delta_seconds() / 1.0;
        let groundmove_fact_lat = speed * time.delta_seconds() * SPEED_DEGREE_PER_M;
        let groundmove_fact_lon = groundmove_fact_lat / view.geo_coord.lat.to_radians().sin();
        let rotation_fact = time.delta_seconds() * 20.0; // delta time * degrees per second = delta degrees

        let dir = view.direction.to_radians();
        // Todo?:  make a geo_forward/right? Put lat/lon in a vec3 or 2?
        let forward = dir.cos();
        let right = -dir.sin();

        let moved = keys.get_pressed().len() > 0;
        for key in keys.get_pressed() {
            // match key does not work with struct key_bindings
            let key = *key;
            // forward/backward
            if key == key_bindings.move_forward || key == key_bindings.move_forward2 {
                view.geo_coord.lat += forward * groundmove_fact_lat;
                view.geo_coord.lon += right * groundmove_fact_lon;
            } else if key == key_bindings.move_backward || key == key_bindings.move_backward2 {
                view.geo_coord.lat -= forward * groundmove_fact_lat;
                view.geo_coord.lon -= right * groundmove_fact_lon;
            //
            // sidewise
            } else if key == key_bindings.move_right {
                view.geo_coord.lat -= right * groundmove_fact_lat;
                view.geo_coord.lon += forward * groundmove_fact_lon;
            } else if key == key_bindings.move_left {
                view.geo_coord.lat += right * groundmove_fact_lat;
                view.geo_coord.lon -= forward * groundmove_fact_lon;
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
            view.limit();
            let galactic_transform = view.to_galactic_transform(true);
            player.set_pos(galactic_transform);
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

/// Handles moving around if 1st key is pressed, looking around if 2nd key is pressed
fn player_look(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_motion: Res<Events<MouseMotion>>,
    mouse_input: Res<Input<MouseButton>>,
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
                let forward = dir.cos();
                let right = -dir.sin();
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

                    view.geo_coord.lon += forward * yaw * groundmove_fact_lon;
                    view.geo_coord.lat -= forward * pitch * groundmove_fact_lat;

                    view.geo_coord.lat -= right * yaw * groundmove_fact_lat;
                    view.geo_coord.lon -= right * pitch * groundmove_fact_lon;
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
        warn!("Primary window not found for `player_look`!");
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
        app
//            .init_resource::<MovementValues>()
            .add_systems(Startup, setup)
            .init_resource::<InputState>()
            .init_resource::<KeyBindings>()
            .add_systems(Update, player_look)
            .add_systems(Update, player_move) // Toto: ok? move also sets the changes of look
            ;
    }
}
