use super::responsive::{ResponsiveFontSize, ResponsiveInfo};
use crate::theme::KonnektorenTheme;
use bevy::prelude::*;
use bevy_egui::egui;

/// A reusable button widget styled with the Konnektoren theme.
pub struct ThemedButton<'a> {
    pub label: &'a str,
    pub theme: &'a KonnektorenTheme,
    pub enabled: bool,
    pub width: Option<f32>,
    pub opacity: f32,
    pub responsive_info: Option<&'a ResponsiveInfo>,
    pub custom_style: Option<Box<dyn FnOnce(egui::Button<'a>) -> egui::Button<'a> + 'a>>,
}

impl<'a> ThemedButton<'a> {
    pub fn new(label: &'a str, theme: &'a KonnektorenTheme) -> Self {
        Self {
            label,
            theme,
            enabled: true,
            width: None,
            opacity: 1.0,
            responsive_info: None,
            custom_style: None,
        }
    }

    pub fn responsive(mut self, responsive_info: &'a ResponsiveInfo) -> Self {
        self.responsive_info = Some(responsive_info);
        self
    }

    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Apply custom styling to the button
    pub fn with_style<F>(mut self, style_fn: F) -> Self
    where
        F: FnOnce(egui::Button<'a>) -> egui::Button<'a> + 'a,
    {
        self.custom_style = Some(Box::new(style_fn));
        self
    }
}

impl<'a> egui::Widget for ThemedButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.show(ui)
    }
}

impl<'a> ThemedButton<'a> {
    /// Show the button
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        // Determine font size and minimum size based on responsiveness
        let (font_size, min_height, min_width) = if let Some(responsive_info) = self.responsive_info
        {
            let font_size = responsive_info.font_size(ResponsiveFontSize::Medium);
            let min_height = if responsive_info.is_mobile() {
                44.0
            } else {
                32.0
            };
            let min_width = if responsive_info.is_mobile() {
                120.0
            } else {
                80.0
            };
            (font_size, min_height, min_width)
        } else {
            (18.0, 32.0, 80.0)
        };

        let mut button = egui::Button::new(
            egui::RichText::new(self.label)
                .color(self.theme.primary_content.linear_multiply(self.opacity))
                .size(font_size),
        )
        .fill(self.theme.primary.linear_multiply(self.opacity));

        // Apply custom styling if provided
        if let Some(style_fn) = self.custom_style {
            button = style_fn(button);
        }

        // Set minimum size for touch targets on mobile
        let final_width = self.width.unwrap_or(min_width).max(min_width);
        button = button.min_size(egui::vec2(final_width, min_height));

        ui.add_enabled(self.enabled, button)
    }

    /// Get the configured font size for external use
    pub fn get_font_size(&self) -> f32 {
        if let Some(responsive_info) = self.responsive_info {
            responsive_info.font_size(ResponsiveFontSize::Medium)
        } else {
            18.0
        }
    }

    /// Get the configured minimum dimensions for external use
    pub fn get_min_dimensions(&self) -> (f32, f32) {
        if let Some(responsive_info) = self.responsive_info {
            let min_height = if responsive_info.is_mobile() {
                44.0
            } else {
                32.0
            };
            let min_width = if responsive_info.is_mobile() {
                120.0
            } else {
                80.0
            };
            (min_width, min_height)
        } else {
            (80.0, 32.0)
        }
    }
}

/// Responsive text widget that adjusts size based on device type
pub struct ResponsiveText<'a> {
    pub text: &'a str,
    pub font_size_type: ResponsiveFontSize,
    pub color: egui::Color32,
    pub responsive_info: Option<&'a ResponsiveInfo>,
    pub strong: bool,
}

impl<'a> ResponsiveText<'a> {
    pub fn new(text: &'a str, font_size_type: ResponsiveFontSize, color: egui::Color32) -> Self {
        Self {
            text,
            font_size_type,
            color,
            responsive_info: None,
            strong: false,
        }
    }

    pub fn responsive(mut self, responsive_info: &'a ResponsiveInfo) -> Self {
        self.responsive_info = Some(responsive_info);
        self
    }

    pub fn strong(mut self) -> Self {
        self.strong = true;
        self
    }
}

impl<'a> egui::Widget for ResponsiveText<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let font_size = if let Some(responsive_info) = self.responsive_info {
            responsive_info.font_size(self.font_size_type)
        } else {
            match self.font_size_type {
                ResponsiveFontSize::Small => 14.0,
                ResponsiveFontSize::Medium => 18.0,
                ResponsiveFontSize::Large => 24.0,
                ResponsiveFontSize::Header => 32.0,
                ResponsiveFontSize::Title => 28.0,
            }
        };

        let mut rich_text = egui::RichText::new(self.text)
            .size(font_size)
            .color(self.color);

        if self.strong {
            rich_text = rich_text.strong();
        }

        ui.label(rich_text)
    }
}

/// A reusable egui widget for displaying a loading spinner.
pub struct SpinnerWidget<'a> {
    pub theme: &'a KonnektorenTheme,
    pub size: f32,
    pub responsive_info: Option<&'a ResponsiveInfo>,
}

impl<'a> SpinnerWidget<'a> {
    pub fn new(theme: &'a KonnektorenTheme, size: f32) -> Self {
        Self {
            theme,
            size,
            responsive_info: None,
        }
    }

    pub fn responsive(mut self, responsive_info: &'a ResponsiveInfo) -> Self {
        self.responsive_info = Some(responsive_info);
        self
    }
}

impl<'a> egui::Widget for SpinnerWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Adjust spinner size based on device type
        let final_size = if let Some(responsive_info) = self.responsive_info {
            let scale_factor = match responsive_info.device_type {
                crate::ui::responsive::DeviceType::Mobile => 1.2,
                crate::ui::responsive::DeviceType::Tablet => 1.1,
                crate::ui::responsive::DeviceType::Desktop => 1.0,
            };
            self.size * scale_factor
        } else {
            self.size
        };

        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(final_size, final_size), egui::Sense::hover());
        let center = rect.center();

        let time = ui.input(|i| i.time);
        let angle = time % 2.0 * std::f64::consts::PI;
        let points = 8;
        let radius = final_size * 0.375;

        let painter = ui.painter();

        for i in 0..points {
            let phase = angle + i as f64 * 2.0 * std::f64::consts::PI / points as f64;
            let point_distance = radius * 0.7;
            let pos = egui::pos2(
                center.x + (point_distance * phase.cos() as f32),
                center.y + (point_distance * phase.sin() as f32),
            );

            let alpha = ((1.0 - (i as f64 / points as f64)) * 0.8 + 0.2) as f32;
            let color = self.theme.primary.linear_multiply(alpha);

            painter.circle_filled(pos, final_size * 0.075, color);
        }

        response
    }
}
