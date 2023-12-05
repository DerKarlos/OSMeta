//! Everything related to the global light and shadow logic

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap, NotShadowCaster},
    prelude::*,
    render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
};
use std::f32::consts::*;

use crate::{geopos::EARTH_RADIUS, GalacticGrid};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 })
            .add_systems(Startup, setup)
            .add_systems(Update, (animate_light_direction,));
    }
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
    mut images: ResMut<Assets<Image>>,
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
            sectors: 64,
            stacks: 32,
        }
        .try_into()
        .unwrap(),
    );

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
    if let Some(image) = images.get_mut(image.clone()) {
        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            address_mode_w: ImageAddressMode::Repeat,
            ..default()
        });
    }

    // ground
    commands.spawn((
        PbrBundle {
            mesh,
            material: materials.add(StandardMaterial {
                base_color: Color::hex("533621").unwrap(),
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
}
