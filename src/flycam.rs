//! This module contains everything about the controls and rendering related
//! to the non-VR "player".

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};

use crate::LocalPlayer;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
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
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut movement_settings: ResMut<MovementSettings>,
) {
    let transform =
        Transform::from_xyz(0., 100., -300.).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y);

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

    commands
        .spawn((
            Camera3dBundle {
                transform,
                ..default()
            },
            InheritedVisibility::default(),
            FlyCam,
            EnvironmentMapLight {
                diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
                specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            },
            FogSettings {
                color: Color::rgba(0.35, 0.48, 0.66, 1.0),
                directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
                directional_light_exponent: 30.0,
                falloff: FogFalloff::from_visibility_colors(
                    10000.0, // distance in world units up to which objects retain visibility (>= 5% contrast)
                    Color::rgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
                    Color::rgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
                ),
            },
            LocalPlayer,
        ))
        .add_child(sphere);
    // FIXME: attach the camera bundle to the world, so when we move the world, the player is automatically moved with it.
    // We'll need this when the player moves very far or teleports to another place, as we need to ensure we don't go into
    // regions where the floating point numbers become imprecise.

    movement_settings.speed = 100.0;
}
