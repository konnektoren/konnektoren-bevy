use bevy::prelude::*;
use bevy_egui::egui::{self, Widget};
use konnektoren_bevy::prelude::*;

// Example extension widget for general demo
pub fn custom_about_widget(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
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
pub fn game_specific_widget(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
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
pub fn render_custom_section(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
) {
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
