//! Loads and renders a glTF file as a scene.

use bevy::{
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
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

fn main() {
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
            ),
        )
        .add_plugins(NoCameraPlayerPlugin) // https://github.com/sburris0/bevy_flycam (bevy_config_cam dies not work wiht Bevy 12)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3., 100., 400.)
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
            ..default()
        },
        FlyCam,
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
        },
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

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
