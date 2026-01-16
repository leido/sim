use bevy::prelude::*;
use bevy_egui::egui::TextWrapMode;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_6, PI, TAU};

use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};

use crate::camera::CameraMode;
use crate::car_dynamics::{Car, EgoControl, EgoState};

#[derive(Resource, Default)]
pub struct DebugPanelVisible(bool);

#[derive(Resource, Default)]
pub struct HelpMenuVisible(bool);

struct Speedometer {
    radius: f32,
    v: f32, // km/h
    s: f32, // km
}

impl Speedometer {
    fn new(radius: f32, v: f32, s: f32) -> Self {
        Self {
            radius,
            v: v * 3.6,
            s: s / 1000.0,
        }
    }
}

impl egui::Widget for Speedometer {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(self.radius * 2.0), egui::Sense::hover());
        let painter = ui.painter();
        let max_speed = 240.0; // km/h
        let center = rect.center();
        let radius = self.radius;
        let speed = self.v;
        let mileage = self.s;
        painter.circle(
            center,
            radius,
            egui::Color32::from_rgb(30, 30, 40),
            egui::Stroke::new(5.0, egui::Color32::from_rgb(80, 80, 100)),
        );
        draw_speed_scale(painter, center, radius, max_speed);
        draw_speed_pointer(painter, center, radius, speed, max_speed);
        let speed_text = format!("{:.0}", speed);
        painter.text(
            center - egui::Vec2::new(0.0, radius * 0.2),
            egui::Align2::CENTER_CENTER,
            speed_text,
            egui::FontId::proportional(radius * 0.2),
            egui::Color32::WHITE,
        );
        let mileage_text = format!("{:.1} km", mileage);
        painter.text(
            center + egui::Vec2::new(0.0, radius * 0.2),
            egui::Align2::CENTER_CENTER,
            mileage_text,
            egui::FontId::proportional(radius * 0.3),
            egui::Color32::from_rgb(200, 200, 100),
        );
        response
    }
}

struct SteerWheel {
    radius: f32,
    angle: f32,
}

impl SteerWheel {
    fn new(radius: f32, angle: f32) -> Self {
        Self { radius, angle }
    }
}

impl egui::Widget for SteerWheel {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(self.radius * 2.0), egui::Sense::hover());
        let painter = ui.painter();
        let center = rect.center();
        painter.circle(
            center,
            self.radius,
            egui::Color32::from_gray(50),
            egui::Stroke::new(5.0, egui::Color32::from_gray(150)),
        );
        painter.circle(
            center,
            self.radius * 0.3,
            egui::Color32::from_gray(70),
            egui::Stroke::new(3.0, egui::Color32::from_gray(180)),
        );
        for i in 0..3 {
            let spoke_angle = -FRAC_PI_6 - self.angle + (i as f32) * TAU / 3.0;
            let end_point = center + egui::Vec2::angled(spoke_angle) * self.radius;
            painter.line_segment(
                [center, end_point],
                egui::Stroke::new(4.0, egui::Color32::from_gray(180)),
            );
        }
        let top_point = center + egui::Vec2::angled(-self.angle - FRAC_PI_2) * self.radius;
        painter.circle(
            top_point,
            self.radius * 0.1,
            egui::Color32::RED,
            egui::Stroke::NONE,
        );
        painter.text(
            center + egui::Vec2::new(0.0, self.radius * 0.2),
            egui::Align2::CENTER_CENTER,
            format!("{:.1}°", self.angle.to_degrees()),
            egui::FontId::proportional(self.radius * 0.3),
            egui::Color32::from_rgb(200, 200, 100),
        );
        response
    }
}

struct PedalIndicator {
    height: f32,
    width: f32,
    corner_radius: f32,
    value: f32,
    color: egui::Color32,
}

impl PedalIndicator {
    fn new(height: f32, value: f32, color: egui::Color32) -> Self {
        Self {
            height,
            width: height * 0.15,
            corner_radius: height * 0.075,
            color,
            value,
        }
    }
}

impl egui::Widget for PedalIndicator {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(
            egui::Vec2::new(self.width, self.height),
            egui::Sense::hover(),
        );
        let painter = ui.painter();
        let width = self.width;
        let length = self.height;
        let pos = rect.center();
        let corner_radius = self.corner_radius;
        let bg_rect = egui::Rect::from_center_size(pos, egui::Vec2::new(width, length));
        painter.rect(
            bg_rect,
            corner_radius,
            egui::Color32::from_gray(30),
            egui::Stroke::new(1.0, egui::Color32::from_gray(80)),
            egui::StrokeKind::Middle,
        );
        let fill_height = length * self.value;
        let fill_rect = egui::Rect::from_center_size(
            pos + egui::Vec2::new(0.0, (length - fill_height) * 0.5),
            egui::Vec2::new(width * 0.9, fill_height),
        );
        painter.rect(
            fill_rect,
            corner_radius,
            self.color,
            egui::Stroke::NONE,
            egui::StrokeKind::Middle,
        );
        let text_pos = pos + egui::Vec2::new(0.0, length * 0.5 + 20.0);
        painter.text(
            text_pos,
            egui::Align2::CENTER_CENTER,
            "",
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );
        response
    }
}

fn setup_debug_panel(mut commands: Commands) {
    commands.insert_resource(DebugPanelVisible(true));
    commands.insert_resource(HelpMenuVisible(false));
}

fn toggle_debug_panel(mut debug_panel: ResMut<DebugPanelVisible>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::KeyT) {
        debug_panel.0 = !debug_panel.0;
    }
}

fn toggle_help_menu(mut help_menu: ResMut<HelpMenuVisible>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::F1) || key.just_pressed(KeyCode::Slash) {
        help_menu.0 = !help_menu.0;
    }
}

fn update_debug_panel(
    mut contexts: EguiContexts,
    window: Single<&Window>,
    debug_panel: Res<DebugPanelVisible>,
    help_menu: Res<HelpMenuVisible>,
    query: Single<&EgoState, With<Car>>,
    control: Single<&EgoControl, With<Car>>,
    camera_mode: ResMut<CameraMode>,
) {
    let ctx = contexts.ctx_mut().unwrap();

    // Show help menu
    if help_menu.0 {
        egui::Window::new("Keyboard Shortcuts")
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                draw_help_menu(ui);
            });
    }

    if !debug_panel.0 {
        return;
    }

    // Dashboard at bottom center
    egui::Area::new(egui::Id::new("bottom"))
        .anchor(egui::Align2::CENTER_BOTTOM, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            let height = window.height() * 0.2;
            let radius = height / 2.0;
            ui.horizontal(|ui| {
                ui.add(Speedometer::new(radius, query.v, query.s));
                ui.add(PedalIndicator::new(
                    height,
                    control.brake,
                    egui::Color32::from_rgb(200, 80, 80),
                ));
                ui.add(PedalIndicator::new(
                    height,
                    control.throttle,
                    egui::Color32::from_rgb(80, 200, 80),
                ));
                ui.add(SteerWheel::new(radius, control.steer_wheel_angle));
            });
        });

    // Basic info at bottom right
    egui::Area::new(egui::Id::new("info_text"))
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-15.0, -10.0))
        .show(ctx, |ui| {
            draw_basic_info(ui, &query, &control, camera_mode);
        });
}

pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        assert!(app.is_plugin_added::<EguiPlugin>());
        app.init_resource::<DebugPanelVisible>()
            .add_systems(Startup, setup_debug_panel)
            .add_systems(
                EguiPrimaryContextPass,
                (update_debug_panel, toggle_debug_panel, toggle_help_menu),
            );
    }
}

fn draw_basic_info(
    ui: &mut egui::Ui,
    query: &EgoState,
    control: &EgoControl,
    mut camera_mode: ResMut<CameraMode>,
) {
    ui.vertical(|ui| {
        ui.style_mut().wrap_mode = Some(TextWrapMode::Extend);
        ui.label(format!("Position: ({:.2}, {:.2})", query.x, query.y));
        ui.label(format!(
            "Speed: {:.2} m/s ({:.1} km/h)",
            query.v,
            query.v * 3.6
        ));
        ui.label(format!("Theta: {:.2}°", query.yaw.to_degrees()));
        ui.label(format!("Trip Distance: {:.2}m", query.s));
        ui.label(format!("Throttle: {:.2}", control.throttle));
        ui.label(format!("Brake: {:.2}", control.brake));
        ui.label(format!(
            "Steer Angle: {:.2}°",
            control.front_wheel_angle.to_degrees()
        ));

        ui.add_space(5.0);
        ui.label("Camera Mode:");

        // Rebind to get mutable reference to the inner value
        let mode = camera_mode.as_mut();
        ui.radio_value(mode, CameraMode::FirstPersonView, "First Person");
        ui.radio_value(mode, CameraMode::ThirdPersonView, "Third Person");
        ui.radio_value(mode, CameraMode::OverShoulderView, "Over Shoulder");

        ui.add_space(5.0);
        ui.label(egui::RichText::new("[F1 or /] Help")
            .italics()
            .small()
            .color(egui::Color32::GRAY));
    });
}

fn draw_speed_scale(painter: &egui::Painter, center: egui::Pos2, radius: f32, max_speed: f32) {
    let major_step = 20.0; // Major tick interval
    let minor_step = 10.0; // Minor tick interval

    // Draw scale ring
    let inner_radius = radius * 0.85;
    let outer_radius = radius * 0.95;
    let minor_length = radius * 0.03;

    // Major ticks and numbers
    for speed in (0..=max_speed as i32).step_by(major_step as usize) {
        let angle = speed_to_angle(speed as f32, max_speed);
        let dir = egui::Vec2::angled(angle);

        // Major tick line
        painter.line_segment(
            [center + dir * inner_radius, center + dir * outer_radius],
            egui::Stroke::new(2.0, egui::Color32::WHITE),
        );

        // Tick numbers
        if speed % 40 == 0 {
            // Show number every 40
            let text_pos = center + dir * (inner_radius - radius * 0.1);
            painter.text(
                text_pos,
                egui::Align2::CENTER_CENTER,
                speed.to_string(),
                egui::FontId::proportional(radius * 0.15),
                egui::Color32::WHITE,
            );
        }
    }

    // Minor ticks
    for speed in (0..=max_speed as i32).step_by(minor_step as usize) {
        if speed % major_step as i32 == 0 {
            continue; // Skip major ticks
        }

        let angle = speed_to_angle(speed as f32, max_speed);
        let dir = egui::Vec2::angled(angle);

        painter.line_segment(
            [
                center + dir * (outer_radius - minor_length),
                center + dir * outer_radius,
            ],
            egui::Stroke::new(1.0, egui::Color32::from_gray(150)),
        );
    }
}

fn draw_speed_pointer(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    speed: f32,
    max_speed: f32,
) {
    let angle = speed_to_angle(speed, max_speed);
    let pointer_length = radius * 0.8;
    let pointer_width = radius * 0.03;

    // Pointer direction vector
    let dir = egui::Vec2::angled(angle);
    // Calculate direction of two points at pointer bottom (perpendicular to pointer)
    let perpendicular = dir.rot90(); // Get perpendicular direction

    // Pointer shape (triangle)
    let tip = center + dir * pointer_length;
    let base1 = center - dir * pointer_width * 0.5 + perpendicular * pointer_width;
    let base2 = center - dir * pointer_width * 0.5 - perpendicular * pointer_width;

    painter.add(egui::Shape::convex_polygon(
        vec![tip, base1, base2],
        egui::Color32::RED,
        egui::Stroke::new(1.0, egui::Color32::from_rgb(150, 0, 0)),
    ));

    // Pointer center circle
    painter.circle(
        center,
        pointer_width * 0.8,
        egui::Color32::from_rgb(80, 80, 80),
        egui::Stroke::new(1.0, egui::Color32::BLACK),
    );
}

// Convert speed to angle (speed 0 corresponds to -135 degrees, max speed corresponds to 135 degrees)
fn speed_to_angle(speed: f32, max_speed: f32) -> f32 {
    let progress = speed / max_speed;
    -PI * 0.75 + progress * PI * 0.75 * 2.0
}

fn draw_help_menu(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        ui.heading("Driving Controls");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("W").strong().color(egui::Color32::YELLOW));
            ui.label(": Throttle");
        });
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("S").strong().color(egui::Color32::YELLOW));
            ui.label(": Brake");
        });
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("A").strong().color(egui::Color32::YELLOW));
            ui.label(": Steer Left");
        });
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("D").strong().color(egui::Color32::YELLOW));
            ui.label(": Steer Right");
        });

        ui.add_space(10.0);

        ui.heading("Camera Controls");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("H").strong().color(egui::Color32::YELLOW));
            ui.label(": Cycle Camera Mode");
        });
        ui.label("  First Person → Third Person → Over Shoulder");
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("I / K").strong().color(egui::Color32::YELLOW));
            ui.label(": Pitch Up / Down");
        });
        ui.label("  Third Person View Only");
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("J / L").strong().color(egui::Color32::YELLOW));
            ui.label(": Yaw Left / Right");
        });
        ui.label("  Third Person View Only");

        ui.add_space(10.0);

        ui.heading("UI Controls");
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("T").strong().color(egui::Color32::YELLOW));
            ui.label(": Toggle Debug Panel");
        });
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("F1 / /").strong().color(egui::Color32::YELLOW));
            ui.label(": Toggle Help Menu");
        });

        ui.add_space(10.0);
        ui.separator();
        ui.label(egui::RichText::new("Tip: Gamepad Supported")
            .italics()
            .color(egui::Color32::GRAY));
    });
}
