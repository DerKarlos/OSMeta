// fly_control.rs  fly_camera.rs

mod viewer; // Window/Frame with the ???    // In javascript: canvas view 
mod osmscene;
mod cameraview;
mod geopos;
mod geoview;
mod utils;

use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use geoview::*;
use geopos::*;

//mod o2w;
// use super::o2w::*;

use viewer::*;

// pub(crate) mod viewer;     // canvas handler


/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
pub struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
    viewer: Viewer,
}

/// Mouse sensitivity and movement speed

#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

impl MovementSettings {
    fn calc_speed(&mut self, value: f32) {
        //println!("yyyy: {:?}",value);
        let mut speed = value.abs() * 1.0;  //1.0 parameter???
        if speed < 0.01 {speed = 0.01}
        self.speed = speed;
    }
}


/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct FlyCam;

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_grab_mode() {
        CursorGrabMode::None => {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);
        }
        _ => {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}

/// Grabs the cursor when game first starts
fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

/*
/// Spawns the `Camera3dBundle` to be controlled
fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
    ));
}
*/


/// Handles keyboard input and movement
fn player_move( // runs cycvlically! Only because of time ???
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
  //mut viewer: ResMut<Viewer>,
    mut settings: ResMut<MovementSettings>,
    mut query:  Query<&mut Transform, With<FlyCam>>,
) {
    if let Some(window) = windows.get_primary() {

        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let mut position = None;
            let mut rotation =Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(   local_z.x, 0.,    local_z.z);
            let right   =  Vec3::new(local_z.z, 0., -local_z.x);

            let delta_state = state.as_mut();

            for key in keys.get_just_pressed() {

        //cument.cookie = "OSM2World_GeoView_Digit6=48.56690610000000 13.448879100000000 1.6 121 -10 65.0 23;samesite=strict"; // 6 windows
        //                                          lat               lon                hi  dir vie rad  fov

                        let gv = GeoView{  // 6 windows
                            geo_pos: GeoPos{
                                     lat: 48.5669061,
                                     lon: 13.4488791,
                                 },
                            height:  1.6,
                            dir:   121.0, // alpha
                            view:  -10.0, // beta
                            radius: 65.0,
                            fov:    23.0,
                        };

                        match key {
                            KeyCode::Key1 => { delta_state.viewer.set_geo_view(None,     &mut transform); }, // default: next to university
                            KeyCode::Key6 => { delta_state.viewer.set_geo_view(Some(gv), &mut transform);  // windows (of Acropolos?)
                                    delta_state.yaw   = gv.dir.to_radians();
                                    delta_state.pitch = gv.view.to_radians();
                                },
                            _ => (),

/*
        //                                          lat               lon                hi  dir vie rad  fov
        document.cookie = "OSM2World_GeoView_Digit1=48.56583701046516 13.453166231638868 1.6 -32 -12 547. 23;samesite=strict"; // 1 default uni
        document.cookie = "OSM2World_GeoView_Digit2=48.57203151991611 13.456722845073253 1.6 -63 -6. 77.1 23;samesite=strict"; // 2 center
        document.cookie = "OSM2World_GeoView_Digit3=48.56439632980203 13.430698929491165 1.6 -68 -0. 35.7 23;samesite=strict"; // 3 cars
        document.cookie = "OSM2World_GeoView_Digit4=48.56725450000000 13.453000000000000 1.6 -30 -10 375. 23;samesite=strict"; // 4 F4
        document.cookie = "OSM2World_GeoView_Digit5=48.52713530972139 13.415167708626935 1.6 182 -25 1377 23;samesite=strict"; // 5 wood
        document.cookie = "OSM2World_GeoView_Digit6=48.56690610000000 13.448879100000000 1.6 121 -10 65.0 23;samesite=strict"; // 6 windows
        document.cookie = "OSM2World_GeoView_Digit7=48.57739134741593 13.423744066161506 1.6 -44 -21 116. 23;samesite=strict"; // 7 solar
        document.cookie = "OSM2World_GeoView_Digit8=48.59397502885877 13.389369883128929 97. 120 -10 0.50 23;samesite=strict"; // 8 crane
        document.cookie = "OSM2World_GeoView_Digit9=48.59158409512967 12.701907495407475 1.6 63. -3. 705. 23;samesite=strict"; // 9 wind
*/


                        }
            }

            for key in keys.get_pressed() { // get me all the pressed keys for a loop
                //println!("key: {:?}",key);

                match window.cursor_grab_mode() {
                    CursorGrabMode::None => (), // no grap, no acton
                    _ => {

                        match key {
                            KeyCode::W | KeyCode::Up    => velocity += forward,
                            KeyCode::S | KeyCode::Down  => velocity -= forward,
                            KeyCode::A | KeyCode::Left  => velocity -=   right,
                            KeyCode::D | KeyCode::Right => velocity +=   right,
                            KeyCode::Space  | KeyCode::Plus  | KeyCode::Equals => { settings.calc_speed(transform.translation.y); velocity += Vec3::Y }, // Equals is # on the German keyboard
                            KeyCode::LShift | KeyCode::Minus                   => { settings.calc_speed(transform.translation.y); velocity -= Vec3::Y },

                            KeyCode::Key0 => { // center of pbf tile
                                position = Some(Vec3::new(-0000.000, 010.0,  050.000 ));
                                rotation =      Vec3::new(-0000.000, 000.0, -000.000 );
                            },


                            KeyCode::Key5 => { // acropolis??
                                position = Some(Vec3::new(-1450.028, 001.6, -758.637 ));
                                rotation =      Vec3::new(-0010.000, 121.0, -000.000 );
                            },

                            _ => (),
                        }
                    },
                }
            }


/*
        document.cookie = "OSM2World_GeoView_Digit1=48.56583701046516 13.453166231638868 1.6 -32 -12 547. 23;samesite=strict"; // 1 default uni?
        document.cookie = "OSM2World_GeoView_Digit2=48.57203151991611 13.456722845073253 1.6 -63 -6. 77.1 23;samesite=strict"; // 2 center
        document.cookie = "OSM2World_GeoView_Digit3=48.56439632980203 13.430698929491165 1.6 -68 -0. 35.7 23;samesite=strict"; // 3 cars
        document.cookie = "OSM2World_GeoView_Digit4=48.56725450000000 13.453000000000000 1.6 -30 -10 375. 23;samesite=strict"; // 4 F4
        document.cookie = "OSM2World_GeoView_Digit5=48.52713530972139 13.415167708626935 1.6 182 -25 1377 23;samesite=strict"; // 5 wood

        document.cookie = "OSM2World_GeoView_Digit6=48.56690610000000 13.448879100000000 1.6 121 -10 65.0 23;samesite=strict"; // 6 windows

        document.cookie = "OSM2World_GeoView_Digit7=48.57739134741593 13.423744066161506 1.6 -44 -21 116. 23;samesite=strict"; // 7 solar
        document.cookie = "OSM2World_GeoView_Digit8=48.59397502885877 13.389369883128929 97. 120 -10 0.50 23;samesite=strict"; // 8 crane
        document.cookie = "OSM2World_GeoView_Digit9=48.59158409512967 12.701907495407475 1.6 63. -3. 705. 23;samesite=strict"; // 9 wind
*/


            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * settings.speed;

            if let Some(new_position) = position {
                //let mut delta_state = state.as_mut();
                delta_state.yaw   = (rotation.y).to_radians();  // -90?  up/down
                delta_state.pitch = (rotation.x).to_radians();  // +90?  right/left
                transform.translation = new_position;
                // Order is important to prevent unintended roll
                transform.rotation
                    = Quat::from_axis_angle(Vec3::Y, delta_state.yaw  )  // beta?   mouse y: up/down
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch); // alpha?  mouse x: right/left
            //  transform.with_scale(Vec3::new(1.,1.,1.));



            }
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

/// Handles looking around if cursor is locked
fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                match window.cursor_grab_mode() {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale   = window.height().min(window.width());
                        delta_state.pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians(); // mouse y: up/down
                        delta_state.yaw   -= (settings.sensitivity * ev.delta.x * window_scale).to_radians(); // mouse x: right/left
                    }
                }

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation
                    = Quat::from_axis_angle(Vec3::Y, delta_state.yaw  )  // beta?   mouse y: up/down
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch); // alpha?  mouse x: right/left
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

/*
/// Contains everything needed to add first-person fly camera behavior to your game
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(setup_player)
            .add_startup_system(initial_grab_cursor)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}
*/

/// Same as [`PlayerPlugin`] but does not spawn a camera
pub struct NoCameraPlayerPlugin;
impl Plugin for NoCameraPlayerPlugin {
    fn build(&self, app: &mut App) {
        app. init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(initial_grab_cursor)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab)
            ;
    }
}