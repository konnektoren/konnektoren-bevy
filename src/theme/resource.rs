use bevy::prelude::*;
use bevy_egui::egui;

/// Shared theme resource that works for both Bevy UI and egui
#[derive(Resource, Clone)]
pub struct KonnektorenTheme {
    // Base colors
    pub base_100: egui::Color32,
    pub base_200: egui::Color32,
    pub base_300: egui::Color32,
    pub base_content: egui::Color32,

    // Primary colors
    pub primary: egui::Color32,
    pub primary_content: egui::Color32,

    // Secondary colors
    pub secondary: egui::Color32,
    pub secondary_content: egui::Color32,

    // Accent colors
    pub accent: egui::Color32,
    pub accent_content: egui::Color32,

    // Status colors
    pub info: egui::Color32,
    pub success: egui::Color32,
    pub warning: egui::Color32,
    pub error: egui::Color32,
    pub error_content: egui::Color32,

    // UI elements
    pub radius: u8,
    pub border_width: f32,
}

impl Default for KonnektorenTheme {
    fn default() -> Self {
        Self {
            // Base colors
            base_100: egui::Color32::from_rgb(246, 246, 246),
            base_200: egui::Color32::from_rgb(240, 240, 240),
            base_300: egui::Color32::from_rgb(232, 232, 232),
            base_content: egui::Color32::from_rgb(41, 41, 41),

            // Primary color - orange FF8C00
            primary: egui::Color32::from_rgb(255, 140, 0),
            primary_content: egui::Color32::from_rgb(255, 255, 255),

            // Secondary color - purple 8A2BE2
            secondary: egui::Color32::from_rgb(138, 43, 226),
            secondary_content: egui::Color32::from_rgb(255, 255, 255),

            // Accent color - gray 808080
            accent: egui::Color32::from_rgb(128, 128, 128),
            accent_content: egui::Color32::from_rgb(255, 255, 255),

            // Status colors
            info: egui::Color32::from_rgb(23, 162, 184),
            success: egui::Color32::from_rgb(40, 167, 69),
            warning: egui::Color32::from_rgb(255, 193, 7),
            error: egui::Color32::from_rgb(220, 53, 69),
            error_content: egui::Color32::WHITE,

            radius: 8,
            border_width: 1.0,
        }
    }
}

impl KonnektorenTheme {
    /// Create a new theme with custom colors
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a theme builder for customization
    pub fn builder() -> KonnektorenThemeBuilder {
        KonnektorenThemeBuilder::new()
    }

    /// Convert Bevy Color to egui Color32
    pub fn bevy_to_egui(color: Color) -> egui::Color32 {
        let rgba = color.to_srgba();
        egui::Color32::from_rgba_unmultiplied(
            (rgba.red * 255.0) as u8,
            (rgba.green * 255.0) as u8,
            (rgba.blue * 255.0) as u8,
            (rgba.alpha * 255.0) as u8,
        )
    }

    /// Get Bevy-compatible colors
    pub fn primary_bevy(&self) -> Color {
        Color::srgb(
            self.primary.r() as f32 / 255.0,
            self.primary.g() as f32 / 255.0,
            self.primary.b() as f32 / 255.0,
        )
    }

    pub fn secondary_bevy(&self) -> Color {
        Color::srgb(
            self.secondary.r() as f32 / 255.0,
            self.secondary.g() as f32 / 255.0,
            self.secondary.b() as f32 / 255.0,
        )
    }

    pub fn success_bevy(&self) -> Color {
        Color::srgb(
            self.success.r() as f32 / 255.0,
            self.success.g() as f32 / 255.0,
            self.success.b() as f32 / 255.0,
        )
    }

    pub fn error_bevy(&self) -> Color {
        Color::srgb(
            self.error.r() as f32 / 255.0,
            self.error.g() as f32 / 255.0,
            self.error.b() as f32 / 255.0,
        )
    }
}

/// Builder for creating custom themes
pub struct KonnektorenThemeBuilder {
    theme: KonnektorenTheme,
}

impl KonnektorenThemeBuilder {
    pub fn new() -> Self {
        Self {
            theme: KonnektorenTheme::default(),
        }
    }

    pub fn primary(mut self, color: egui::Color32) -> Self {
        self.theme.primary = color;
        self
    }

    pub fn secondary(mut self, color: egui::Color32) -> Self {
        self.theme.secondary = color;
        self
    }

    pub fn accent(mut self, color: egui::Color32) -> Self {
        self.theme.accent = color;
        self
    }

    pub fn success(mut self, color: egui::Color32) -> Self {
        self.theme.success = color;
        self
    }

    pub fn error(mut self, color: egui::Color32) -> Self {
        self.theme.error = color;
        self
    }

    pub fn radius(mut self, radius: u8) -> Self {
        self.theme.radius = radius;
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        self.theme.border_width = width;
        self
    }

    pub fn build(self) -> KonnektorenTheme {
        self.theme
    }
}

impl Default for KonnektorenThemeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
