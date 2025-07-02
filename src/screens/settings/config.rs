#[cfg(feature = "settings")]
use crate::settings::{Setting, SettingType, SettingValue};
use bevy::prelude::*;

/// Configuration for screen-based settings (not component-based)
#[derive(Component, Clone)]
pub struct SettingsScreenConfig {
    pub title: String,
    pub sections: Vec<SettingsSection>,
    pub allow_dismissal: bool,
    pub back_button_text: String,
    pub navigation_enabled: bool,
    pub mobile_layout: bool,
}

impl Default for SettingsScreenConfig {
    fn default() -> Self {
        Self {
            title: "Settings".to_string(),
            sections: vec![],
            allow_dismissal: true,
            back_button_text: "Back".to_string(),
            navigation_enabled: true,
            mobile_layout: false,
        }
    }
}

impl SettingsScreenConfig {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn with_sections(mut self, sections: Vec<SettingsSection>) -> Self {
        self.sections = sections;
        self
    }

    pub fn add_section(mut self, section: SettingsSection) -> Self {
        self.sections.push(section);
        self
    }

    pub fn with_back_button_text(mut self, text: impl Into<String>) -> Self {
        self.back_button_text = text.into();
        self
    }

    pub fn with_navigation(mut self, enabled: bool) -> Self {
        self.navigation_enabled = enabled;
        self
    }

    pub fn mobile_layout(mut self, mobile: bool) -> Self {
        self.mobile_layout = mobile;
        self
    }

    pub fn no_dismissal(mut self) -> Self {
        self.allow_dismissal = false;
        self
    }
}

/// A section in the settings screen
#[derive(Clone)]
pub struct SettingsSection {
    pub title: String,
    pub settings: Vec<ScreenSettingsItem>,
}

impl SettingsSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            settings: vec![],
        }
    }

    pub fn with_settings(mut self, settings: Vec<ScreenSettingsItem>) -> Self {
        self.settings = settings;
        self
    }

    pub fn add_setting(mut self, setting: ScreenSettingsItem) -> Self {
        self.settings.push(setting);
        self
    }
}

/// Individual setting item for screen-based settings
/// This wraps the core SettingType with screen-specific data and current values
#[derive(Clone)]
pub struct ScreenSettingsItem {
    pub id: String,
    pub label: String,
    #[cfg(feature = "settings")]
    pub setting_type: SettingType,
    #[cfg(not(feature = "settings"))]
    pub setting_type: ScreenOnlySettingType,
    #[cfg(feature = "settings")]
    pub current_value: SettingValue,
    #[cfg(not(feature = "settings"))]
    pub current_value: ScreenSettingValue,
    pub navigation_index: Option<usize>,
}

/// Fallback setting type when core settings feature is disabled
#[cfg(not(feature = "settings"))]
#[derive(Clone)]
pub enum ScreenOnlySettingType {
    Toggle,
    FloatRange {
        min: f32,
        max: f32,
        step: f32,
    },
    IntRange {
        min: i32,
        max: i32,
        step: i32,
    },
    Selection {
        options: Vec<String>,
    },
    Text {
        max_length: Option<usize>,
    },
    Custom {
        display_fn: fn(&ScreenSettingValue) -> String,
    },
}

/// Setting value types for screen-based settings (when core settings not available)
#[cfg(not(feature = "settings"))]
#[derive(Debug, Clone)]
pub enum ScreenSettingValue {
    Bool(bool),
    Float(f32),
    Int(i32),
    String(String),
    Selection(usize),
}

impl ScreenSettingsItem {
    #[cfg(feature = "settings")]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        setting_type: SettingType,
        current_value: SettingValue,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            setting_type,
            current_value,
            navigation_index: None,
        }
    }

    #[cfg(not(feature = "settings"))]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        setting_type: ScreenOnlySettingType,
        current_value: ScreenSettingValue,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            setting_type,
            current_value,
            navigation_index: None,
        }
    }

    pub fn with_navigation_index(mut self, index: usize) -> Self {
        self.navigation_index = Some(index);
        self
    }

    /// Update the current value
    #[cfg(feature = "settings")]
    pub fn with_value(mut self, value: SettingValue) -> Self {
        self.current_value = value;
        self
    }

    #[cfg(not(feature = "settings"))]
    pub fn with_value(mut self, value: ScreenSettingValue) -> Self {
        self.current_value = value;
        self
    }
}

/// Conversion from core Setting to ScreenSettingsItem
#[cfg(feature = "settings")]
impl From<&Setting> for ScreenSettingsItem {
    fn from(setting: &Setting) -> Self {
        Self {
            id: setting.id.clone(),
            label: setting.label.clone(),
            setting_type: setting.setting_type.clone(),
            current_value: setting.value.clone(),
            navigation_index: setting.tab_index,
        }
    }
}

#[cfg(feature = "settings")]
impl ScreenSettingsItem {
    /// Create from a core Setting component
    pub fn from_setting(setting: &Setting) -> Self {
        setting.into()
    }

    /// Helper functions to create common setting types
    pub fn toggle(id: impl Into<String>, label: impl Into<String>, current_value: bool) -> Self {
        Self::new(
            id,
            label,
            SettingType::Toggle,
            SettingValue::Bool(current_value),
        )
    }

    pub fn slider(
        id: impl Into<String>,
        label: impl Into<String>,
        current_value: f32,
        min: f32,
        max: f32,
        step: f32,
    ) -> Self {
        Self::new(
            id,
            label,
            SettingType::FloatRange { min, max, step },
            SettingValue::Float(current_value),
        )
    }

    pub fn int_slider(
        id: impl Into<String>,
        label: impl Into<String>,
        current_value: i32,
        min: i32,
        max: i32,
        step: i32,
    ) -> Self {
        Self::new(
            id,
            label,
            SettingType::IntRange { min, max, step },
            SettingValue::Int(current_value),
        )
    }

    pub fn selection(
        id: impl Into<String>,
        label: impl Into<String>,
        options: Vec<String>,
        current_index: usize,
    ) -> Self {
        Self::new(
            id,
            label,
            SettingType::Selection { options },
            SettingValue::Selection(current_index),
        )
    }

    pub fn text(
        id: impl Into<String>,
        label: impl Into<String>,
        current_text: String,
        max_length: Option<usize>,
    ) -> Self {
        Self::new(
            id,
            label,
            SettingType::Text { max_length },
            SettingValue::String(current_text),
        )
    }

    /// Create a custom setting type - ADD THIS METHOD
    pub fn custom(
        id: impl Into<String>,
        label: impl Into<String>,
        current_value: SettingValue,
        setting_type: SettingType,
    ) -> Self {
        Self::new(id, label, setting_type, current_value)
    }
}

/// Events for settings screen interactions
#[derive(Event)]
pub enum SettingsScreenEvent {
    /// A setting value changed
    ValueChanged {
        entity: Entity,
        setting_id: String,
        #[cfg(feature = "settings")]
        value: SettingValue,
        #[cfg(not(feature = "settings"))]
        value: ScreenSettingValue,
    },
    /// Settings screen dismissed
    Dismissed { entity: Entity },
    /// Navigation event
    Navigate { direction: NavigationDirection },
}

/// Navigation directions
#[derive(Debug, Clone)]
pub enum NavigationDirection {
    Up,
    Down,
    Left,
    Right,
    Select,
}

// Pre-built sections using core types
impl SettingsSection {
    /// Create an audio settings section with common audio controls
    #[cfg(feature = "settings")]
    pub fn audio_section() -> Self {
        Self::new("Audio Settings")
            .add_setting(ScreenSettingsItem::slider(
                "master_volume",
                "Master Volume",
                1.0,
                0.0,
                1.0,
                0.1,
            ))
            .add_setting(ScreenSettingsItem::slider(
                "music_volume",
                "Music Volume",
                0.8,
                0.0,
                1.0,
                0.1,
            ))
            .add_setting(ScreenSettingsItem::slider(
                "sfx_volume",
                "Sound Effects",
                1.0,
                0.0,
                1.0,
                0.1,
            ))
            .add_setting(ScreenSettingsItem::toggle(
                "audio_enabled",
                "Enable Audio",
                true,
            ))
    }

    #[cfg(not(feature = "settings"))]
    pub fn audio_section() -> Self {
        Self::new("Audio Settings")
            .add_setting(ScreenSettingsItem::new(
                "master_volume",
                "Master Volume",
                ScreenOnlySettingType::FloatRange {
                    min: 0.0,
                    max: 1.0,
                    step: 0.1,
                },
                ScreenSettingValue::Float(1.0),
            ))
            .add_setting(ScreenSettingsItem::new(
                "audio_enabled",
                "Enable Audio",
                ScreenOnlySettingType::Toggle,
                ScreenSettingValue::Bool(true),
            ))
    }

    /// ADD INPUT SECTION HERE
    #[cfg(feature = "input")]
    pub fn input_section() -> Self {
        Self::new("Input Settings").add_setting(ScreenSettingsItem::custom(
            "configure_players",
            "Configure Players",
            #[cfg(feature = "settings")]
            SettingValue::String("Configure".to_string()),
            #[cfg(not(feature = "settings"))]
            ScreenSettingValue::String("Configure".to_string()),
            #[cfg(feature = "settings")]
            SettingType::Custom {
                validator: |_| true,
                display_fn: |_| "Configure Input Devices →".to_string(),
            },
            #[cfg(not(feature = "settings"))]
            ScreenOnlySettingType::Custom {
                display_fn: |_| "Configure Input Devices →".to_string(),
            },
        ))
    }

    #[cfg(feature = "settings")]
    pub fn graphics_section() -> Self {
        Self::new("Graphics Settings")
            .add_setting(ScreenSettingsItem::selection(
                "resolution",
                "Resolution",
                vec![
                    "1920x1080".to_string(),
                    "1680x1050".to_string(),
                    "1440x900".to_string(),
                    "1280x720".to_string(),
                ],
                0,
            ))
            .add_setting(ScreenSettingsItem::toggle(
                "fullscreen",
                "Fullscreen",
                false,
            ))
            .add_setting(ScreenSettingsItem::toggle("vsync", "V-Sync", true))
    }

    #[cfg(feature = "settings")]
    pub fn gameplay_section() -> Self {
        Self::new("Gameplay Settings")
            .add_setting(ScreenSettingsItem::selection(
                "difficulty",
                "Difficulty",
                vec![
                    "Easy".to_string(),
                    "Normal".to_string(),
                    "Hard".to_string(),
                    "Expert".to_string(),
                ],
                1,
            ))
            .add_setting(ScreenSettingsItem::toggle("auto_save", "Auto Save", true))
    }

    /// Create from component-based settings
    #[cfg(feature = "settings")]
    pub fn from_settings(title: impl Into<String>, settings: &[&Setting]) -> Self {
        let mut section = Self::new(title);
        for setting in settings {
            section = section.add_setting(ScreenSettingsItem::from_setting(setting));
        }
        section
    }
}

// Pre-built configurations
impl SettingsScreenConfig {
    /// Create a complete game settings configuration with common sections
    #[cfg(all(feature = "settings", feature = "input"))]
    pub fn game_settings_with_input(title: impl Into<String>) -> Self {
        Self::new(title)
            .add_section(SettingsSection::audio_section())
            .add_section(SettingsSection::graphics_section())
            .add_section(SettingsSection::input_section())
            .add_section(SettingsSection::gameplay_section())
    }

    /// Create a complete game settings configuration with common sections
    #[cfg(feature = "settings")]
    pub fn game_settings(title: impl Into<String>) -> Self {
        Self::new(title)
            .add_section(SettingsSection::audio_section())
            .add_section(SettingsSection::graphics_section())
            .add_section(SettingsSection::gameplay_section())
    }

    /// Create minimal audio-only settings
    pub fn audio_only(title: impl Into<String>) -> Self {
        Self::new(title).add_section(SettingsSection::audio_section())
    }

    /// Create settings focused on educational games
    #[cfg(feature = "settings")]
    pub fn educational_game(title: impl Into<String>) -> Self {
        let learning_section = SettingsSection::new("Learning Settings")
            .add_setting(ScreenSettingsItem::toggle(
                "hints_enabled",
                "Show Hints",
                true,
            ))
            .add_setting(ScreenSettingsItem::selection(
                "feedback_level",
                "Feedback Level",
                vec![
                    "Minimal".to_string(),
                    "Standard".to_string(),
                    "Detailed".to_string(),
                ],
                1,
            ))
            .add_setting(ScreenSettingsItem::toggle(
                "progress_tracking",
                "Track Progress",
                true,
            ));

        Self::new(title)
            .add_section(SettingsSection::audio_section())
            .add_section(learning_section)
            .add_section(SettingsSection::gameplay_section())
    }

    /// Create from component-based settings grouped by category
    #[cfg(feature = "settings")]
    pub fn from_component_settings(title: impl Into<String>, settings: &[&Setting]) -> Self {
        use std::collections::HashMap;

        let mut categories: HashMap<String, Vec<&Setting>> = HashMap::new();

        for setting in settings {
            let category = setting
                .category
                .clone()
                .unwrap_or_else(|| "General".to_string());
            categories.entry(category).or_default().push(setting);
        }

        let mut config = Self::new(title);
        for (category_name, category_settings) in categories {
            let section = SettingsSection::from_settings(category_name, &category_settings);
            config = config.add_section(section);
        }

        config
    }
}
