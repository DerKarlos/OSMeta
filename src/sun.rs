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
    geopos::{EARTH_RADIUS, MOON_ORBIT, MOON_RADIUS},
    GalacticGrid,
};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .insert_resource(Clouds::default())
            .insert_resource(Earth::default())
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    animate_light_direction,
                    set_clouds_transparent,
                    set_earth_repeat,
                ),
            );
    }
}

#[derive(Resource, Default)]
struct Clouds {
    image: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
struct Earth {
    images: Vec<Handle<Image>>,
}

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
    mut clouds: ResMut<Clouds>,
    mut earth: ResMut<Earth>,
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

    let mesh = meshes.add(
        shape::UVSphere {
            radius: 1.0,
            sectors: 128,
            stacks: 64,
        }
        .try_into()
        .unwrap(),
    );

    // Stars
    let image = server.load("https://www.solarsystemscope.com/textures/download/2k_stars.jpg");
    earth.images.push(image.clone());
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                base_color_texture: Some(image),
                fog_enabled: false,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS + 1000_000_000_000.0)),
            ..default()
        },
        NotShadowCaster,
        GalacticGrid::ZERO,
    ));

    // Clouds visible from earth and space
    let image =
        server.load("https://www.solarsystemscope.com/textures/download/2k_earth_clouds.jpg");
    clouds.image = Some(image.clone());
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                alpha_mode: AlphaMode::Blend,
                cull_mode: None,
                base_color_texture: Some(image),
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS + 100000.0)),
            ..default()
        },
        NotShadowCaster,
        GalacticGrid::ZERO,
    ));

    // Sky
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
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

    let image =
        server.load("https://www.solarsystemscope.com/textures/download/2k_earth_daymap.jpg");
    earth.images.push(image.clone());

    // ground
    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                cull_mode: None,
                base_color_texture: Some(image),
                perceptual_roughness: 1.0,
                fog_enabled: false,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS)),
            ..default()
        },
        NotShadowCaster,
        GalacticGrid::ZERO,
    ));

    let image = server.load("https://www.solarsystemscope.com/textures/download/2k_moon.jpg");
    earth.images.push(image.clone());

    // moon
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                unlit: true,
                cull_mode: None,
                base_color_texture: Some(image),
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
    ));
}

fn set_clouds_transparent(
    mut images: ResMut<Assets<Image>>,
    mut clouds: ResMut<Clouds>,
    server: Res<AssetServer>,
) {
    if let Some(handle) = clouds.image.take() {
        use bevy::asset::LoadState::*;
        if let Loaded | Failed = server.get_load_state(&handle).unwrap() {
            let Some(image) = images.get_mut(&handle) else {
                unreachable!()
            };
            info!("clouds made transparent and linear filtered");
            *image = image.convert(TextureFormat::Rgba8UnormSrgb).unwrap();
            for chunk in image.data.chunks_exact_mut(4) {
                let [r, _g, _b, a] = chunk else {
                    unreachable!()
                };
                *a = *r;
            }
            image.sampler = repeat_sampler();
        } else {
            clouds.image = Some(handle);
        }
    }
}

fn set_earth_repeat(
    mut images: ResMut<Assets<Image>>,
    mut earth: ResMut<Earth>,
    server: Res<AssetServer>,
) {
    earth.images.retain(|handle| {
        use bevy::asset::LoadState::*;
        if let Loaded | Failed = server.get_load_state(handle).unwrap() {
            let Some(image) = images.get_mut(handle) else {
                unreachable!()
            };
            image.sampler = repeat_sampler();
            false
        } else {
            true
        }
    });
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
