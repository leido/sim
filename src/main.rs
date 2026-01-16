use avian3d::prelude::*;

use bevy::asset::{AssetMetaCheck, AssetMode};
use bevy::log::Level;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod camera;
mod car_dynamics;
mod init;
mod panel;
mod sound;
mod utils;

mod input;

// mod usb_cam;

fn main() {
    // Enable detailed logging for debugging
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // provide the ID selector string here
                        canvas: Some("#wasm-canvas".into()),
                        fit_canvas_to_parent: true,
                        // ... any other window properties ...
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    level: Level::INFO,
                    filter: "wgpu=warn".to_string(),
                    ..default()
                })
                .set(AssetPlugin {
                    mode: AssetMode::Unprocessed,
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            PhysicsPlugins::default(),
            // debug ui
            // PhysicsDebugPlugin,

            // diagnostics ui
            // PhysicsDiagnosticsPlugin,
            // PhysicsDiagnosticsUiPlugin
        ))
        // .insert_gizmo_config(
        //     PhysicsGizmos {
        //         aabb_color: Some(Color::WHITE),
        //         ..default()
        //     },
        //     GizmoConfig::default(),
        // )
        .add_plugins(EguiPlugin::default())
        .add_plugins(init::InitPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(sound::SoundPlugin)
        .add_plugins(panel::PanelPlugin)
        .add_plugins(car_dynamics::CarDynamicsPlugin)
        .add_plugins(input::InputPlugin)
        // .add_plugins(usb_cam::UsbCamPlugin)
        .run();
}
