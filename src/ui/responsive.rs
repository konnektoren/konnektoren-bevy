use bevy::prelude::*;
use bevy_egui::egui;

/// Resource that tracks the current screen size and provides responsive utilities
#[derive(Resource)]
pub struct ResponsiveInfo {
    pub screen_size: Vec2,
    pub device_type: DeviceType,
    pub orientation: Orientation,
    pub scale_factor: f32,
}

impl Default for ResponsiveInfo {
    fn default() -> Self {
        // Initialize with reasonable defaults for desktop
        let mut info = Self {
            screen_size: Vec2::new(1024.0, 768.0),
            device_type: DeviceType::Desktop,
            orientation: Orientation::Landscape,
            scale_factor: 1.0,
        };
        // Update device type based on default screen size
        info.update(info.screen_size, info.scale_factor);

        info
    }
}

/// Device type based on screen dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceType {
    #[default]
    Desktop,
    Tablet,
    Mobile,
}

/// Screen orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Landscape,
    Portrait,
}

/// Responsive breakpoints (in logical pixels)
pub struct Breakpoints;

impl Breakpoints {
    pub const MOBILE_MAX: f32 = 480.0;
    pub const TABLET_MAX: f32 = 768.0;
    pub const DESKTOP_MIN: f32 = 769.0;
}

/// Responsive font size types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveFontSize {
    Small,
    Medium,
    Large,
    Header,
    Title,
}

/// Responsive spacing types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveSpacing {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

/// Responsive border radius types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveBorderRadius {
    None,
    Small,
    Medium,
    Large,
    XLarge,
}

/// Responsive margin types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponsiveMargin {
    None,
    Small,
    Medium,
    Large,
    XLarge,
}

impl ResponsiveInfo {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update responsive info based on window size
    pub fn update(&mut self, window_size: Vec2, scale_factor: f32) {
        self.screen_size = window_size;
        self.scale_factor = scale_factor;

        // Determine device type based on the smaller dimension
        let min_dimension = window_size.x.min(window_size.y);
        self.device_type = if min_dimension <= Breakpoints::MOBILE_MAX {
            DeviceType::Mobile
        } else if min_dimension <= Breakpoints::TABLET_MAX {
            DeviceType::Tablet
        } else {
            DeviceType::Desktop
        };

        // Determine orientation
        self.orientation = if window_size.x > window_size.y {
            Orientation::Landscape
        } else {
            Orientation::Portrait
        };
    }

    /// Get responsive font size
    pub fn font_size(&self, size_type: ResponsiveFontSize) -> f32 {
        let base_scale = match (self.device_type, self.orientation) {
            (DeviceType::Mobile, Orientation::Landscape) => 0.7,
            (DeviceType::Mobile, Orientation::Portrait) => 0.8,
            (DeviceType::Tablet, _) => 0.9,
            (DeviceType::Desktop, _) => 1.0,
        };

        let base_size = match size_type {
            ResponsiveFontSize::Small => 12.0,
            ResponsiveFontSize::Medium => 16.0,
            ResponsiveFontSize::Large => 20.0,
            ResponsiveFontSize::Header => 24.0,
            ResponsiveFontSize::Title => 32.0,
        };

        base_size * base_scale
    }

    /// Get responsive spacing
    pub fn spacing(&self, spacing_type: ResponsiveSpacing) -> f32 {
        let base_scale = match (self.device_type, self.orientation) {
            (DeviceType::Mobile, Orientation::Landscape) => 0.6,
            (DeviceType::Mobile, Orientation::Portrait) => 0.7,
            (DeviceType::Tablet, _) => 0.85,
            (DeviceType::Desktop, _) => 1.0,
        };

        let base_spacing = match spacing_type {
            ResponsiveSpacing::XSmall => 4.0,
            ResponsiveSpacing::Small => 8.0,
            ResponsiveSpacing::Medium => 16.0,
            ResponsiveSpacing::Large => 24.0,
            ResponsiveSpacing::XLarge => 32.0,
        };

        base_spacing * base_scale
    }

    /// Get responsive border radius
    pub fn border_radius(&self, radius_type: ResponsiveBorderRadius) -> f32 {
        let base_scale = match (self.device_type, self.orientation) {
            (DeviceType::Mobile, Orientation::Landscape) => 0.8,
            (DeviceType::Mobile, Orientation::Portrait) => 0.9,
            (DeviceType::Tablet, _) => 0.95,
            (DeviceType::Desktop, _) => 1.0,
        };

        let base_radius = match radius_type {
            ResponsiveBorderRadius::None => 0.0,
            ResponsiveBorderRadius::Small => 4.0,
            ResponsiveBorderRadius::Medium => 8.0,
            ResponsiveBorderRadius::Large => 12.0,
            ResponsiveBorderRadius::XLarge => 16.0,
        };

        base_radius * base_scale
    }

    /// Get responsive margin as i8 (egui compatible)
    pub fn margin(&self, margin_type: ResponsiveMargin) -> i8 {
        let base_scale = match (self.device_type, self.orientation) {
            (DeviceType::Mobile, Orientation::Landscape) => 0.5,
            (DeviceType::Mobile, Orientation::Portrait) => 0.6,
            (DeviceType::Tablet, _) => 0.8,
            (DeviceType::Desktop, _) => 1.0,
        };

        let base_margin = match margin_type {
            ResponsiveMargin::None => 0.0,
            ResponsiveMargin::Small => 8.0,
            ResponsiveMargin::Medium => 16.0,
            ResponsiveMargin::Large => 24.0,
            ResponsiveMargin::XLarge => 32.0,
        };

        (base_margin * base_scale) as i8
    }

    /// Get default container margin
    pub fn container_margin(&self) -> i8 {
        self.margin(ResponsiveMargin::Medium)
    }

    /// Get egui::Margin with equal margins on all sides
    pub fn margin_all(&self, margin_type: ResponsiveMargin) -> egui::Margin {
        let margin = self.margin(margin_type);
        egui::Margin::same(margin)
    }

    /// Get default container margin as egui::Margin
    pub fn container_margin_egui(&self) -> egui::Margin {
        self.margin_all(ResponsiveMargin::Medium)
    }

    /// Get symmetric egui::Margin (horizontal and vertical)
    pub fn margin_symmetric(
        &self,
        horizontal: ResponsiveMargin,
        vertical: ResponsiveMargin,
    ) -> egui::Margin {
        let h_margin = self.margin(horizontal);
        let v_margin = self.margin(vertical);
        egui::Margin::symmetric(h_margin, v_margin)
    }

    /// Get custom egui::Margin for each side
    pub fn margin_custom(
        &self,
        left: ResponsiveMargin,
        right: ResponsiveMargin,
        top: ResponsiveMargin,
        bottom: ResponsiveMargin,
    ) -> egui::Margin {
        egui::Margin {
            left: self.margin(left),
            right: self.margin(right),
            top: self.margin(top),
            bottom: self.margin(bottom),
        }
    }

    /// Check if device is mobile
    pub fn is_mobile(&self) -> bool {
        self.device_type == DeviceType::Mobile
    }

    /// Check if device is tablet
    pub fn is_tablet(&self) -> bool {
        self.device_type == DeviceType::Tablet
    }

    /// Check if device is desktop
    pub fn is_desktop(&self) -> bool {
        self.device_type == DeviceType::Desktop
    }

    /// Check if orientation is portrait
    pub fn is_portrait(&self) -> bool {
        self.orientation == Orientation::Portrait
    }
}

/// System to update responsive info when window size changes
pub fn update_responsive_info(
    mut responsive_info: ResMut<ResponsiveInfo>,
    windows: Query<&Window>,
) {
    if let Ok(window) = windows.single() {
        let window_size = Vec2::new(window.width(), window.height());
        responsive_info.update(window_size, window.scale_factor());
    }
}

/// Plugin for responsive UI system
pub struct ResponsivePlugin;

impl Plugin for ResponsivePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ResponsiveInfo>()
            .add_systems(PreUpdate, update_responsive_info);
    }
}
