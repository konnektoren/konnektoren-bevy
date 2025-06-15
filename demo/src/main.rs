//! Full demo showcasing all konnektoren-bevy features

use bevy::prelude::*;
use bevy_egui::{egui::Widget, EguiPlugin};
use konnektoren_bevy::prelude::*;

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
        .add_systems(Startup, setup_demo)
        .add_systems(Update, handle_splash_dismissed)
        .add_systems(bevy_egui::EguiContextPass, demo_ui)
        .run();
}

#[derive(Component)]
struct DemoState {
    current_demo: DemoType,
    splash_shown: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DemoType {
    Splash,
    Complete,
}

fn setup_demo(mut commands: Commands) {
    // Initialize demo state
    commands.spawn(DemoState {
        current_demo: DemoType::Splash,
        splash_shown: false,
    });

    // Show initial splash screen
    commands.spawn_konnektoren_splash();
}

fn handle_splash_dismissed(
    mut splash_events: EventReader<SplashDismissed>,
    mut demo_query: Query<&mut DemoState>,
) {
    for _event in splash_events.read() {
        if let Ok(mut demo_state) = demo_query.single_mut() {
            demo_state.splash_shown = true;
            demo_state.current_demo = DemoType::Complete;
            info!("Splash dismissed, showing main demo");
        }
    }
}

fn demo_ui(
    mut contexts: bevy_egui::EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    demo_query: Query<&DemoState>,
    mut commands: Commands,
    active_splash_query: Query<Entity, With<ActiveSplash>>,
) {
    if let Ok(demo_state) = demo_query.single() {
        // Don't show UI if splash is active
        if !active_splash_query.is_empty() {
            return;
        }

        if !demo_state.splash_shown {
            return; // Wait for first splash to finish
        }

        bevy_egui::egui::SidePanel::left("demo_panel")
            .resizable(true)
            .default_width(250.0)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Konnektoren Bevy Demo");
                ui.separator();

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

                ui.separator();
                ui.label(format!(
                    "Screen: {}x{}",
                    responsive.screen_size.x, responsive.screen_size.y
                ));
                ui.label(format!("Device: {:?}", responsive.device_type));
                ui.label(format!("Orientation: {:?}", responsive.orientation));

                // Show if splash is active
                let splash_active = !active_splash_query.is_empty();
                ui.label(format!("Splash Active: {}", splash_active));
            });

        bevy_egui::egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            match demo_state.current_demo {
                DemoType::Complete => show_complete_demo(ui, &theme, &responsive),
                _ => {}
            }
        });
    }
}

fn show_complete_demo(
    ui: &mut bevy_egui::egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
    ui.heading("Welcome to Konnektoren Bevy!");

    ui.separator();

    // Theme demonstration
    ui.heading("Theme Colors");
    ui.horizontal(|ui| {
        ui.label("Primary:");
        ui.colored_label(theme.primary, "■ Primary Color");
    });
    ui.horizontal(|ui| {
        ui.label("Secondary:");
        ui.colored_label(theme.secondary, "■ Secondary Color");
    });
    ui.horizontal(|ui| {
        ui.label("Accent:");
        ui.colored_label(theme.accent, "■ Accent Color");
    });

    ui.separator();

    // Button demonstrations
    ui.heading("Themed Buttons");
    ui.horizontal(|ui| {
        if ThemedButton::new("Primary Button", theme)
            .responsive(responsive)
            .show(ui)
            .clicked()
        {
            info!("Primary button clicked!");
        }

        if ThemedButton::new("Disabled Button", theme)
            .responsive(responsive)
            .enabled(false)
            .opacity(0.5)
            .show(ui)
            .clicked()
        {
            info!("This shouldn't happen!");
        }
    });

    ui.separator();

    // Responsive text
    ui.heading("Responsive Text");
    ResponsiveText::new("Small Text", ResponsiveFontSize::Small, theme.base_content)
        .responsive(responsive)
        .ui(ui);
    ResponsiveText::new("Medium Text", ResponsiveFontSize::Medium, theme.primary)
        .responsive(responsive)
        .strong()
        .ui(ui);
    ResponsiveText::new("Large Text", ResponsiveFontSize::Large, theme.secondary)
        .responsive(responsive)
        .ui(ui);

    ui.separator();

    // Spinner widget
    ui.heading("Loading Spinner");
    SpinnerWidget::new(theme, 32.0)
        .responsive(responsive)
        .ui(ui);

    ui.separator();

    // Instructions
    ui.heading("Instructions");
    ui.label("• Click any splash button in the left panel to see different splash screen styles");
    ui.label("• Splash screens can be dismissed by clicking the button (if shown) or pressing Space/Enter/Escape");
    ui.label("• The UI is hidden while splash screens are active");
    ui.label("• Resize the window to see responsive design in action");
}
