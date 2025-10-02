use super::resource::KonnektorenTheme;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub struct EguiThemePlugin;

impl Plugin for EguiThemePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KonnektorenTheme>()
            .add_systems(Update, apply_theme);
    }
}

/// System to apply the theme to egui
fn apply_theme(mut contexts: EguiContexts, theme: Res<KonnektorenTheme>) {
    // Handle the Result properly
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    // Calculate half radius for widgets
    let half_radius = if theme.radius > 1 {
        theme.radius / 2
    } else {
        theme.radius
    };

    // Create a visuals object with our custom style
    let mut visuals = egui::Visuals {
        window_corner_radius: theme.radius.into(),
        window_shadow: egui::epaint::Shadow {
            color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 96),
            ..Default::default()
        },
        window_fill: theme.base_100,
        panel_fill: theme.base_100,

        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: theme.base_200,
                bg_stroke: egui::Stroke::new(theme.border_width, theme.base_300),
                corner_radius: half_radius.into(),
                fg_stroke: egui::Stroke::new(1.0, theme.base_content),
                weak_bg_fill: theme.base_200,
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: theme.base_200,
                bg_stroke: egui::Stroke::new(theme.border_width, theme.accent),
                corner_radius: half_radius.into(),
                fg_stroke: egui::Stroke::new(1.0, theme.base_content),
                weak_bg_fill: theme.base_200,
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: theme.base_300,
                bg_stroke: egui::Stroke::new(theme.border_width, theme.primary),
                corner_radius: half_radius.into(),
                fg_stroke: egui::Stroke::new(1.5, theme.primary),
                weak_bg_fill: theme.base_300,
                expansion: 1.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: theme.primary,
                bg_stroke: egui::Stroke::new(theme.border_width, theme.primary),
                corner_radius: half_radius.into(),
                fg_stroke: egui::Stroke::new(1.5, theme.primary_content),
                weak_bg_fill: theme.primary.linear_multiply(0.9),
                expansion: 1.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: theme.secondary,
                bg_stroke: egui::Stroke::new(theme.border_width, theme.secondary),
                corner_radius: half_radius.into(),
                fg_stroke: egui::Stroke::new(1.5, theme.secondary_content),
                weak_bg_fill: theme.secondary.linear_multiply(0.9),
                expansion: 0.0,
            },
        },

        selection: egui::style::Selection {
            bg_fill: theme.primary.linear_multiply(0.4),
            stroke: egui::Stroke::new(1.0, theme.primary),
        },

        override_text_color: Some(theme.base_content),
        dark_mode: false,
        ..Default::default()
    };

    visuals.button_frame = true;
    visuals.collapsing_header_frame = true;

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(16);
    style.spacing.button_padding = egui::vec2(8.0, 4.0);

    ctx.set_style(style);
}
