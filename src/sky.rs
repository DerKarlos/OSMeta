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
    big_space::Space,
    geocoord::{GeoCoord, CLOUDS_HEIGHT, EARTH_RADIUS, MOON_ORBIT, MOON_RADIUS, SHOW_SIZE},
    geoview::{GeoView, Views},
    player::OSM_LAT_LIMIT,
    GalacticGrid, StartingValues,
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

#[derive(Component)]
pub struct Galactica;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
    mut views: ResMut<Views>,
    starting_values: Res<StartingValues>,
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

    let sphere = meshes.add(Sphere::new(1.0).mesh().uv(128, 64)); // todo: subdivisions?  128  // BAD!: .ico(8).unwrap()

    // earth ground
    let image = server.load("embedded://8k_earth_daymap.jpg");
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

    // Gamification
    if starting_values.gamification >= 1 {
        // Blue Sky
        commands.spawn((
            PbrBundle {
                mesh: sphere.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("000088").unwrap(),
                    unlit: true,
                    cull_mode: Some(bevy::render::render_resource::Face::Front),
                    ..default()
                }),
                transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS + CLOUDS_HEIGHT * 2.0)),
                ..default()
            },
            NotShadowCaster,
            GalacticGrid::ZERO,
        ));

        // Fog
        let material = materials.add(StandardMaterial {
            fog_enabled: false,
            ..default()
        });

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
                transform: Transform::from_scale(Vec3::splat(EARTH_RADIUS + CLOUDS_HEIGHT)),
                ..default()
            },
            NotShadowCaster,
            GalacticGrid::ZERO,
            NeedsTextureTransparencyEqualToRed(image.clone()),
            NeedsTextureSetToRepeat(image),
        ));

        // Rotational Earth-Axis
        let to_north_pole = Quat::from_axis_angle(Vec3::X, FRAC_PI_2); // X is to the equator below Greenwich?
        let transform = Transform::from_translation(Vec3::NEG_Z * EARTH_RADIUS * 1.5)
            .with_rotation(to_north_pole);
        let mesh = meshes.add(Cylinder::new(SHOW_SIZE, EARTH_RADIUS * 3.0).mesh());
        commands.spawn((
            PbrBundle {
                mesh,
                transform,
                material: material.clone(),
                ..default()
            },
            GalacticGrid::ZERO,
        ));

        // Earth-Equator
        let mesh = meshes.add(Cylinder::new(EARTH_RADIUS + SHOW_SIZE, SHOW_SIZE).mesh());
        commands.spawn((
            PbrBundle {
                mesh,
                transform: Transform::from_rotation(to_north_pole),
                material,
                ..default()
            },
            GalacticGrid::ZERO,
        ));

        // Moon
        let image = server.load("embedded://8k_moon.jpg");
        let (grid, pos): (GalacticGrid, Vec3) = Space::translation_to_grid(Vec3 {
            x: -MOON_ORBIT, // -X = Pacific +X = 0/0              Northpole? No z=Earth axis
            y: 1000.,
            z: 1000.,
        }); // x=Atlantic? -y=Greenwich +y=America
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
                transform: Transform::from_scale(Vec3::splat(MOON_RADIUS)).with_translation(pos),
                ..default()
            },
            NotShadowCaster,
            grid,
            // NeedsTextureSetToRepeat(image),
        ));

        // Test key 7: View next to the Moon
        GeoView {
            geo_coord: GeoCoord {
                lat: -0.4,
                lon: -0.4, // 179.6,
            },
            up_view: -OSM_LAT_LIMIT,
            elevation: MOON_ORBIT * 1.2,
            ..Default::default()
        }
        .store(KeyCode::Digit7, &mut views.map);

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
    }

    // Galactica
    if starting_values.gamification >= 2 {
        let view = GeoView {
            geo_coord: GeoCoord { lat: 0.1, lon: 0.2 },
            elevation: MOON_ORBIT / 2.0,
            ..Default::default()
        };

        // Test key 6: at (inside) the Galactica
        view.store(KeyCode::Digit6, &mut views.map);
        let galactic_transform = view.to_galactic_transform(false);
        let transform = galactic_transform.transform;
        let cell = galactic_transform.cell;

        // Loaded from: https://sketchfab.com/3d-models/crucero-medio-valkyrie-m-1-b394296dc39a493e92a441c14208a3cc#download
        let galactica = server.load("embedded://bs_galactica1.glb#Scene0"); // xwing Galactica  1701A2 crucero_medio_valkyrie_m_1
        commands.spawn((
            SceneBundle {
                scene: galactica,
                transform,
                ..Default::default()
            },
            NotShadowCaster,
            Galactica,
            cell,
        ));

        // Test key 5: below Galactica
        let view = GeoView {
            geo_coord: GeoCoord {
                lat: 0.1023688,
                lon: 0.20474587,
            },
            elevation: 6406666.,
            direction: -143.79845,
            up_view: 0.7181056,
            distance: 500.,
            ..Default::default()
        };
        view.store(KeyCode::Digit5, &mut views.map);
    }
}

// Set-Functions ////////////////

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
