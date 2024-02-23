use crate::player;
use crate::GalacticTransform;
use bevy::{pbr::NotShadowCaster, prelude::*};

#[cfg(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32"))))]
use bevy_oxr::xr_input::trackers::OpenXRTrackingRoot;

// Compass
#[derive(Component)]
pub struct Compass;

pub fn reposition_compass(
    mut compass: Query<
        GalacticTransform,
        (
            With<Compass>,
            Without<player::Control>,
            Without<OpenXRTrackingRoot>,
        ),
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
    player: player::PlayerQuery,
) {
    if let Ok(mut compass) = compass.get_single_mut() {
        let player = player.pos();
        let directions = player.directions();
        compass.transform.translation =
            player.galactic_transform.transform.translation - directions.up * 5.;
        *compass.cell = player.cell;
        compass.transform.look_to(directions.north, directions.up)
    } else {
        let mesh = meshes.add(Plane3d::default());
        let image = server.load("embedded://compass.png");
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(image),
            unlit: true,
            cull_mode: None,
            perceptual_roughness: 1.0,
            fog_enabled: false,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh,
                material,
                ..default()
            },
            crate::GalacticGrid::ZERO,
            Compass,
            NotShadowCaster,
        ));
    }
}

#[cfg(not(all(feature = "xr", not(any(target_os = "macos", target_arch = "wasm32")))))]
/// HACK: we can't attach `LocalPlayer` to the xr player yet, so we need
/// to access the OpenXRTrackingRoot, but that doesn't exist without the xr feature.
/// So we create a dummy that is not attached to anything on platforms without the XR player.
#[derive(Component)]
pub struct OpenXRTrackingRoot;
