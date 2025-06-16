use super::screens::*;
use crate::demo::DemoState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use konnektoren_bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn render_side_panel(
    contexts: &mut EguiContexts,
    _theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    commands: &mut Commands,
    active_splash_query: &Query<Entity, With<ActiveSplash>>,
    active_about_query: &Query<Entity, With<ActiveAbout>>,
    active_settings_query: &Query<Entity, With<ActiveSettingsScreen>>, // Fixed
    existing_about_configs: &Query<Entity, With<AboutConfig>>,
    existing_settings_configs: &Query<Entity, With<SettingsScreenConfig>>,
) {
    egui::SidePanel::left("demo_panel")
        .resizable(true)
        .default_width(280.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Konnektoren Bevy Demo");
            ui.separator();

            render_splash_section(ui, commands);
            ui.separator();
            render_about_section(ui, commands, existing_about_configs);
            ui.separator();
            render_settings_section(ui, commands, responsive, existing_settings_configs);
            ui.separator();
            render_status_section(
                ui,
                responsive,
                active_splash_query,
                active_about_query,
                active_settings_query,
            );
        });
}

pub fn render_main_panel(
    contexts: &mut EguiContexts,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    _demo_state: &DemoState,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        show_complete_demo(ui, theme, responsive)
    });
}

fn render_splash_section(ui: &mut egui::Ui, commands: &mut Commands) {
    ui.heading("Splash Examples");

    if ui.button("Konnektoren Splash").clicked() {
        info!("Spawning Konnektoren splash");
        commands.spawn_konnektoren_splash();
    }

    if ui.button("Emoji Logo Splash").clicked() {
        info!("Spawning emoji splash");
        commands.spawn_splash(
            SplashConfig::new("Demo Splash")
                .with_emoji_logo("🚀")
                .with_subtitle("Reloading demo...")
                .with_duration(2.0),
        );
    }

    if ui.button("Text Logo Splash").clicked() {
        info!("Spawning text logo splash");
        commands.spawn_splash(
            SplashConfig::new("My Studio")
                .with_text_logo("MS")
                .with_subtitle("Presents...")
                .with_duration(2.5)
                .with_logo_size(1.5),
        );
    }

    if ui.button("Image Logo Splash").clicked() {
        info!("Spawning image splash");
        commands.spawn_splash(
            SplashConfig::new("Loading Assets")
                .with_image_logo("logo.png")
                .with_subtitle("Loading game assets...")
                .with_duration(3.0),
        );
    }

    if ui.button("Manual Dismiss Splash").clicked() {
        info!("Spawning manual dismiss splash");
        commands.spawn_splash(
            SplashConfig::new("Press to Continue")
                .with_emoji_logo("⏸️")
                .with_subtitle("Ready when you are!")
                .infinite()
                .with_button_text("Let's Go!"),
        );
    }

    if ui.button("Clean Splash (No Loading)").clicked() {
        info!("Spawning clean splash");
        commands.spawn_splash(
            SplashConfig::new("Clean Look")
                .with_text_logo("C")
                .with_subtitle("Minimalist design")
                .with_duration(2.0)
                .with_loading_indicator(false),
        );
    }
}

fn render_about_section(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    existing_about_configs: &Query<Entity, With<AboutConfig>>,
) {
    ui.heading("About Screen Examples");

    if ui.button("Simple About").clicked() {
        info!("Spawning simple about screen");
        cleanup_existing_about_screens(commands, existing_about_configs);
        commands.spawn_simple_about("My App");
    }

    if ui.button("Game About").clicked() {
        info!("Spawning game about screen");
        cleanup_existing_about_screens(commands, existing_about_configs);
        commands.spawn_game_about("My Learning Game");
    }

    if ui.button("Custom About").clicked() {
        info!("Spawning custom about screen");
        cleanup_existing_about_screens(commands, existing_about_configs);
        spawn_custom_about(commands);
    }

    if ui.button("Educational Game About").clicked() {
        info!("Spawning educational game about screen");
        cleanup_existing_about_screens(commands, existing_about_configs);
        spawn_educational_about(commands);
    }
}

fn render_settings_section(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    responsive: &ResponsiveInfo,
    existing_settings_configs: &Query<Entity, With<SettingsScreenConfig>>,
) {
    ui.heading("Settings Screen Examples");

    if ui.button("Simple Settings").clicked() {
        info!("Spawning simple settings screen");
        cleanup_existing_settings_screens(commands, existing_settings_configs);
        commands.spawn_simple_settings_screen("Basic Settings");
    }

    if ui.button("Game Settings").clicked() {
        info!("Spawning game settings screen");
        cleanup_existing_settings_screens(commands, existing_settings_configs);
        commands.spawn_settings_screen(SettingsScreenConfig::game_settings("Game Settings"));
    }

    if ui.button("Educational Settings").clicked() {
        info!("Spawning educational settings screen");
        cleanup_existing_settings_screens(commands, existing_settings_configs);
        commands.spawn_settings_screen(SettingsScreenConfig::educational_game("Learning Settings"));
    }

    if ui.button("Audio Only Settings").clicked() {
        info!("Spawning audio-only settings screen");
        cleanup_existing_settings_screens(commands, existing_settings_configs);
        commands.spawn_settings_screen(SettingsScreenConfig::audio_only("Audio Settings"));
    }

    if ui.button("Custom Settings").clicked() {
        info!("Spawning custom settings screen");
        cleanup_existing_settings_screens(commands, existing_settings_configs);
        spawn_custom_settings(commands, responsive);
    }

    if ui.button("Mobile Layout Settings").clicked() {
        info!("Spawning mobile layout settings screen");
        cleanup_existing_settings_screens(commands, existing_settings_configs);
        spawn_mobile_settings(commands);
    }
}

fn render_status_section(
    ui: &mut egui::Ui,
    responsive: &ResponsiveInfo,
    active_splash_query: &Query<Entity, With<ActiveSplash>>,
    active_about_query: &Query<Entity, With<ActiveAbout>>,
    active_settings_query: &Query<Entity, With<ActiveSettingsScreen>>, // Fixed
) {
    ui.heading("System Status");

    ui.label(format!(
        "Screen: {}x{}",
        responsive.screen_size.x, responsive.screen_size.y
    ));
    ui.label(format!("Device: {:?}", responsive.device_type));
    ui.label(format!("Orientation: {:?}", responsive.orientation));

    ui.add_space(10.0);

    // Show if screens are active
    let splash_active = !active_splash_query.is_empty();
    let about_active = !active_about_query.is_empty();
    let settings_active = !active_settings_query.is_empty();

    ui.label(format!("Splash Active: {}", splash_active));
    ui.label(format!("About Active: {}", about_active));
    ui.label(format!("Settings Active: {}", settings_active));
}

// Helper function to clean up existing about screens
fn cleanup_existing_about_screens(
    commands: &mut Commands,
    existing_about_configs: &Query<Entity, With<AboutConfig>>,
) {
    for entity in existing_about_configs.iter() {
        info!("Cleaning up existing about screen: {:?}", entity);
        commands.entity(entity).despawn();
    }
}

// Helper function to clean up existing settings screens
fn cleanup_existing_settings_screens(
    commands: &mut Commands,
    existing_settings_configs: &Query<Entity, With<SettingsScreenConfig>>,
) {
    for entity in existing_settings_configs.iter() {
        info!("Cleaning up existing settings screen: {:?}", entity);
        commands.entity(entity).despawn();
    }
}
