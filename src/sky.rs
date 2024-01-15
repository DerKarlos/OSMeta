//! Everything related to the global light and shadow logic

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, NotShadowCaster},
    prelude::*,
    render::{
        render_resource::TextureFormat,
        texture::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    },
};
use std::f32::consts::*;

use crate::{
    geocoord::{EARTH_RADIUS, MOON_ORBIT, MOON_RADIUS},
    GalacticGrid,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    animate_light_direction,
                    set_texture_transparent,
                    set_texture_repeat,
                ),
            );
    }
}

#[derive(Component)]
struct NeedsTextureSetToRepeat(Handle<Image>);

#[derive(Component)]
struct NeedsTextureTransparencyEqualToRed(Handle<Image>);

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            time.elapsed_seconds() * PI / 100.0,
            0.0,
            -FRAC_PI_4,
        );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    // Sun
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

    let sphere = meshes.add(
        shape::UVSphere {
            radius: 1.0,
            sectors: 128,
            stacks: 64,
        }
        .try_into()
        .unwrap(),
    );

    // Stars
    let image = server.load("embedded://8k_stars.jpg");
    commands.spawn((
        PbrBundle {
            mesh: sphere.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                base_color_texture: Some(image.clone()),
                fog_enabled: false,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS + 1_000_000_000_000.0)),
            ..default()
        },
        NeedsTextureSetToRepeat(image),
        NotShadowCaster,
        GalacticGrid::ZERO,
    ));

    // Earth

    let rot = Quat::from_axis_angle(Vec3::X, FRAC_PI_2);
    let transform =
        Transform::from_translation(Vec3::NEG_Z * EARTH_RADIUS * 1.5).with_rotation(rot);

    let material = materials.add(StandardMaterial {
        fog_enabled: false,
        ..default()
    });

    // Rotational axis
    let mesh = meshes.add(
        shape::Cylinder {
            radius: 1000.0,
            height: EARTH_RADIUS * 6.0,
            resolution: 16,
            segments: 1,
        }
        .into(),
    );
    commands.spawn((
        PbrBundle {
            mesh,
            transform,
            material: material.clone(),
            ..default()
        },
        GalacticGrid::ZERO,
    ));

    // Equator
    let mesh = meshes.add(
        shape::Cylinder {
            radius: EARTH_RADIUS + 1000.0,
            height: 1.0,
            resolution: 64,
            segments: 1,
        }
        .into(),
    );
    commands.spawn((
        PbrBundle {
            mesh,
            transform: Transform::from_rotation(rot),
            material,
            ..default()
        },
        GalacticGrid::ZERO,
    ));

    // Clouds visible from earth and space
    let image = server.load("embedded://8k_earth_clouds.jpg");
    commands.spawn((
        PbrBundle {
            mesh: sphere.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                base_color_texture: Some(image.clone()),
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS + 100000.0)),
            ..default()
        },
        NotShadowCaster,
        GalacticGrid::ZERO,
        NeedsTextureTransparencyEqualToRed(image.clone()),
        NeedsTextureSetToRepeat(image),
    ));

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: sphere.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("000088").unwrap(),
                unlit: true,
                cull_mode: Some(bevy::render::render_resource::Face::Front),
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS + 200000.0)),
            ..default()
        },
        NotShadowCaster,
        GalacticGrid::ZERO,
    ));

    let image = server.load("embedded://8k_earth_daymap.jpg");

    // ground
    commands.spawn((
        PbrBundle {
            mesh: sphere.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                cull_mode: None,
                base_color_texture: Some(image.clone()),
                perceptual_roughness: 1.0,
                fog_enabled: false,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS)),
            ..default()
        },
        NotShadowCaster,
        GalacticGrid::ZERO,
        NeedsTextureSetToRepeat(image),
    ));

    let image = server.load("embedded://8k_moon.jpg");

    // moon
    commands.spawn((
        PbrBundle {
            mesh: sphere,
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                cull_mode: None,
                base_color_texture: Some(image.clone()),
                perceptual_roughness: 1.0,
                fog_enabled: false,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(MOON_RADIUS))
                .with_translation(Vec3::X * MOON_ORBIT),
            ..default()
        },
        NotShadowCaster,
        GalacticGrid::ZERO,
        NeedsTextureSetToRepeat(image),
    ));
}

fn set_texture_transparent(
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    server: Res<AssetServer>,
    textures: Query<(Entity, &NeedsTextureTransparencyEqualToRed)>,
) {
    for (entity, NeedsTextureTransparencyEqualToRed(handle)) in textures.iter() {
        use bevy::asset::LoadState::*;
        match server.get_load_state(handle).unwrap() {
            Loaded => {
                let Some(image) = images.get_mut(handle) else {
                    unreachable!()
                };
                *image = image.convert(TextureFormat::Rgba8UnormSrgb).unwrap();
                for chunk in image.data.chunks_exact_mut(4) {
                    let [r, _g, _b, a] = chunk else {
                        unreachable!()
                    };
                    *a = *r;
                }
                commands
                    .entity(entity)
                    .remove::<NeedsTextureTransparencyEqualToRed>();
            }
            Failed => {
                commands
                    .entity(entity)
                    .remove::<NeedsTextureTransparencyEqualToRed>();
            }
            _ => (),
        }
    }
}

fn set_texture_repeat(
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    server: Res<AssetServer>,
    textures: Query<(Entity, &NeedsTextureSetToRepeat)>,
) {
    for (entity, NeedsTextureSetToRepeat(handle)) in textures.iter() {
        use bevy::asset::LoadState::*;
        match server.get_load_state(handle).unwrap() {
            Loaded => {
                let Some(image) = images.get_mut(handle) else {
                    unreachable!()
                };
                image.sampler = repeat_sampler();
                commands.entity(entity).remove::<NeedsTextureSetToRepeat>();
            }
            Failed => {
                commands.entity(entity).remove::<NeedsTextureSetToRepeat>();
            }
            _ => (),
        }
    }
}

fn repeat_sampler() -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        anisotropy_clamp: 4,
        ..default()
    })
}
