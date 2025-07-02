use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use konnektoren_bevy::prelude::*;

mod demo;
mod handlers;
mod ui;
mod widgets;

use demo::*;
use handlers::*;
use ui::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Set asset folder to the workspace root assets folder
                    file_path: "../assets".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Konnektoren Bevy - Demo".into(),
                        resolution: (800.0, 600.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(KonnektorenThemePlugin)
        .add_plugins(UIPlugin)
        .add_plugins(ScreensPlugin)
        .add_plugins(InputPlugin)
        .add_systems(Startup, setup_demo)
        .add_systems(
            Update,
            (
                handle_splash_dismissed,
                handle_about_dismissed,
                handle_settings_events,
            ),
        )
        .add_systems(bevy_egui::EguiContextPass, demo_ui)
        .run();
}
