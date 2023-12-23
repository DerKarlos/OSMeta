use bevy::pbr::build_directional_light_cascades;
use bevy::prelude::*;
use bevy_oxr::input::XrInput;
use bevy_oxr::resources::{XrFrameState, XrSession};
use bevy_oxr::xr_input::actions::XrActionSets;
use bevy_oxr::xr_input::interactions::{
    draw_interaction_gizmos, draw_socket_gizmos, interactions, socket_interactions,
    update_interactable_states, InteractionEvent, XRDirectInteractor, XRInteractorState,
    XRRayInteractor,
};
use bevy_oxr::xr_input::oculus_touch::OculusController;
use bevy_oxr::xr_input::prototype_locomotion::{
    proto_locomotion, PrototypeLocomotionConfig, RotationType,
};
use bevy_oxr::xr_input::trackers::{
    AimPose, OpenXRController, OpenXRLeftController, OpenXRRightController, OpenXRTracker,
};
use bevy_oxr::xr_input::Hand;
use bevy_oxr::xr_input::{
    hands::common::{HandInputDebugRenderer, OpenXrHandInput},
    xr_camera::XRProjection,
};
use bevy_oxr::DefaultXrPlugins;
use big_space::FloatingOrigin;

use crate::GalacticGrid;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultXrPlugins.build().disable::<TransformPlugin>())
            .add_systems(Update, proto_locomotion)
            .insert_resource(PrototypeLocomotionConfig {
                locomotion_speed: 10.0,
                rotation_type: RotationType::Snap,
                ..default()
            })
            .add_systems(Startup, spawn_controllers_example)
            .add_plugins(OpenXrHandInput)
            .add_plugins(HandInputDebugRenderer)
            .add_systems(
                Update,
                draw_interaction_gizmos.after(update_interactable_states),
            )
            .add_systems(Update, draw_socket_gizmos.after(update_interactable_states))
            .add_systems(Update, interactions.before(update_interactable_states))
            .add_systems(
                Update,
                socket_interactions.before(update_interactable_states),
            )
            .add_systems(Update, prototype_interaction_input)
            .add_systems(Update, update_interactable_states)
            .add_systems(Update, update_grabbables.after(update_interactable_states))
            // Ensure that the XR cameras are registered with light cascades (otherwise we'll get a panic)
            .add_systems(
                PostUpdate,
                build_directional_light_cascades::<XRProjection>
                    .after(build_directional_light_cascades::<Projection>),
            )
            .add_event::<InteractionEvent>();
    }
}

fn spawn_controllers_example(mut commands: Commands) {
    //left hand
    commands.spawn((
        OpenXRLeftController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
        XRRayInteractor,
        AimPose(Transform::default()),
        XRInteractorState::default(),
        FloatingOrigin,
        GalacticGrid::ZERO,
    ));
    //right hand
    commands.spawn((
        OpenXRRightController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
        XRDirectInteractor,
        XRInteractorState::default(),
    ));
}

fn prototype_interaction_input(
    oculus_controller: Res<OculusController>,
    frame_state: Res<XrFrameState>,
    xr_input: Res<XrInput>,
    session: Res<XrSession>,
    mut right_interactor_query: Query<
        &mut XRInteractorState,
        (
            With<XRDirectInteractor>,
            With<OpenXRRightController>,
            Without<OpenXRLeftController>,
        ),
    >,
    mut left_interactor_query: Query<
        &mut XRInteractorState,
        (
            With<XRRayInteractor>,
            With<OpenXRLeftController>,
            Without<OpenXRRightController>,
        ),
    >,
    action_sets: Res<XrActionSets>,
) {
    //lock frame
    let frame_state = *frame_state.lock().unwrap();
    //get controller
    let controller = oculus_controller.get_ref(&session, &frame_state, &xr_input, &action_sets);
    //get controller triggers
    let left_trigger = controller.trigger(Hand::Left);
    let right_trigger = controller.trigger(Hand::Right);
    //get the interactors and do state stuff
    let mut left_state = left_interactor_query.single_mut();
    if left_trigger > 0.8 {
        *left_state = XRInteractorState::Selecting;
    } else {
        *left_state = XRInteractorState::Idle;
    }
    let mut right_state = right_interactor_query.single_mut();
    if right_trigger > 0.8 {
        *right_state = XRInteractorState::Selecting;
    } else {
        *right_state = XRInteractorState::Idle;
    }
}

#[derive(Component)]
struct Grabbable;

fn update_grabbables(
    mut events: EventReader<InteractionEvent>,
    mut grabbable_query: Query<(&mut Transform, With<Grabbable>, Without<XRDirectInteractor>)>,
    interactor_query: Query<(&GlobalTransform, &XRInteractorState, Without<Grabbable>)>,
) {
    //so basically the idea is to try all the events?
    for event in events.read() {
        // info!("some event");
        match grabbable_query.get_mut(event.interactable) {
            Ok(mut grabbable_transform) => {
                // info!("we got a grabbable");
                //now we need the location of our interactor
                match interactor_query.get(event.interactor) {
                    Ok(interactor_transform) => {
                        match interactor_transform.1 {
                            XRInteractorState::Idle => (),
                            XRInteractorState::Selecting => {
                                // info!("its a direct interactor?");
                                *grabbable_transform.0 = interactor_transform.0.compute_transform();
                            }
                        }
                    }
                    Err(_) => {
                        // info!("not a direct interactor")
                    }
                }
            }
            Err(_) => {
                // info!("not a grabbable?")
            }
        }
    }
}


pub fn pull_to_ground(
    time: Res<Time>,
    mut tracking_root_query: Query<GalacticTransform, With<OpenXRTrackingRoot>>,
    space: Res<FloatingOriginSettings>,
) {
    let Ok(mut root) = tracking_root_query.get_single_mut() else {
        return;
    };

    let adjustment_rate = (time.delta_seconds() * 10.0).min(1.0);

    // Lower player onto sphere
    let real_pos = root.position_double(&space);
    let up = real_pos.normalize();
    let diff = up * EARTH_RADIUS as f64 - real_pos;
    root.transform.translation += diff.as_vec3() * adjustment_rate;

    // Rotate player to be upright on sphere
    let angle_diff = Quat::from_rotation_arc(root.transform.up(), up.as_vec3());
    root.transform
        .rotate(Quat::IDENTITY.slerp(angle_diff, adjustment_rate));
}
