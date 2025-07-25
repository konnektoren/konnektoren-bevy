use crate::widgets::*;
use bevy::prelude::*;
use bevy_egui::egui::{self, Widget};
use konnektoren_bevy::prelude::*;

pub fn show_complete_demo(
    ui: &mut egui::Ui,
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
    ui.label("• Click any settings button to see different settings screen layouts");
    ui.label("• Splash screens can be dismissed by clicking the button (if shown) or pressing Space/Enter/Escape");
    ui.label("• About and Settings screens can be dismissed by clicking the back button or pressing Escape");
    ui.label("• The UI is hidden while any overlay screen is active");
    ui.label("• Resize the window to see responsive design in action");
    ui.label("• Settings screens adapt to mobile and desktop layouts automatically");

    ui.separator();

    // Feature showcase
    ui.heading("Library Features");
    ui.label("✅ Splash screens with image loading, animations, and customizable content");
    ui.label("✅ About screens with extensible sections and custom widgets");
    ui.label("✅ Settings screens with various control types and responsive layouts");
    ui.label("✅ Responsive design that adapts to different screen sizes");
    ui.label("✅ Consistent theming system with customizable colors");
    ui.label("✅ Reusable UI widgets (buttons, text, spinners)");
    ui.label("✅ Event-driven architecture for easy integration");
    ui.label("✅ Pre-built setting types: toggles, sliders, button groups, custom");
    ui.label("✅ Common setting sections for audio, graphics, gameplay, and input");
}

pub fn spawn_custom_about(commands: &mut Commands) {
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

pub fn spawn_demo_credits(commands: &mut Commands) {
    commands.spawn_credits(
        CreditsConfig::new("Konnektoren Bevy Demo")
            .with_subtitle("Demo Credits")
            .add_team_member("Jane Example", "Demo Developer")
            .add_team_member("John Example", "UI Design")
            .add_asset("Demo Logo", "CC0 Example Asset")
            .add_special_thanks("You!", "For trying the demo")
            .add_technology("Bevy", "Game engine")
            .add_technology("egui", "Immediate mode GUI")
            .with_dismiss_button_text("← Back"),
    );
}

pub fn spawn_educational_about(commands: &mut Commands) {
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

pub fn spawn_custom_settings(commands: &mut Commands, responsive: &ResponsiveInfo) {
    let custom_config = SettingsScreenConfig::new("Custom Settings")
        .mobile_layout(responsive.is_mobile())
        .add_section(SettingsSection::audio_section())
        .add_section(SettingsSection::input_section())
        .add_section(
            SettingsSection::new("Demo Settings")
                .add_setting(ScreenSettingsItem::selection(
                    "demo_mode",
                    "Demo Mode",
                    vec![
                        "Beginner".to_string(),
                        "Intermediate".to_string(),
                        "Advanced".to_string(),
                    ],
                    1,
                ))
                .add_setting(ScreenSettingsItem::toggle("auto_demo", "Auto Demo", false))
                .add_setting(ScreenSettingsItem::slider(
                    "demo_speed",
                    "Demo Speed",
                    1.0,
                    0.5,
                    2.0,
                    0.1,
                )),
        )
        .add_section(
            SettingsSection::new("Advanced Options")
                .add_setting(ScreenSettingsItem::toggle(
                    "debug_mode",
                    "Debug Mode",
                    false,
                ))
                .add_setting(ScreenSettingsItem::selection(
                    "log_level",
                    "Log Level",
                    vec![
                        "Error".to_string(),
                        "Warn".to_string(),
                        "Info".to_string(),
                        "Debug".to_string(),
                        "Trace".to_string(),
                    ],
                    2,
                ))
                .add_setting(ScreenSettingsItem::slider(
                    "max_fps", "Max FPS", 60.0, 30.0, 144.0, 1.0,
                )),
        );

    commands.spawn_settings_screen(custom_config);
}

pub fn spawn_mobile_settings(commands: &mut Commands) {
    let mobile_config = SettingsScreenConfig::new("Mobile Settings")
        .mobile_layout(true)
        .add_section(SettingsSection::audio_section())
        .add_section(
            SettingsSection::new("Touch Controls")
                .add_setting(ScreenSettingsItem::slider(
                    "touch_sensitivity",
                    "Touch Sensitivity",
                    1.0,
                    0.5,
                    2.0,
                    0.1,
                ))
                .add_setting(ScreenSettingsItem::toggle(
                    "haptic_feedback",
                    "Haptic Feedback",
                    true,
                ))
                .add_setting(ScreenSettingsItem::toggle(
                    "gesture_controls",
                    "Gesture Controls",
                    true,
                )),
        );

    commands.spawn_settings_screen(mobile_config);
}
