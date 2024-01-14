/*
https://bevy-cheatbook.github.io/fundamentals/coords.html
https://de.wikipedia.org/wiki/Kugelkoordinaten#Übliche_Konvention
https://en.wikipedia.org/wiki/Spherical_coordinate_system
*/

use bevy::{prelude::*, utils::petgraph::matrix_graph::Zero};
use bevy_panorbit_camera::*; // https://docs.rs/bevy_panorbit_camera/latest/bevy_panorbit_camera/
use globe_rs::{CartesianPoint, GeographicPoint};

use glam::DVec3;

const EARTH_RADIUS: f32 = 1.0;

const LAT: f32 = 0.; // 48.1408;
const LON: f32 = 0.; // 11.5577;
const ELEVATION: f32 = 0.7;

const DIRECTION: f32 = 0.0;
const UP_VIEW: f32 = -90.0;
const DISTANCE: f32 = 0.7;

const SECTORS: usize = 32;

#[derive(Component)]
pub struct CamControl;

#[derive(Resource, Default)]
pub struct CamData {
    pub lat: f32,
    pub lon: f32,
    pub direction: f32,
}

// Trolley Angle Marker
#[derive(Component)]
pub struct Join;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(CamData {
            lat: LAT,
            lon: LON,
            direction: DIRECTION,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, move_join)
        .add_systems(Update, move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Camera-Meshes (body and lense)
    let cube = meshes.add(shape::Cube { size: 1.0 }.try_into().unwrap());

    let camera_box = commands
        .spawn((
            PbrBundle {
                mesh: cube.clone(),
                // box, visualisizing a camera in Bevy coordiantes/rotatons
                transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::new(0.3, 0.5, 0.4)),
                material: materials.add(StandardMaterial {
                    base_color: Color::GRAY,
                    ..default()
                }),
                ..default()
            },
            CamControl,
        ))
        .id();

    let lense = commands
        .spawn((PbrBundle {
            mesh: cube.clone(),
            // place lense in front of the box (-z) a bit up (+y) in Bevy coordinates
            transform: Transform::from_xyz(0.0, 0.15, -0.6).with_scale(Vec3::new(0.25, 0.25, 0.25)),
            ..default()
        },))
        .id();

    commands.entity(camera_box).push_children(&[lense]);

    ////////////////////////////////////////////////////////
    // Earth with equator and greewich meridian and markers
    let sphere = meshes.add(
        shape::UVSphere {
            radius: EARTH_RADIUS,
            sectors: SECTORS,
            stacks: SECTORS / 2,
        }
        .try_into()
        .unwrap(),
    );

    commands.spawn((PbrBundle {
        mesh: sphere,
        material: materials.add(StandardMaterial {
            base_color: Color::BLUE,
            ..default()
        }),
        ..default()
    },));

    let disk = meshes.add(
        shape::Cylinder {
            radius: 1.02,
            height: 0.05,
            resolution: 16,
            segments: 8,
        }
        .try_into()
        .unwrap(),
    );

    // Equator: 0 latitude
    commands.spawn((PbrBundle {
        mesh: disk.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }),
        ..default()
    },));

    // 0 longditure
    commands.spawn(PbrBundle {
        mesh: disk,
        transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // Null-Island
    commands.spawn((PbrBundle {
        mesh: cube.clone(),
        transform: Transform::from_translation(calc_geographic_translation(0., 0., 0., 0., 0.))
            .with_scale(Vec3::new(0.1, 0.1, 0.1)),
        ..default()
    },));

    // Greenwich
    commands.spawn((PbrBundle {
        mesh: cube.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }),
        transform: Transform::from_translation(calc_geographic_translation(51.84, 0., 0., 0., 0.))
            .with_scale(Vec3::new(0.1, 0.1, 0.1)),
        ..default()
    },));

    // Northpole
    commands.spawn((PbrBundle {
        mesh: cube.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        }),
        transform: Transform::from_translation(calc_geographic_translation(90., 0., 0., 0., 0.))
            .with_scale(Vec3::new(0.1, 0.1, 0.1)),
        ..default()
    },));

    // Munich, Germany
    commands.spawn((PbrBundle {
        mesh: cube.clone(),
        material: materials.add(StandardMaterial {
            base_color: Color::ALICE_BLUE,
            ..default()
        }),
        transform: Transform::from_translation(calc_geographic_translation(
            48.1408, 11.5577, 0., 0., 0.,
        ))
        .with_scale(Vec3::new(0.1, 0.1, 0.1)),
        ..default()
    },));

    // Join
    commands.spawn((
        PbrBundle {
            mesh: cube.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                ..default()
            }),
            transform: Transform::from_translation(calc_geographic_translation(0., 0., 0., 0., 0.))
                .with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        },
        Join,
    ));

    ///// Camera & Light /////////////////////////////////////////////////////

    // bevy-camera
    commands.spawn((
        Camera3dBundle {
            //transform: Transform::from_xyz(0., 10., 0.),
            ..default()
        },
        //CamControl,
        PanOrbitCamera {
            //alpha: Some((-90.0_f32).to_radians()),
            //beta: Some((0.0_f32).to_radians()),
            radius: Some(6.0_f32),
            ..default()
        },
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn move_join(
    mut transform_query: Query<&mut Transform, With<Join>>, //
    cam_data: Res<CamData>,
) {
    for mut transform in transform_query.iter_mut() {
        let translation =
            calc_geographic_translation(cam_data.lat, cam_data.lon, ELEVATION, 0., 0.);

        transform.translation = translation;
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
////// Learning rotation, quaternion, transformation /////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn move_camera(
    mut cam_transform_query: Query<&mut Transform, With<CamControl>>, //
    cam_data: ResMut<CamData>,
    _time: Res<Time>,
) {
    for mut transform in cam_transform_query.iter_mut() {
        // todo: how to do it with single() ?

        //cam_data.lon += 2.0 * time.delta_seconds();
        //cam_data.lat += 5.0 * time.delta_seconds();
        //cam_data.direction += 45.0 * time.delta_seconds(); // degrees per second   45 = 8s for one full rotation

        //info!("cam_data.lon: {:?}",cam_data.lon);
        let translation = calc_geographic_translation(
            cam_data.lat,
            cam_data.lon,
            ELEVATION,
            cam_data.direction,
            DISTANCE,
        );
        let up = translation.normalize();
        let west = Vec3::Z.cross(up); // west is a cycle NOT corssing the equator?
        let north = up.cross(west); // north = "crossing" up and west

        /*
        let geo = GeographicPoint::new(
            (cam_data.lon as f64).to_radians(),
            (cam_data.lat as f64).to_radians(),
            EARTH_RADIUS as f64,
        );


        // to_cartesian
        let cart = CartesianPoint::from_geographic(&geo);
        let pos = DVec3::new(-cart.x(), -cart.y(), cart.z());
        // pub fn directions(&self)
        let up = pos.normalize().as_vec3();
        let west = Vec3::Z.cross(up);
        let _north = up.cross(west);
        transform.translation = pos.as_vec3();

        // Add camera / player height above ground
        transform.translation += up * ELEVATION;
        */
        transform.translation = translation;

        // There are differnt ways to rotate to point the camera to the Earth, independend of the lat/lon coordinates:
        if true {
            // Rotate form Bevy default to OSMeta default: Null-Island with Greenwich abowe
            // // transform.rotation = Quat::from_axis_angle(Vec3{x:0.,y:1.,z:0.},(-90.0_f32).to_radians());
            // // transform.rotate_axis(Vec3{x:1.,y:0.,z:0.},(90.0_f32).to_radians());
            // This can be done by one function: (But why not YXZ? If Y is rotated, do X and Z rotate to?!)
            transform.rotation = Quat::from_euler(
                EulerRot::YZX,
                (-90.0_f32).to_radians(),
                (-90.0_f32).to_radians(),
                0.0,
            );

            // Compensate first Lon and then Lat
            transform.rotate_local_z(-cam_data.lon.to_radians()); // Also ok: rotate_axis(Vec3::Z,
            transform.rotate_local_x(-cam_data.lat.to_radians());
        } else {
            // Looking down to earth center is quite more simple
            transform.look_to(-up, north); // OR: transform.look_at(Vec3::new(0.,0.,0.),north);
        }

        // Now rotate to the acutal view angles (alpha=directin,beta=up_view)
        // Absolute OR Relative/Local? The first may be both, the secound must be relative
        // // transform.rotate_axis(up, cam_data.direction.to_radians());
        transform.rotate_local_z(cam_data.direction.to_radians());
        transform.rotate_local_x((UP_VIEW + 90.).to_radians()); // todo: ok? tested:not needed   - FRAC_PI_2
    }
}

fn calc_geographic_translation(lat: f32, lon: f32, ele: f32, _dir: f32, dist: f32) -> Vec3 {
    let geo = GeographicPoint::new(
        (lon as f64).to_radians(),
        (lat as f64).to_radians(),
        EARTH_RADIUS as f64,
    );

    // to_cartesian / geographic
    let cart = CartesianPoint::from_geographic(&geo);
    let ground_pos = DVec3::new(-cart.x(), -cart.y(), cart.z()); // Mind the minus!
    let up = ground_pos.normalize().as_vec3();

    let mut pos = ground_pos.as_vec3() + up * ele;

    if !dist.abs().is_zero() {
        // Rotate form Bevy default to OSMeta default: Null-Island with Greenwich abowe
        let rotation_osmeta_default = Quat::from_euler(
            EulerRot::YZX,
            (-90.0_f32).to_radians(),
            (-90.0_f32).to_radians(),
            0.0,
        );
        let transform = Transform::from_rotation(rotation_osmeta_default);
        // Compensate first Lon and then Lat
        //    transform.rotate_local_z(-lon.to_radians());
        //    transform.rotate_local_x(-lat.to_radians());

        // Now rotate to the acutal view angles (alpha=directin,beta=up_view)
        //transform.rotate_local_z(dir.to_radians());
        //transform.rotate_local_x((UP_VIEW + 90.).to_radians());

        let ro = transform.rotation;
        let (dir, _) = ro.to_axis_angle();
        let boom_direction = dir.normalize();

        pos += boom_direction * dist;
    }

    pos
}
