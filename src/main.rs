// main_o2w1.rs = main_osm1.rs

mod cam_map; // mod fly_control;
////mod o2w;

use bevy::prelude::*;
//e bevy::render::render_resource::SamplerDescriptor;
//e bevy::render::texture::ImageSettings;

// use o2w::*;
use cam_map::{CamMap, CamMapSettings, NoCamMapPlugin };  // CameraPlugin  - fly_control::

//d ui;
//e ui::*;


pub const TEST: bool = true;
pub const FPS:  bool = false;


fn main() {

    let window_plugin = WindowPlugin {
        window: WindowDescriptor {
            title: "OSMeta - OpenStreetMap Metaverse ;-)".to_string(),
            //width: 500.,
            //height: 300.,
            //present_mode: PresentMode::AutoVsync,
            //always_on_top: true,
            //????  present_mode: bevy::window::PresentMode::Fifo, // Not that usefull???: Immediate: 16-18, Mailbox:14.5, Fifo:14.x
            ..default()
        },
        ..default()
    };

    let image_plugin = ImagePlugin {
            default_sampler: wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            ..default()
        },
        ..default()
    };


    let mut app = App::new();
    app
        .add_plugins(
            DefaultPlugins
                .set( window_plugin)
                .set( image_plugin)
            ) // add_plugins

    //  .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64( 1.0 / 20.0, )))  // no different
    //  .with_run_criteria(FixedTimestep::step( (1.0/60.0) as f64))
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup)

        // https://github.com/sburris0/bevy_flycam  ##  https://github.com/BlackPhlox/bevy_config_cam
        // NoCameraPlugin as we provide the camera
        .add_plugin(NoCamMapPlugin)
        .insert_resource(CamMapSettings {
            sensitivity: 0.00015,               //  mouse sensitivity, default: 0.00012
            speed:      30.0,                   // camera movement speed, default: 12.0
            rotate:    (30.0_f32).to_radians(), // camera rotate speed  [degrees.to_radiants per secound]
        })
        ;

        if FPS {
            // Show Framerate in Console   
            app
                .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
                .add_system(ui_update_system)
                ;
        }
    
        //  .add_system(fixup_images)

        app.run();

        info!("Move camera around by using WASD for lateral movement");
        info!("Use Left Shift and Spacebar for vertical movement");
        info!("Use the mouse to look around");
        info!("Press Esc to hide or show the mouse cursor");
}

//#[derive(Component)]
//struct Movable;

// A unit struct to help identify the FPS UI component, since there may be many Text components
// if FPS?
#[derive(Component)]
struct FpsText;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    //// light ////8
    // Shadows do not work correct on my Macbook Air native, but in the browser it is ok.
    let mut _shadows = true;
    #[cfg(not(target_arch = "wasm32"))]
    { _shadows = false; }
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 0.01, // 1500?  ?????????????
            shadows_enabled: _shadows,
            ..default()
        },
        transform: Transform::from_xyz(4000.0, 8000.0, 4000.0),
        ..default()
    });

    //// camera ////
    let camera = 
    if !TEST {
        Camera3dBundle { // Agropolis auf Straße
            transform: Transform::from_xyz(  -1450.028,      4.807,     -0758.637    )
                .looking_at(Vec3::new(-1450.028-0.0, 4.807+0.0, -0758.637-1.0), Vec3::Y)
                .with_scale(Vec3::new(1.,1.,1.))
                ,
            ..Default::default()             //          -1.0          +7.8             -1.0
        }

    } else {
        Camera3dBundle { // almost 0
            transform: Transform::from_xyz(   0.0, 10.0, 50.0)
                .looking_at(Vec3::new(0.0, 00.0, 00.0), Vec3::Y)
                ,
            ..Default::default()             //          -1.0          +7.8             -1.0
        }

    };
    // add plugin
    commands.spawn(camera).insert(CamMap);


    // Create the OSM2World-viewer

    // let mut viewer = commands.spawn( Viewer::new() )
    //  .insert(Viewer)
    //;


    //let mut _viewer = Viewer::new(
    //  camera,
        /* 
        {   // Options: todo? Structure ?
            control: true, // shall the O2W default control be used?
            shadow: 1, // 0: off 1:1024 2:2048 4:4096 8:8192       ??????? water makes mesh invisible BJS? !!!!!!
            water: 1,  // 0: no waves 300ms, no reflection, 1: only skydome 310ms, 2: all shadow caster 420ms
            fpsMin: +5, // +15!! schrauben
            //selected: ObjectSelected, //null, //Function, // = this.defaultObjectSelected.bind(this);
            // xrMode: 0, // -1:off  0:auto (default)  1:on  2:WebAR
            // distanceMax: 1111,
            // viewRings:0,
            //cars: 0.99,
        } */
    //);

    // var geoPos = new OSM2WORLD.GeoPos(48.591941, 12.703934); // wind
    //var geoPos = new OSM2WORLD.GeoPos(48.572044, 13.458089); // passau=default
    //var geoView = new OSM2WORLD.GeoView(geoPos /*default: geoPos, 1.8, -90.8, -15.8, 500.8*/ ); // hejght, compas, up/down, distance
//  viewer.set_view(None); // geoView // finds or creates a new Scene at this place on Earth and sets the camera view


    if !TEST {
        // OpenStreetMap !!!  Viewer instead?
    //    let _osm2world = OSM2World::new(
    //        &mut commands,
    //        &mut meshes,
    //        &mut materials,
    //        &asset_server,
    //        Vec3::new(0.0, 30.0, 0.0), // camera.transform.translation.clone(),    ::ZERO
    //    );
    } else {
        // test cube only
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        });

    }


    if FPS {
        // Text with multiple sections
        commands.spawn((
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 20.0,
                    color: Color::GOLD,
                }),
            ]),
            FpsText,
        ));
    }





}



// if FPS?   MAY BE USE https://doc.rust-lang.org/cargo/reference/features.html
fn ui_update_system(diagnostics: Res<bevy::diagnostic::Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

