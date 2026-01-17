use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use std::f32::consts::{PI, TAU};

use crate::car_dynamics::{Car, EgoState};
use crate::utils::wrap;

#[derive(Resource, PartialEq, Eq, Debug, Clone, Copy)]
pub enum CameraMode {
    ThirdPersonView,
    FirstPersonView,
    OverShoulderView,
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, setup_camera)
            .add_systems(
                PostUpdate,
                camera_follow.before(TransformSystems::Propagate),
            )
            .add_systems(Update, camera_mode_switch)
            .add_systems(
                Update,
                set_camera_for_mode
                    .run_if(resource_changed::<CameraMode>.and(not(resource_added::<CameraMode>))),
            );
    }
}

const CAMERA_RADIUS: f32 = 50.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        PanOrbitCamera {
            axis: [Vec3::X, Vec3::Z, Vec3::Y],
            focus: Vec3::ZERO,
            radius: Some(CAMERA_RADIUS),
            pitch: Some(-TAU / 8.0),
            yaw: Some(0.0),
            pitch_lower_limit: Some(-TAU / 4.0),
            pitch_upper_limit: Some(TAU / 4.0),
            zoom_lower_limit: 0.001,
            ..default()
        },
    ));
    commands.insert_resource(CameraMode::ThirdPersonView);
}

fn camera_follow(
    mut camera: Single<&mut PanOrbitCamera>,
    camera_mode: Res<CameraMode>,
    car_transform: Single<&Transform, With<Car>>,
    ego_state: Single<&EgoState, With<Car>>
) {
    camera.target_focus = car_transform.translation;
    if *camera_mode != CameraMode::ThirdPersonView {
        camera.target_yaw = wrap(ego_state.yaw - 90.0_f32.to_radians(), -PI, PI);
        let yaw_d = camera.target_yaw - camera.yaw.unwrap();
        if yaw_d > PI {
            camera.yaw = camera.yaw.map(|y| y + TAU);
        } else if yaw_d < -PI {
            camera.yaw = camera.yaw.map(|y| y - TAU);
        }
        let offset = Vec3::new(3.0, 0.0, 0.0).rotate_z(ego_state.yaw);
        camera.target_focus = car_transform.translation + offset;
    }
}

fn camera_mode_switch(mut camera_mode: ResMut<CameraMode>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::KeyH) {
        match *camera_mode {
            CameraMode::ThirdPersonView => *camera_mode = CameraMode::FirstPersonView,
            CameraMode::FirstPersonView => *camera_mode = CameraMode::OverShoulderView,
            CameraMode::OverShoulderView => *camera_mode = CameraMode::ThirdPersonView,
        }
    }
}

fn set_camera_for_mode(
    camera_mode: Res<CameraMode>,
    mut camera: Single<&mut PanOrbitCamera>,
) {
    println!("Switching to camera mode: {:?}", *camera_mode);
    match *camera_mode {
        CameraMode::ThirdPersonView => {
            camera.target_yaw = PI / 4.0;
            camera.target_pitch = -60.0_f32.to_radians();
            camera.target_radius = CAMERA_RADIUS;
            camera.force_update = true;
        }
        CameraMode::FirstPersonView => {
            camera.target_pitch = -80.0_f32.to_radians();
            camera.target_radius = 2.0;
            camera.force_update = true;

        }
        CameraMode::OverShoulderView => {
            camera.target_pitch = -3.0 * PI / 8.0;
            camera.target_radius = 20.0;
            camera.force_update = true;
        }
    }
}
