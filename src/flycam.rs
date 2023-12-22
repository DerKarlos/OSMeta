//! This module contains everything about the controls and rendering related
//! to the non-VR "player".

use bevy::window::CursorGrabMode;
use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_flycam::{FlyCam, KeyBindings, MovementSettings, NoCameraPlayerPlugin};
use big_space::{FloatingOrigin, FloatingOriginSettings};

use crate::geocoord::EARTH_RADIUS;
use crate::{GalacticGrid, GalacticTransform};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, update_camera_speed)
            .add_systems(Update, grab_cursor)
            .add_plugins(NoCameraPlayerPlugin); // https://github.com/sburris0/bevy_flycam (bevy_config_cam dies not work wiht Bevy 12)
    }
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut movement_settings: ResMut<MovementSettings>,
    mut keys: ResMut<KeyBindings>,
    args: Res<crate::Args>,
    space: Res<FloatingOriginSettings>,
) {
    let height_direction = args.starting_position.normalize().as_vec3();
    let (grid, subgrid): (GalacticGrid, _) = space.translation_to_grid(args.starting_position);

    let mut transform = Transform::from_translation(subgrid + height_direction * args.elevation)
        // This "hack" rotates the camera-view down and accroding to Lat/Lon
        .looking_at(subgrid, Vec3::Z) // ::Z for Nord
        ;

    // Todo?: Rotate realy by Lat/Lon
    // Bevy Camera default-rotation is: view to -z and up = +y
    // OSMeta Earth is Nord = +Z and Greenwich at 0 degrees to -Y? so the default view should be +Y.
    // So the initial rotaton needs to be 90 degrees -X ???
    // let mut rotation = Quat::from_axis_angle(Vec3::Y, (-90_f32).to_radians())
    //                  * Quat::from_axis_angle(Vec3::Z, (-90_f32).to_radians())

    let rotation = Quat::from_axis_angle(Vec3::Z, args.direction.to_radians())
        * Quat::from_axis_angle(Vec3::X, args.up_view.to_radians());
    transform = transform * Transform::from_rotation(rotation);

    movement_settings.up = height_direction;

    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 1.0,
            subdivisions: 10,
        }
        .try_into()
        .unwrap(),
    );
    let sphere = commands
        .spawn(PbrBundle {
            mesh,
            material,
            ..default()
        })
        .id();

    let mut camera = commands.spawn((
        Camera3dBundle {
            transform,
            ..default()
        },
        InheritedVisibility::default(),
        FlyCam,
        grid,
        FogSettings {
            color: Color::rgba(0.35, 0.48, 0.66, 1.0),
            directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
            directional_light_exponent: 30.0,
            falloff: FogFalloff::from_visibility_colors(
                EARTH_RADIUS * 2.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
            ),
        },
    ));
    camera.add_child(sphere);
    if !args.xr {
        camera.insert(FloatingOrigin);
    }
    // FIXME: attach the camera bundle to the world, so when we move the world, the player is automatically moved with it.
    // We'll need this when the player moves very far or teleports to another place, as we need to ensure we don't go into
    // regions where the floating point numbers become imprecise.

    movement_settings.speed = 100.0;
    // Don't use ESC for grabbing/releasing the cursor. That's what browsers use, too, so it gets grabbed by bevy and released by the browser at the same time.
    keys.toggle_grab_cursor = KeyCode::G;
}

fn update_camera_speed(
    mut movement_settings: ResMut<MovementSettings>,
    fly_cam: Query<GalacticTransform, With<FlyCam>>,
    space: Res<FloatingOriginSettings>,
) {
    let elevation = fly_cam.single().position_double(&space).length() as f32;
    let speed = (1. * (elevation - crate::geocoord::EARTH_RADIUS - 300.0)).max(100.0);
    movement_settings.speed = speed;
}

// Todo ? Merge both to fn update? To many different parameters?

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
