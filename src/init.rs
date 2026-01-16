use avian3d::prelude::*;
use bevy::color::palettes::css::*;
use bevy::color::palettes::tailwind::GRAY_100;
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::car_dynamics::*;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec3::NEG_Z * 9.8))
            .add_systems(Startup, setup_gizmos)
            .add_systems(Startup, spawn_ego)
            .add_systems(Startup, spawn_obstacle);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyGizmo;

fn setup_gizmos(mut commands: Commands, mut gizmo_assets: ResMut<Assets<GizmoAsset>>) {
    let mut gizmos = GizmoAsset::new();

    let length = 100.0;
    gizmos.arrow(Vec3::ZERO, Vec3::X * length, RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Y * length, GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::Z * length, BLUE);

    draw_ground_grid(&mut gizmos);

    commands.spawn(Gizmo {
        handle: gizmo_assets.add(gizmos),
        ..default()
    });
}

fn spawn_ego(mut commands: Commands, asset_server: Res<AssetServer>) {
    let car = asset_server.load("car-race.glb#Scene0");
    let wheel = asset_server.load("wheel.glb#Scene0");

    // Config
    // Manual collider for SceneRoot (scene-based colliders not yet supported)
    // These dimensions match the visual car model
    let car_collider = Collider::cuboid(2., 1.0, 5.0);

    // Ground is at z=0, wheels are at relative y=-0.25
    // Car height = wheel_radius (0.34) + wheel_offset (0.25) to position wheels on ground
    let car_height = 0.64;
    let car_transform = Transform::from_translation(Vec3::new(0.0, 0.0, car_height))
        .with_rotation(Quat::from_rotation_z(PI / 2.0) * Quat::from_rotation_x(PI / 2.0));

    let wheel_scale = Vec3::new(0.7, 0.34, 0.7);
    let wheels = [
        // Front left (steering)
        (Vec3::new(0.5, -0.25, 1.3), Quat::from_axis_angle(Vec3::Z, -PI / 2.0), true),
        // Rear left
        (Vec3::new(0.5, -0.25, -1.3), Quat::from_axis_angle(Vec3::Z, -PI / 2.0), false),
        // Front right (steering)
        (Vec3::new(-0.5, -0.25, 1.3), Quat::from_axis_angle(Vec3::Z, PI / 2.0), true),
        // Rear right
        (Vec3::new(-0.5, -0.25, -1.3), Quat::from_axis_angle(Vec3::Z, PI / 2.0), false),
    ];

    let car_entity = commands
        .spawn((
            Car,
            RigidBody::Kinematic,
            car_collider,
            TransformInterpolation,
            SceneRoot(car),
            car_transform,
            EgoControl::default(),
            EgoState::default(),
        ))
        .id();

    for (pos, rot, is_steering) in wheels {
        let init_rotation = InitWheelRotation(rot);
        let mut entity = commands.spawn((
            SceneRoot(wheel.clone()),
            RollingWheel,
            init_rotation,
            Transform::from_rotation(rot).with_scale(wheel_scale).with_translation(pos),
        ));

        if is_steering {
            entity.insert(SteeringWheel);
        }

        let wheel_entity = entity.id();
        commands.entity(car_entity).add_child(wheel_entity);
    }
}

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct GroundPlane;

fn spawn_obstacle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let obstacle_pos = Vec3::new(10.0, 0.0, 1.0);
    let obstacle_collider = Collider::cuboid(5.0, 2.0, 1.0); // Match car dimensions
    let obstacle_density = ColliderDensity(100.0);

    commands.spawn((
        Obstacle,
        RigidBody::Dynamic,
        obstacle_collider,
        obstacle_density,
        Mesh3d(meshes.add(Cuboid::new(5.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(RED))),
        Transform::from_translation(obstacle_pos),
    ));

    // Ground config - rectangular grid
    let ground_size = (200.0, 200.0); // width, height
    let ground_pos = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn((
        GroundPlane,
        RigidBody::Static,
        ColliderConstructor::TrimeshFromMesh,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::new(ground_size.0, ground_size.1)))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(GRAY_100))),
        Transform::from_translation(ground_pos),
    ));
}

fn draw_ground_grid(gizmos: &mut GizmoAsset) {
    let grid_size = 200.0;
    let grid_spacing = 10.0;
    let half_size = grid_size / 2.0;

    // Draw grid lines along X axis
    for i in (-half_size as i32..=half_size as i32).step_by(grid_spacing as usize) {
        let y = i as f32;
        gizmos.line(
            Vec3::new(-half_size, y, 0.0),
            Vec3::new(half_size, y, 0.0),
            Color::srgba(0.5, 0.5, 0.5, 0.3),
        );
    }

    // Draw grid lines along Y axis
    for i in (-half_size as i32..=half_size as i32).step_by(grid_spacing as usize) {
        let x = i as f32;
        gizmos.line(
            Vec3::new(x, -half_size, 0.0),
            Vec3::new(x, half_size, 0.0),
            Color::srgba(0.5, 0.5, 0.5, 0.3),
        );
    }

    // Draw center axes (thicker and brighter)
    gizmos.line(
        Vec3::new(-half_size, 0.0, 0.0),
        Vec3::new(half_size, 0.0, 0.0),
        Color::srgba(0.3, 0.3, 0.3, 0.5),
    );
    gizmos.line(
        Vec3::new(0.0, -half_size, 0.0),
        Vec3::new(0.0, half_size, 0.0),
        Color::srgba(0.3, 0.3, 0.3, 0.5),
    );
}
