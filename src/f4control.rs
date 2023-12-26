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
All controls will have the resource type control later (now FlyCam)
Maximal one control/plurgin/systems should run (may be none)

What about the Player? Is it for FlyCam or for all controls

 */

use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::geoview::GeoView;
use crate::GalacticGrid;
use big_space::{FloatingOrigin, FloatingOriginSettings};

use crate::player::Player;

pub mod prelude {
    pub use crate::*;
}

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    _reader_motion: ManualEventReader<MouseMotion>,
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementValues {
    pub sensitivity: f32,
    pub speed: f32,
    pub up: Vec3,
    pub view: GeoView,
}

impl Default for MovementValues {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
            up: Vec3::Y,
            view: GeoView::new(),
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

#[derive(Component)]
pub struct F4Control;

/*
thread 'main' panicked at /Users/karlos/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.12.1/src/system/system_param.rs:225:5:
error[B0001]:
Query<(&bevy_flycam::FlyCam, &mut bevy_transform::components::transform::Transform), ()>
in system osmeta::f4control::player_move accesses component(s) bevy_transform::components::transform::Transform
in a way that conflicts with a previous system parameter.
Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `ParamSet`.


thread 'main' panicked at /Users/karlos/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_ecs-0.12.1/src/system/system_param.rs:225:5:
error[B0001]:
Query<big_space::world_query::GridTransform<i64>, (bevy_ecs::query::filter::With<bevy_flycam::FlyCam>, bevy_ecs::query::filter::Without<osmeta::OpenXRTrackingRoot>,
bevy_ecs::query::filter::Without<osmeta::Compass>)>
in system osmeta::f4control::player_move accesses component(s)
bevy_transform::components::transform::Transform
in a way that conflicts with a previous system parameter.
Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `ParamSet`.

*/

/// Handles keyboard input and movement
fn player_move(
    keys: Res<Input<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    space: Res<FloatingOriginSettings>,
    mut movement_values: ResMut<MovementValues>,
    mut player: Player,
) {
    if let Ok(_window) = primary_window.get_single() {
        let speed = (1. * (movement_values.view.elevation - 300.0)).max(100.0);
        movement_values.speed = speed;

        let view = &mut movement_values.view;
        let elevation_fakt = 1. + time.delta_seconds() / 1.0;
        let groundmove_fact = speed * time.delta_seconds() / 100000.0;

        for key in keys.get_pressed() {
            // match key does not work with struct key_bindings
            let key = *key;
            if key == key_bindings.move_forward {
                view.geo_coord.lat += groundmove_fact;
            } else if key == key_bindings.move_backward {
                view.geo_coord.lat -= groundmove_fact;
            } else if key == key_bindings.move_left {
                view.geo_coord.lon -= groundmove_fact;
            } else if key == key_bindings.move_right {
                view.geo_coord.lon += groundmove_fact;
            } else if key == key_bindings.move_ascend {
                view.elevation *= elevation_fakt;
            } else if key == key_bindings.move_descend {
                view.elevation /= elevation_fakt;
            }
        }
        view.set_camera_view(&space, &mut player);
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

/// Handles looking around if cursor is locked
fn _player_look(
    settings: Res<MovementValues>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            for ev in state._reader_motion.read(&motion) {
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

fn setup(
    mut commands: Commands,
    mut movement_values: ResMut<MovementValues>,
    starting_values: Res<crate::StartingValues>,
    space: Res<FloatingOriginSettings>,
) {
    let (grid, _): (GalacticGrid, _) =
        space.translation_to_grid(starting_values.planetary_position);

    let mut camera = commands.spawn((
        Camera3dBundle { ..default() },
        InheritedVisibility::default(),
        FlyCam,
        F4Control,
        grid,
    ));
    camera.insert(FloatingOrigin);

    // set up accroding to lat/lon relative to Earth center
    movement_values.up = starting_values.planetary_position.normalize().as_vec3();
    movement_values.speed = 100.0;
    movement_values.view = starting_values.start_view;
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MovementValues>()
            .add_systems(Startup, setup)
            .init_resource::<KeyBindings>()
            .add_systems(Update, player_move)
            //.init_resource::<InputState>()
            //.add_systems(Update, player_look)
            ;
    }
}
