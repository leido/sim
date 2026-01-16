use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use std::f32::consts::{PI, TAU};

use crate::car_dynamics::{Car, EgoState};
use crate::utils::wrap;

#[derive(Resource, PartialEq, Eq, Debug)]
pub enum CameraMode {
    FirstPersonView,
    ThirdPersonView,
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
            .add_systems(Update, camera_mode_switch);
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
    car_transform: Single<&Transform, With<Car>>,
) {
    camera.target_focus = car_transform.translation;
}

fn camera_mode_switch(
    mut camera_mode: ResMut<CameraMode>,
    key: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut PanOrbitCamera>,
    ego_state: Single<&EgoState, With<Car>>,
) {
    if key.just_pressed(KeyCode::KeyH) {
        match *camera_mode {
            CameraMode::FirstPersonView => {
                *camera_mode = CameraMode::ThirdPersonView;
                camera.target_yaw = PI / 4.0;
                camera.target_pitch = -PI / 2.0;
                camera.target_radius = CAMERA_RADIUS;
                camera.force_update = true;
            }
            CameraMode::ThirdPersonView => *camera_mode = CameraMode::OverShoulderView,
            CameraMode::OverShoulderView => *camera_mode = CameraMode::FirstPersonView,
        }
    }
    if *camera_mode == CameraMode::OverShoulderView {
        // camera.target_yaw = normalize_angle(ego_state.yaw - PI / 2.0);
        camera.target_yaw = wrap(ego_state.yaw - PI / 2.0, -PI, PI);
        camera.target_pitch = -3.0 * PI / 8.0;
        camera.target_radius = 20.0;
        camera.force_update = true;
    } else if *camera_mode == CameraMode::FirstPersonView {
        camera.target_yaw = wrap(ego_state.yaw - PI / 2.0, -PI, PI);
        camera.target_pitch = -3.0 * PI / 8.0;
        camera.target_radius = 2.0;
        camera.force_update = true;
    }
}
