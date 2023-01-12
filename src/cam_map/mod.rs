// fly_control.rs  fly_camera.rs   cam_map/mod.rs

mod viewer; // Window/Frame with the ???    // In javascript: canvas view 
mod osmscene;
mod cameraview;
mod geopos;
mod geoview;
mod utils;

mod o2w;
pub use o2w::*;


use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

use geoview::*;
use geopos::*;

use std::collections::HashMap;


//mod o2w;
// use super::o2w::*;
use crate::cam_map::utils::TileName;

use viewer::*;

// pub(crate) mod viewer;     // canvas handler



/// Keeps track of mouse motion events  (not any more:, pitch, and yaw)
#[derive(Resource)]
pub struct CamMapState {
    reader_motion: ManualEventReader<MouseMotion>,
    viewer: Viewer, // The viewer instance(es) for this camera(s)   -- jus one now, but my become able to handle multible windows
    cookies: HashMap<String,String>,  // https://www.sitepoint.com/rust-global-variables/#singlethreadedglobalswithruntimeinitialization
    shift: bool,
    pub init: bool,
}

impl Default for CamMapState {
    fn default() -> Self {

        // let mut viewer = Viewer::default();
        // viewer.set_view(None, &transform);

        let mut cookies = HashMap::default();
        cookies.insert("1".to_string(), "48.56583701046516 13.453166231638868 1.6 -32 -12 547. 23".to_string()); // 1 default uni
        cookies.insert("2".to_string(), "48.57203151991611 13.456722845073253 1.6 -63 -6. 77.1 23".to_string()); // 2 center
        cookies.insert("4".to_string(), "48.56725450000000 13.453000000000000 1.6 -30 -10 375. 23".to_string()); // 4 F4
        cookies.insert("6".to_string(), "48.56690610000000 13.448879100000000 1.6 121 -10 65.0 23".to_string()); // 6 windows
        //          .                         lat               lon                hi  dir vie rad  fov
        //cookie =  "OSM2World_GeoView_Digit1=48.56583701046516 13.453166231638868 1.6 -32 -12 547. 23;samesite=strict"; // 1 default uni
        //cookie =  "OSM2World_GeoView_Digit2=48.57203151991611 13.456722845073253 1.6 -63 -6. 77.1 23;samesite=strict"; // 2 center
        //cookie =  "OSM2World_GeoView_Digit3=48.56439632980203 13.430698929491165 1.6 -68 -0. 35.7 23;samesite=strict"; // 3 cars
        //cookie =  "OSM2World_GeoView_Digit4=48.56725450000000 13.453000000000000 1.6 -30 -10 375. 23;samesite=strict"; // 4 F4
        //cookie =  "OSM2World_GeoView_Digit5=48.52713530972139 13.415167708626935 1.6 182 -25 1377 23;samesite=strict"; // 5 wood
        //cookie =  "OSM2World_GeoView_Digit6=48.56690610000000 13.448879100000000 1.6 121 -10 65.0 23;samesite=strict"; // 6 windows
        //cookie =  "OSM2World_GeoView_Digit7=48.57739134741593 13.423744066161506 1.6 -44 -21 116. 23;samesite=strict"; // 7 solar
        //cookie =  "OSM2World_GeoView_Digit8=48.59397502885877 13.389369883128929 97. 120 -10 0.50 23;samesite=strict"; // 8 crane
        //cookie =  "OSM2World_GeoView_Digit9=48.59158409512967 12.701907495407475 1.6 63. -3. 705. 23;samesite=strict"; // 9 wind
        //                                    lat               lon                hi  dir vie rad  fov


        Self{
            reader_motion: ManualEventReader::default(),
            viewer: Viewer::default(),
            cookies,
            shift:  false,
            init:   false,
        }

        //  settings.viewer.set_view(None, &mut Transform::default() ); // set view to default (transform is not used?)

    }
}

/// Mouse sensitivity and movement speed

#[derive(Resource)]
pub struct CamMapSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub rotate: f32,
}

impl Default for CamMapSettings {
    fn default() -> Self {

        Self {
            sensitivity: 0.00012,               // radiants per screen pixel
            speed:      12.,                    // meter per secound
            rotate:    (30.0_f32).to_radians(), // degrees.to_radiants per secound
        }
        
    }
}

impl CamMapSettings {
    /// The more heigh the camear the more speed
    fn calc_speed(&mut self, height: f32) {
        let mut new_speed = height.abs() * 1.5;  //x.0 parameter???
        if new_speed < 0.01 {new_speed = 0.01}
        self.speed = new_speed;
        // println!("yyyy: {:?} {:?}",self.speed,height);
    }
}


/// A marker component used in queries when you want flycams and not other cameras
#[derive(Component)]
pub struct CamMap;

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



fn set_view(id: String, input_state: &mut CamMapState, transform: &mut Transform) {
    input_state.viewer.get_geo_view_at_camera(transform).store("last".to_string(), &mut input_state.cookies);
    let gv = GeoView::restore(id, &mut input_state.cookies);
    input_state.viewer.set_view(gv, transform);
}

fn store_view(id: String, input_state: &mut CamMapState, transform: &mut Transform) {
    input_state.viewer.get_geo_view_at_camera(&transform).store(id, &mut input_state.cookies);
}



////////////////// andles keyboard input and (cyclically) movement ()Move and Rotate) ////////////
fn camera_move( // runs cycvlically! Only because of time ???
        keys:     Res<Input<KeyCode>>,
    //  scans:    Res<Input<ScanCode>>,
        time:     Res<Time>,
        windows:  Res<Windows>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes:   ResMut<Assets<Mesh>>,
    mut materials:ResMut<Assets<StandardMaterial>>,
    mut state:    ResMut<CamMapState>,
    mut settings: ResMut<CamMapSettings>,
    mut query:    Query<&mut Transform, With<CamMap>>,
) {
    if let Some(window) = windows.get_primary() {

        for mut transform in query.iter_mut() {
            let (mut yaw,mut pitch,_z) = transform.rotation.to_euler(EulerRot::YXZ); // read actual camera angles to add rotations and set the new angles.
            let mut rotated = false;
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(   local_z.x, local_z.y*1.,    local_z.z);
            let right   =  Vec3::new(local_z.z, 0., -local_z.x);

            let input_state = state.as_mut();

            if !input_state.init {
                input_state.init = true;
                input_state.viewer.set_view(None, &mut transform); // default: next to university
            }

            if let Some(pbf_tile) = &input_state.viewer.load_pbf {
                let _osm2world = OSM2World::new(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &asset_server,
                    pbf_tile,
                    Vec3::new(0.0, 30.0, 0.0), // camera.transform.translation.clone(),    ::ZERO
                );
                input_state.viewer.load_pbf = None;
            }

            // test only for change to scan
            //for scan in scans.get_just_pressed() {
            //    println!("shift: {} scan: {:?}", input_state.shift, scan);
            //} // get_just_pressed

            ////////// newly pressed or released key /////////

            for key in keys.get_just_pressed() {

                let g0 = GeoView{  // 6 windows
                    geo_pos: GeoPos{
                             lat: 48.560251-0.0003, // 48.574795  48.545708
                             lon: 13.469238,        // 13.447266  13.491211
                         },
                    height:  1.6,
                    dir:     0.0,
                    view:    0.0,
                    radius: 99.9,
                    fov:    22.2,

                };

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

/*  ERROR! may be in winit
    key:        RShift
    key: Key0   Equals
    key: Key9   Key0
    key: Key8   Key9
    key: Key7   Slash
    key: Key6   Key7
    key: Key5   =
    key: Key4   =
    key: Key3   =
    key: Key2   Apostrophe
    key: Key1   =
*/

                //println!("shift: {} key: {:?}", input_state.shift, key);

                if input_state.shift {
                    match key {
                        KeyCode::Key1       => store_view("1".to_string(), input_state, &mut transform), // 1 default
                        KeyCode::Apostrophe => store_view("2".to_string(), input_state, &mut transform), // 2 center
                        KeyCode::Key4       => store_view("4".to_string(), input_state, &mut transform), // 4 F4
                        KeyCode::Key7       => store_view("6".to_string(), input_state, &mut transform), // 6 windows of Acropolos
                        _ => (),
                    }
                } else {
                    match key {

                        KeyCode::RShift | KeyCode::LShift => input_state.shift = true,

                        // test only:
                        KeyCode::Key9 => { input_state.viewer.set_view(None,     &mut transform); }, // default: next to university
                        KeyCode::Key0 => { input_state.viewer.set_view(Some(g0), &mut transform); }, // center of first tile, test only
                        KeyCode::Key5 => { input_state.viewer.set_view(Some(gv), &mut transform); }, // windows (of Acropolos?)

                        // jump to stored view:
                        KeyCode::Key1 => set_view("1".to_string(), input_state, &mut transform), // 1 default
                        KeyCode::Key2 => set_view("2".to_string(), input_state, &mut transform), // 2 center
                        KeyCode::Key4 => set_view("4".to_string(), input_state, &mut transform), // 4 F4
                        KeyCode::Key6 => set_view("6".to_string(), input_state, &mut transform), // 6 windows of Acropolos

                        KeyCode::Key7 => { // test: store in 6
                            input_state.viewer.get_geo_view_at_camera(&transform).store("6".to_string(), &mut input_state.cookies);
                        },

                        KeyCode::H    => { // test: show actual position as brower URL / abiut the cookie string
                            let gv = input_state.viewer.get_geo_view_at_camera(&transform);
                            println!(
                                "URL:: x/z: {}/{} yaw/pitch:{}/{} lat/lon::{}/{}",
                                transform.translation.x,
                                transform.translation.z,
                                yaw.to_degrees(),
                                pitch.to_degrees(),
                                gv.geo_pos.lat,
                                gv.geo_pos.lon,
                            );
                        },

                        KeyCode::T    => { // Test someting
                            let h = transform.translation.y;
                            println!("test: {:#?} {:#?}", settings.speed,h*h*h*0.2);
                        }

                        _ => (),

                    } // match key
                } // no shift

            } // get_just_pressed


            for key in keys.get_just_released() {
                match key {
                    KeyCode::RShift | KeyCode::LShift => input_state.shift = false,
                    _ => (),
                }
            }


            ////////// continously pressed key /////////

            for key in keys.get_pressed() { // get me all the pressed keys for a loop
                //println!("key: {:?}",key);

                match window.cursor_grab_mode() {
                    CursorGrabMode::None => (), // no grap, no acton
                    _ => {                      // grap: actions:

                        match key {

                            // move
                            KeyCode::W | KeyCode::Up    => velocity += forward,
                            KeyCode::S | KeyCode::Down  => velocity -= forward,
                            KeyCode::A | KeyCode::Left  => velocity -=   right,
                            KeyCode::D | KeyCode::Right => velocity +=   right,
                            // move up/down. calc speed dependent of the height
                            KeyCode::Space  | KeyCode::Plus  | KeyCode::Equals => { velocity += Vec3::Y; }, //  elevate
                            KeyCode::LShift | KeyCode::Minus                   => { velocity -= Vec3::Y; }, // delevate
                            // Key3 is # on the German keyboard ??? Needed for store! use + and -

                            // rotate                                                                                            // ROTATE:
                            KeyCode::Q                  => { yaw   += time.delta_seconds() * settings.rotate; rotated = true; }, // left
                            KeyCode::E                  => { yaw   -= time.delta_seconds() * settings.rotate; rotated = true; }, // right
                            KeyCode::R                  => { pitch += time.delta_seconds() * settings.rotate; rotated = true; }, // up
                            KeyCode::F                  => { pitch -= time.delta_seconds() * settings.rotate; rotated = true; }, // down

                            _ => (),
                        }
                    },
                }
            }

            settings.calc_speed(transform.translation.y);
            velocity = velocity.normalize_or_zero();
            transform.translation += velocity * time.delta_seconds() * settings.speed;
            //if transform.translation.y < 0.3 {transform.translation.y = 0.3};

            if rotated {
                pitch = pitch.clamp(-1.54, 88_f32.to_radians() ); // todo:  degrees.to_radiants()
                // Order is important to prevent unintended roll
                transform.rotation
                    = Quat::from_axis_angle(Vec3::Y, yaw  )  // beta?   mouse y: up/down
                    * Quat::from_axis_angle(Vec3::X, pitch); // alpha?  mouse x: right/left
            }

        }
    } else {
        warn!("Primary window not found for `camera_move`!");
    }
}


////////////////// Handles looking around if cursor is locked //////////////
fn camera_look(
        settings: Res<CamMapSettings>,
        windows:  Res<Windows>,
    mut state:    ResMut<CamMapState>,
        motion:   Res<Events<MouseMotion>>,
    mut query:    Query<&mut Transform, With<CamMap>>,
) {
    if let Some(window) = windows.get_primary() {
        let input_state = state.as_mut();

        for mut transform in query.iter_mut() {
            let (mut yaw,mut pitch,_z) = transform.rotation.to_euler(EulerRot::YXZ);
            for ev in input_state.reader_motion.iter(&motion) {
                match window.cursor_grab_mode() {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale   = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians(); // mouse y: rotate up/down
                        yaw   -= (settings.sensitivity * ev.delta.x * window_scale).to_radians(); // mouse x: rotate right/left
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation
                    = Quat::from_axis_angle(Vec3::Y, yaw  )  // beta?   mouse y: rotate up/down
                    * Quat::from_axis_angle(Vec3::X, pitch); // alpha?  mouse x: rotate right/left
            }

        }
    } else {
        warn!("Primary window not found for `camera_look`!");
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
pub struct CameraPlugin;
impl Plugin for CameraqPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CamMapState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(setup_camera)
            .add_startup_system(initial_grab_cursor)
            .add_system(camera_move)
            .add_system(camera_look)
            .add_system(cursor_grab);
    }
}
*/

/// Same as [`CamearPlugin`] but does not spawn a camera
pub struct NoCamMapPlugin;
impl Plugin for NoCamMapPlugin {
    fn build(&self, app: &mut App) {
        app. init_resource::<CamMapState>()
            .init_resource::<CamMapSettings>()
            .add_startup_system(initial_grab_cursor)
            .add_system(camera_move)
            .add_system(camera_look)
            .add_system(cursor_grab)
            ;
        
    }
}