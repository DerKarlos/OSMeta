// main_o2w1.rs = main_osm1.rs

mod fly_control;
mod o2w;

use bevy::prelude::*;
use o2w::*;  //use rendf::*;
use fly_control::{FlyCam, MovementSettings, NoCameraPlayerPlugin };  // PlayerPlugin
//e bevy::render::render_resource::SamplerDescriptor;
//e bevy::render::texture::ImageSettings;





fn main() {

    App::new()

        .add_plugins(
            DefaultPlugins

                // WINDOW  WINDOW  WINDOW  WINDOW 
                .set( WindowPlugin {
                    window: WindowDescriptor {
                        title: "OSMeta - OpenStreetMap Metaverse ;-)".to_string(),
                        //width: 500.,
                        //height: 300.,
                        //present_mode: PresentMode::AutoVsync,
                        //always_on_top: true,
                        //????  present_mode: bevy::window::PresentMode::Fifo, // Not that usefull???: Immediate: 16-18, Mailbox:14.5, Fifo:14.x
                        ..default()
                    }, // WindowDescriptor
                    ..default()
                }) // set WindowPlugin
            
                // IMAGE REPEAT  IMAGE REPEAT  IMAGE REPEAT
                .set(ImagePlugin {
                    default_sampler: wgpu::SamplerDescriptor {
                        address_mode_u: wgpu::AddressMode::Repeat,
                        address_mode_v: wgpu::AddressMode::Repeat,
                        address_mode_w: wgpu::AddressMode::Repeat,
                        ..default()
                    }, // SamplerDescriptor
                    ..default()
                }) // set ImagePlugin

            )

    //  .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64( 1.0 / 20.0, )))  // no different
    //  .with_run_criteria(FixedTimestep::step( (1.0/60.0) as f64))
        .insert_resource(Msaa { samples: 4 })


        //.add_plugins(DefaultPlugins)
        // Show Framerate in Console
        // .add_plugin(LogDiagnosticsPlugin::default())
    //  .add_plugin(FrameTimeDiagnosticsPlugin::default())

        .add_startup_system(setup)

        // https://github.com/sburris0/bevy_flycam  ##  https://github.com/BlackPhlox/bevy_config_cam
        // NoCameraPlayerPlugin as we provide the camera
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015, //  mouse sensitivity, default: 0.00012
            speed: 30.0, // player movement speed, default: 12.0
        })
        //  .add_system(ui_system)
    //  .add_system(fixup_images)
        .run();

        info!("Move camera around by using WASD for lateral movement");
        info!("Use Left Shift and Spacebar for vertical movement");
        info!("Use the mouse to look around");
        info!("Press Esc to hide or show the mouse cursor");
}

//#[derive(Component)]
//struct Movable;

#[derive(Component)]
struct StatsText;



/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // UI with FPS
    //commands.spawn_bundle(UiCameraBundle::default());
    //commands.spawn_bundle(create_ui(&asset_server)).insert(StatsText);

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
    let camera = Camera3dBundle { // Agropolis auf Straße
        transform: Transform::from_xyz(  -1450.028,      4.807,     -0758.637    )
            .looking_at(Vec3::new(-1450.028-0.0, 4.807+0.0, -0758.637-1.0), Vec3::Y),
        ..Default::default()             //          -1.0          +7.8             -1.0
    };
    // add plugin
    commands.spawn(camera).insert(FlyCam);

    /*
    //// camera ////
    let x = 100.;
    let camera = PerspectiveCameraBundle {
        // pub transform/global_transform:: Transform,
        // pub translation: Vec3,
        //[-1450.028, 4.807, -0758.637],        [00000.000, 1.719,  0000.000], // 6: Agropolis auf Straße
        transform: Transform::from_xyz(-1450.028    , 4.807+8.0, -0758.637  )
                 .looking_at(Vec3::new(-1450.028-1.0, 4.807+7.8, -0758.637+1.0), Vec3::Y),
        //ansform: Transform::from_xyz(-8.0*x   ,10.0*x,    20.0*x).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn_bundle(camera);
    */

    // OpenStreetMap !!!
    let _osm2world = OSM2World::new(
        &mut commands,
        &mut meshes,
        &mut materials,
        &asset_server,
        Vec3::new(0.0, 30.0, 0.0), // camera.transform.translation.clone(),    ::ZERO
    );



}
