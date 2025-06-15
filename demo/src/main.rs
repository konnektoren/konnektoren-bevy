use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Widget},
    EguiPlugin,
};
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
        .add_systems(Update, (handle_splash_dismissed, handle_about_dismissed))
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

fn handle_about_dismissed(mut about_events: EventReader<AboutDismissed>) {
    for event in about_events.read() {
        info!("About screen dismissed for entity {:?}", event.entity);
    }
}

#[allow(clippy::too_many_arguments)]
fn demo_ui(
    mut contexts: bevy_egui::EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    demo_query: Query<&DemoState>,
    mut commands: Commands,
    active_splash_query: Query<Entity, With<ActiveSplash>>,
    active_about_query: Query<Entity, With<ActiveAbout>>,
    existing_about_configs: Query<Entity, With<AboutConfig>>,
) {
    if let Ok(demo_state) = demo_query.single() {
        // Don't show UI if splash or about is active
        if !active_splash_query.is_empty() || !active_about_query.is_empty() {
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
                ui.heading("About Screen Examples");

                if ui.button("Simple About").clicked() {
                    info!("Spawning simple about screen");
                    cleanup_existing_about_screens(&mut commands, &existing_about_configs);
                    commands.spawn_simple_about("My App");
                }

                if ui.button("Game About").clicked() {
                    info!("Spawning game about screen");
                    cleanup_existing_about_screens(&mut commands, &existing_about_configs);
                    commands.spawn_game_about("My Learning Game");
                }

                if ui.button("Custom About").clicked() {
                    info!("Spawning custom about screen");
                    cleanup_existing_about_screens(&mut commands, &existing_about_configs);

                    let about_config = AboutConfig::new("Advanced Demo")
                        .with_subtitle("Showcase Application")
                        .with_version("v1.0.0")
                        .with_description("This is a comprehensive demonstration of the about screen functionality with custom extensions.")
                        .add_feature("🚀 Advanced Features", "Showcasing all available customization options")
                        .add_feature("🎨 Custom Theming", "Responsive design with consistent styling")
                        .add_feature("🔧 Extensible", "Add your own widgets and sections")
                        .add_website("Demo Site", "Visit our demo website", "https://example.com")
                        .add_website("GitHub", "Check out the source code", "https://github.com/konnektoren/konnektoren-bevy")
                        .with_extension_widget(custom_about_widget)
                        .add_custom_section("Custom Section", render_custom_section);

                    commands.spawn_about(about_config);
                }

                if ui.button("Educational Game About").clicked() {
                    info!("Spawning educational game about screen");
                    cleanup_existing_about_screens(&mut commands, &existing_about_configs);

                    let game_config = AboutConfig::for_game("German Grammar Master")
                        .with_subtitle("Interactive Language Learning")
                        .with_version("Beta v0.2.1")
                        .with_description("Master German grammar through engaging gameplay! Learn connecting words (Konnektoren) and improve your language skills with interactive challenges.")
                        .with_why_choose_us("Our game combines proven educational methods with modern game design to make learning German grammar both effective and enjoyable.")
                        .add_feature("🎯 Targeted Learning", "Focus specifically on German connecting words and grammar")
                        .add_feature("🏆 Achievement System", "Unlock rewards as you progress through lessons")
                        .add_feature("📈 Adaptive Difficulty", "Questions adjust to your skill level")
                        .add_feature("🌍 Multiple Game Modes", "Story mode, quick challenges, and practice sessions")
                        .add_website("Play Online", "Try the web version", "https://konnektoren.help")
                        .add_website("Download", "Get the desktop version", "https://github.com/konnektoren/konnektoren-game/releases")
                        .with_extension_widget(game_specific_widget)
                        .with_status_message("🚧 Currently in Beta - New features added regularly!", egui::Color32::from_rgb(52, 152, 219));

                    commands.spawn_about(game_config);
                }

                ui.separator();
                ui.label(format!(
                    "Screen: {}x{}",
                    responsive.screen_size.x, responsive.screen_size.y
                ));
                ui.label(format!("Device: {:?}", responsive.device_type));
                ui.label(format!("Orientation: {:?}", responsive.orientation));

                // Show if screens are active
                let splash_active = !active_splash_query.is_empty();
                let about_active = !active_about_query.is_empty();
                ui.label(format!("Splash Active: {}", splash_active));
                ui.label(format!("About Active: {}", about_active));
            });

        bevy_egui::egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            if demo_state.current_demo == DemoType::Complete {
                show_complete_demo(ui, &theme, &responsive)
            }
        });
    }
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
    ui.label("• Click any about button to see different about screen configurations");
    ui.label("• Splash screens can be dismissed by clicking the button (if shown) or pressing Space/Enter/Escape");
    ui.label("• About screens can be dismissed by clicking the back button or pressing Escape");
    ui.label("• The UI is hidden while splash or about screens are active");
    ui.label("• Resize the window to see responsive design in action");
    ui.label("• Try the different about screen examples to see customization options");

    ui.separator();

    // Feature showcase
    ui.heading("Library Features");
    ui.label("✅ Splash screens with image loading, animations, and customizable content");
    ui.label("✅ About screens with extensible sections and custom widgets");
    ui.label("✅ Responsive design that adapts to different screen sizes");
    ui.label("✅ Consistent theming system with customizable colors");
    ui.label("✅ Reusable UI widgets (buttons, text, spinners)");
    ui.label("✅ Event-driven architecture for easy integration");
}

// Example extension widget for general demo
fn custom_about_widget(ui: &mut egui::Ui, theme: &KonnektorenTheme, responsive: &ResponsiveInfo) {
    ui.vertical_centered(|ui| {
        ResponsiveText::new("🎯 Custom Extension Widget", ResponsiveFontSize::Large, theme.accent)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(8.0);

        ResponsiveText::new(
            "This is a custom widget that can be added to extend the about screen with game-specific or app-specific content. You can add any egui widgets here!",
            ResponsiveFontSize::Medium,
            theme.base_content,
        )
        .responsive(responsive)
        .ui(ui);

        ui.add_space(12.0);

        // Example of adding interactive elements
        ui.horizontal(|ui| {
            if ThemedButton::new("Custom Action", theme)
                .responsive(responsive)
                .show(ui)
                .clicked()
            {
                info!("Custom widget button clicked!");
            }

            ui.add_space(8.0);

            // Example spinner
            SpinnerWidget::new(theme, 24.0)
                .responsive(responsive)
                .ui(ui);
        });
    });
}

// Example game-specific extension widget
fn game_specific_widget(ui: &mut egui::Ui, theme: &KonnektorenTheme, responsive: &ResponsiveInfo) {
    ui.vertical_centered(|ui| {
        ResponsiveText::new(
            "🎮 Game Statistics",
            ResponsiveFontSize::Large,
            theme.secondary,
        )
        .responsive(responsive)
        .strong()
        .ui(ui);

        ui.add_space(8.0);

        // Mock game statistics
        let stats = [
            ("Total Players", "10,234"),
            ("Lessons Completed", "45,678"),
            ("Average Score", "87%"),
            ("Languages Supported", "German"),
        ];

        for (label, value) in stats {
            ui.horizontal(|ui| {
                ResponsiveText::new(label, ResponsiveFontSize::Medium, theme.base_content)
                    .responsive(responsive)
                    .ui(ui);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ResponsiveText::new(value, ResponsiveFontSize::Medium, theme.primary)
                        .responsive(responsive)
                        .strong()
                        .ui(ui);
                });
            });
            ui.add_space(4.0);
        }

        ui.add_space(12.0);

        ResponsiveText::new(
            "Join thousands of learners improving their German skills!",
            ResponsiveFontSize::Medium,
            theme.success,
        )
        .responsive(responsive)
        .ui(ui);
    });
}

// Example custom section
fn render_custom_section(ui: &mut egui::Ui, theme: &KonnektorenTheme, responsive: &ResponsiveInfo) {
    ResponsiveText::new(
        "This is a completely custom section that can contain any content you need for your specific application. You have full control over the layout and styling.",
        ResponsiveFontSize::Medium,
        theme.base_content,
    )
    .responsive(responsive)
    .ui(ui);

    ui.add_space(12.0);

    // Example of a custom layout
    egui::Grid::new("custom_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .show(ui, |ui| {
            ui.label("Framework:");
            ResponsiveText::new("Bevy Engine", ResponsiveFontSize::Medium, theme.primary)
                .responsive(responsive)
                .ui(ui);
            ui.end_row();

            ui.label("UI Library:");
            ResponsiveText::new("egui", ResponsiveFontSize::Medium, theme.primary)
                .responsive(responsive)
                .ui(ui);
            ui.end_row();

            ui.label("Language:");
            ResponsiveText::new("Rust", ResponsiveFontSize::Medium, theme.primary)
                .responsive(responsive)
                .ui(ui);
            ui.end_row();
        });
}
