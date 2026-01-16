use crate::utils::normalize_angle;
use bevy::prelude::*;
use std::{
    f32::consts::PI,
    ops::{Add, Mul},
};

pub struct CarDynamicsPlugin;

impl Plugin for CarDynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (bicycle_kinematic_model, wheel_movement));
    }
}

const WHEELBASE: f32 = 3.0;
const MAX_ACCELERATION: f32 = 5.0;
pub const MAX_STEERING_ANGLE: f32 = 35.0f32.to_radians();
const MAX_SPEED: f32 = 33.3;
pub const STEER_RATIO: f32 = 15.0;

#[derive(Component)]
pub struct Car;

#[derive(Component, Default)]
pub struct EgoControl {
    pub throttle: f32,
    pub brake: f32,
    pub front_wheel_angle: f32,
    pub steer_wheel_angle: f32,
}

#[derive(Component)]
pub struct RollingWheel;

#[derive(Component)]
pub struct SteeringWheel;

#[derive(Component)]
pub struct InitWheelRotation(pub Quat);

#[derive(Copy, Clone, Component, Default)]
pub struct EgoState {
    pub x: f32,
    pub y: f32,
    pub yaw: f32,
    pub v: f32,
    pub s: f32,
}

#[derive(Copy, Clone, Default)]
pub struct EgoStateDerivative {
    pub dx: f32,
    pub dy: f32,
    pub dtheta: f32,
    pub dv: f32,
    pub ds: f32,
}

impl EgoState {
    pub fn apply_derivative(&mut self, dq: EgoStateDerivative, dt: f32) {
        *self = *self + dq * dt;
    }
}

impl Add<EgoStateDerivative> for EgoState {
    type Output = Self;

    fn add(self, rhs: EgoStateDerivative) -> Self::Output {
        EgoState {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
            yaw: normalize_angle(self.yaw + rhs.dtheta),
            v: self.v + rhs.dv,
            s: self.s + rhs.ds,
        }
    }
}

impl Mul<f32> for EgoStateDerivative {
    type Output = Self;

    fn mul(self, dt: f32) -> Self::Output {
        EgoStateDerivative {
            dx: self.dx * dt,
            dy: self.dy * dt,
            dtheta: self.dtheta * dt,
            dv: self.dv * dt,
            ds: self.ds * dt,
        }
    }
}

pub fn wheel_movement(
    mut wheels: Query<
        (&mut Transform, &InitWheelRotation, Option<&SteeringWheel>),
        With<RollingWheel>,
    >,
    control: Single<&EgoControl, With<Car>>,
    q: Single<&EgoState, With<Car>>,
) {
    let d_theta = (q.s / 0.35).rem_euclid(2.0 * PI);
    let x_rot = Quat::from_rotation_x(d_theta); // wheel roll
    let delta = control.front_wheel_angle;
    for (mut transform, init_rot, is_steer) in wheels.iter_mut() {
        transform.rotation = x_rot * init_rot.0;
        if is_steer.is_some() {
            transform.rotate_y(delta); // wheel steer
        }
    }
}

fn calculate_acceleration(throttle: f32, v: f32) -> f32 {
    MAX_ACCELERATION * throttle * (1.0 - v / MAX_SPEED)
}

fn calculate_deceleration(brake: f32, v: f32) -> f32 {
    let brake_curve_midpoint = 0.4;
    let brake_response_gain = 7.0;
    let v_threshold = 0.2;
    let a_target = if brake == 0.0 {
        0.0
    } else {
        -1.0 * MAX_ACCELERATION
            / (1.0 + (-brake_response_gain * (brake - brake_curve_midpoint)).exp())
    };
    let a_brake = if v > v_threshold {
        a_target
    } else {
        a_target * (v / v_threshold)
    };
    a_brake
}

fn bicycle_kinematic_model(
    state: Single<(&mut Transform, &mut EgoState), With<Car>>,
    control: Single<&EgoControl, With<Car>>,
    time: Res<Time>,
) {
    let delta = control.front_wheel_angle;
    let (mut trans, mut q) = state.into_inner();
    let dq = EgoStateDerivative {
        dx: q.v * q.yaw.cos(),
        dy: q.v * q.yaw.sin(),
        dtheta: q.v * delta.tan() / WHEELBASE,
        dv: calculate_acceleration(control.throttle, q.v)
            + calculate_deceleration(control.brake, q.v),
        ds: q.v,
    };
    let dt = time.delta_secs();
    q.apply_derivative(dq, dt);

    trans.translation.x = q.x;
    trans.translation.y = q.y;
    trans.rotate_z(dq.dtheta * dt);
}
