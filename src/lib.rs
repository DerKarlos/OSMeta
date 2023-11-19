//! Loads and renders a glTF file as a scene.

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use std::f32::consts::*;
use tilemap::TileMap;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
use xr::XRPlugin;

use bevy_flycam::prelude::*;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
use bevy_oxr::DefaultXrPlugins;

mod tilemap;
#[cfg(all(feature = "xr", not(target_os = "macos")))]
mod xr;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    if std::env::args().any(|arg| arg == "xr") {
        #[cfg(all(feature = "xr", not(target_os = "macos")))]
        app.add_plugins(DefaultXrPlugins).add_plugins(XRPlugin);
    } else {
        app.add_plugins(DefaultPlugins);
    }
    app.insert_resource(DirectionalLightShadowMap { size: 4096 })
        .insert_resource(Msaa::Sample4) // Msaa::Sample4  Msaa::default()   -- Todo: tut nichts?
        .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_light_direction,
                update_active_tile_zone,
                tilemap::update,
                move_shape_with_camera,
            ),
        )
        .add_plugins(NoCameraPlayerPlugin) // https://github.com/sburris0/bevy_flycam (bevy_config_cam dies not work wiht Bevy 12)
        .run();
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

#[derive(Component)]
struct Shape;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut movement_settings: ResMut<MovementSettings>,
) {
    let transform =
        Transform::from_xyz(3., 100., 400.).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform,
            ..default()
        },
        FlyCam,
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        },
    ));

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
    commands.spawn((
        PbrBundle {
            mesh,
            material,
            transform,
            ..default()
        },
        Shape,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000., // lux  https://docs.rs/bevy/latest/bevy/pbr/struct.DirectionalLight.html
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            //maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    movement_settings.speed = 100.0;

    commands.spawn(TileMap::new(&mut meshes));
}

fn update_active_tile_zone(mut commands: Commands, mut tilemap: Query<&mut TileMap>) {
    let mut tilemap = tilemap.single_mut();
    tilemap.load(&mut commands, 17429, 11369);
    tilemap.load(&mut commands, 17429, 11370);
    tilemap.load(&mut commands, 17429, 11371);

    tilemap.load(&mut commands, 17430, 11369);
    tilemap.load(&mut commands, 17430, 11370);
    tilemap.load(&mut commands, 17430, 11371);

    tilemap.load(&mut commands, 17431, 11369);
    tilemap.load(&mut commands, 17431, 11370);
    tilemap.load(&mut commands, 17431, 11371);
}

fn move_shape_with_camera(
    mut shape: Query<&mut Transform, (With<Shape>, Without<FlyCam>)>,
    camera: Query<&Transform, (Without<Shape>, With<FlyCam>)>,
) {
    *shape.single_mut() = *camera.single();
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 100.0,
            -FRAC_PI_4,
        );
    }
}
