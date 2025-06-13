use bevy::prelude::*;

// Base colors - with better contrast for dark backgrounds
/// #F5F5F5 - Base 100 (light background)
pub const BASE_100: Color = Color::srgb(0.961, 0.961, 0.961);
/// Slightly darker base
pub const BASE_200: Color = Color::srgb(0.933, 0.933, 0.933);
/// Even darker base
pub const BASE_300: Color = Color::srgb(0.906, 0.906, 0.906);
/// Base content color - brighter for better contrast on dark backgrounds
pub const BASE_CONTENT: Color = Color::srgb(0.95, 0.95, 0.95);

// Primary colors - Brighter Orange for better visibility
/// Primary color - Brighter orange
pub const PRIMARY: Color = Color::srgb(1.0, 0.6, 0.1);
/// Slightly lighter primary for hover states
pub const PRIMARY_LIGHT: Color = Color::srgb(1.0, 0.7, 0.3);
/// Darker primary for pressed states
pub const PRIMARY_DARK: Color = Color::srgb(0.9, 0.5, 0.0);
/// Content color on primary background
pub const PRIMARY_CONTENT: Color = Color::WHITE;

// Secondary colors - Brighter Purple
/// Secondary color - Brighter purple
pub const SECONDARY: Color = Color::srgb(0.651, 0.329, 0.996);
/// Lighter secondary for hover states
pub const SECONDARY_LIGHT: Color = Color::srgb(0.776, 0.502, 1.0);
/// Darker secondary for pressed states
pub const SECONDARY_DARK: Color = Color::srgb(0.553, 0.235, 0.882);
/// Content color on secondary background
pub const SECONDARY_CONTENT: Color = Color::WHITE;

// Accent/Tertiary - Lighter Gray for better visibility
/// Accent color - Lighter gray
pub const ACCENT: Color = Color::srgb(0.702, 0.702, 0.702);
/// Content color on accent background
pub const ACCENT_CONTENT: Color = Color::BLACK;

// Text colors with high contrast for dark backgrounds
/// Header text - Bright and visible against dark backgrounds
pub const HEADER_TEXT: Color = Color::srgb(1.0, 0.95, 0.8); // Warm white
/// Label text - Bright for visibility
pub const LABEL_TEXT: Color = Color::srgb(0.9, 0.9, 0.9); // Light gray
/// Button text - Using white for maximum contrast
pub const BUTTON_TEXT: Color = Color::WHITE;

// Info/Status colors - Brightened for better visibility
/// Info color - Brighter blue
pub const INFO: Color = Color::srgb(0.2, 0.7, 0.9);
/// Success color - Brighter green
pub const SUCCESS: Color = Color::srgb(0.2, 0.8, 0.4);
/// Warning color - Brighter yellow
pub const WARNING: Color = Color::srgb(1.0, 0.85, 0.2);
/// Error color - Brighter red
pub const ERROR: Color = Color::srgb(1.0, 0.3, 0.3);
