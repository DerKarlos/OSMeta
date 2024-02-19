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
    geocoord::{GeoCoord, EARTH_RADIUS, MOON_ORBIT, MOON_RADIUS},
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


    // moon
    let image = server.load("embedded://8k_moon.jpg");

    //let coord = pos.as_coord();
    //let a = coord.to_geo_coord().to_cartesian();

    let (grid, pos): (GalacticGrid, glam::Vec3) = Space::translation_to_grid(Vec3{y:-MOON_ORBIT/20.,x:0.,z:0.}); // x=Atlantic? -y=Greenwich +y=America

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
            transform: Transform::from_scale(Vec3::splat(MOON_RADIUS))
                .with_translation(pos), //Vec3::X * MOON_ORBIT), // X = ? Northpole? No z=Earth axis
            ..default()
        },
        NotShadowCaster,
        grid, // GalacticGrid::ZERO, // ZERO? Makes it in-exact? If yes, not relevant for the moon.
        //        NeedsTextureSetToRepeat(image),
    ));


    GeoView {
        geo_coord: GeoCoord {
            lat: 48.1408,
            lon: 11.5577,
        },
        up_view: -OSM_LAT_LIMIT,
        elevation: 1.4,
        distance: 4000.0,
        ..Default::default()
    }
    .store("Key8".to_string(), &mut views.map); // below the clouds

    // Gamification: Galactica ////////////////////

    GeoView {
        geo_coord: GeoCoord {
            lat: 48.1408,
            lon: 11.5577,
        },
        up_view: -30.0,
        elevation: 1.4,
        distance: 500.,
        ..Default::default()
    }
    .store("Key3".to_string(), &mut views.map); // todo:  Munic?

    // View next to the Moon
    GeoView {
        geo_coord: GeoCoord {
            lat: -0.4,
            lon: 179.6,
        },
        up_view: -OSM_LAT_LIMIT,
        elevation: MOON_ORBIT * 1.2,
        //distance: MOON_RADIUS * 3.0,
        ..Default::default()
    }
    .store("Key9".to_string(), &mut views.map);

    if starting_values.gamification == 1 {

        // -85.05109 -179.99821      192199840 -77.87303 27.064236 500 77
        // -85.05109    0.0006583275 192199940 -75.85857 16.502523 500 77
        let view = GeoView {
            geo_coord: GeoCoord {
                lat: -85.05109,
                lon: 0.0006583275,
            },
            elevation: 192199840.,
            direction: -75.85857,
            up_view: 16.502523,
            distance: 500.,
            ..Default::default()
        };
        view.store("Key6".to_string(), &mut views.map);

        let view = GeoView {
            geo_coord: GeoCoord {
                lat: -OSM_LAT_LIMIT,
                lon: 180.0,
            },
            elevation: MOON_ORBIT / 2.0,
            ..Default::default()
        };

/*
        let gt = view.to_galactic_transform(false);
        let pd = gt.position_double();
        let origin = GeoCoord::from_cartesian(pd);
        let tile_size = origin.tile_size(15);
        info!(tile_size);

/ *                                                 zoom,                                                 coord,                                            pos.pos ,                   coord.right().to_geo_coord(),               coord.right().to_geo_coord().to_cartesian().pos ); //ää
2024-02-08T18:50:20.137052Z  INFO osmeta::geocoord:  15, TileCoord { pos: Vec2(16384.0, 32767.992), zoom: 15 }, DVec3( 550210.5697816543, -0.0, -6354223.188470841), GeoCoord { lat: -85.051125, lon: 0.010986328 }, DVec3(-550209.7135508333, -105.5013925298749,  -6354223.261735754),
2024-02-08T18:50:20.137101Z  INFO osmeta::geocoord:  15, TileCoord { pos: Vec2(16384.0, 32766.857), zoom: 15 }, DVec3(-550314.6419755649, -0.0, -6354214.17602738 ), GeoCoord { lat: -85.05005 , lon: 0.010986328 }, DVec3(-550329.0158086999, -105.52426845159509, -6354212.930271038),
* /

    let pos = player.pos();
    let origin = GeoCoord::from_cartesian(pos);
    let tile_size = origin.tile_size(TILE_ZOOM);

        let coord = self.to_tile_coordinates(zoom);
        let pos   = self.to_cartesian();
        let xxx   =coord.right().to_geo_coord().to_cartesian().distance(*pos);

    let coord = crate::tilemap::TileCoord { pos: Vec2(16384.0, 32767.992), zoom: 15 };
    let mut pos   = coord.to_cartesian();
    pos = pos.right();

*/




        view.store("Key7".to_string(), &mut views.map);
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
    }
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
