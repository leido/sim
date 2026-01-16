use std::f32::consts::PI;

pub fn normalize_angle(angle: f32) -> f32 {
    let two_pi = 2.0 * PI;
    let normalized = angle % two_pi;
    match normalized {
        x if x > PI => x - two_pi,
        x if x < -PI => x + two_pi,
        x => x,
    }
}

pub fn wrap(x: f32, low: f32, high: f32) -> f32 {
    // already in range
    if low <= x && x <= high {
        return x;
    }
    let range = high - low;
    let inv_range = 1.0 / range;
    let num_wraps = ((x - low) * inv_range).floor();
    return x - range * num_wraps;
}