use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::camera::CameraMode;
use crate::car_dynamics::{Car, EgoControl, EgoState, MAX_STEERING_ANGLE, STEER_RATIO};
use crate::utils::normalize_angle;

use std::f32::consts::PI;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, keyboard_system);
        app.add_systems(Update, gamepad_system);
    }
}

// max wheel speed according to speed
fn get_steering_rate(v: f32) -> f32 {
    let omega_min = 2.0; // rad/s
    let omega0 = 12.0; // rad/s
    let lambda = 0.3;
    let v0 = 10.0; // m/s
    let omega_max = omega0 / (1.0 + (lambda * (v - v0)).exp()) + omega_min;
    omega_max
}

fn get_steering_angle(v: f32, front_wheel_angle: f32, ratio: f32, delta_time: f32) -> (f32, f32) {
    let steer_rate = get_steering_rate(v);
    let new_delta = (front_wheel_angle + steer_rate / STEER_RATIO * ratio * delta_time)
        .clamp(-MAX_STEERING_ANGLE, MAX_STEERING_ANGLE);
    (new_delta, new_delta * STEER_RATIO)
}

fn gamepad_system(
    gamepad: Single<&Gamepad>,
    mut query: Single<&mut EgoControl, With<Car>>,
    ego_state: Single<&EgoState, With<Car>>,
    mut camera: Single<&mut PanOrbitCamera>,
    time: Res<Time>,
) {
    const EPS: f32 = 0.01;
    let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
    if right_trigger > EPS {
        query.throttle = right_trigger.clamp(0.0, 1.0);
    }
    let left_trigger = gamepad.get(GamepadButton::LeftTrigger2).unwrap();
    if left_trigger > EPS {
        query.brake = left_trigger.clamp(0.0, 1.0);
    }

    let left_stick_x = gamepad
        .get(GamepadAxis::LeftStickX)
        .unwrap()
        .clamp(-1.0, 1.0);

    let ratio = -1.0 * left_stick_x;

    (query.front_wheel_angle, query.steer_wheel_angle) = get_steering_angle(
        ego_state.v,
        query.front_wheel_angle,
        ratio,
        time.delta_secs(),
    );

    let right_stick_x = gamepad
        .get(GamepadAxis::RightStickX)
        .unwrap()
        .clamp(-1.0, 1.0);
    if right_stick_x.abs() > EPS {
        camera.target_yaw = camera.yaw.unwrap() + right_stick_x * time.delta_secs() * PI;
    }
    let right_stick_y = gamepad
        .get(GamepadAxis::RightStickY)
        .unwrap()
        .clamp(-1.0, 1.0);
    if right_stick_y.abs() > EPS {
        camera.target_pitch = camera.pitch.unwrap() + right_stick_y * time.delta_secs() * PI;
    }
}

fn keyboard_system(
    key: Res<ButtonInput<KeyCode>>,
    mut query: Single<&mut EgoControl, With<Car>>,
    ego_state: Single<&EgoState, With<Car>>,
    mut camera: Single<&mut PanOrbitCamera>,
    camera_mode: Res<CameraMode>,
    time: Res<Time>,
) {
    if key.pressed(KeyCode::KeyW) {
        query.throttle = 1.0;
    } else {
        query.throttle = 0.0;
    }
    if key.pressed(KeyCode::KeyS) {
        query.brake = 1.0;
    } else {
        query.brake = 0.0;
    }
    if key.pressed(KeyCode::KeyA) {
        (query.front_wheel_angle, query.steer_wheel_angle) =
            get_steering_angle(ego_state.v, query.front_wheel_angle, 1.0, time.delta_secs());
    }
    if key.pressed(KeyCode::KeyD) {
        (query.front_wheel_angle, query.steer_wheel_angle) = get_steering_angle(
            ego_state.v,
            query.front_wheel_angle,
            -1.0,
            time.delta_secs(),
        );
    }
    if *camera_mode != CameraMode::ThirdPersonView {
        return;
    }
    if key.pressed(KeyCode::KeyJ) {
        camera.target_yaw = normalize_angle(camera.yaw.unwrap() + time.delta_secs() * PI);
    } else if key.pressed(KeyCode::KeyL) {
        camera.target_yaw = normalize_angle(camera.yaw.unwrap() - time.delta_secs() * PI);
    }
    if key.pressed(KeyCode::KeyI) {
        camera.target_pitch = normalize_angle(camera.pitch.unwrap() + time.delta_secs() * PI);
    } else if key.pressed(KeyCode::KeyK) {
        camera.target_pitch = normalize_angle(camera.pitch.unwrap() - time.delta_secs() * PI);
    }
}
