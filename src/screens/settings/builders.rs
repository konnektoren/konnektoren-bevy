use super::*;

impl SettingsSection {
    /// Create an audio settings section with common audio controls
    pub fn audio_section() -> Self {
        Self::new("Audio Settings")
            .add_setting(SettingsItem::new(
                "master_volume",
                "Master Volume",
                SettingType::Slider {
                    current_value: 1.0,
                    min_value: 0.0,
                    max_value: 1.0,
                    step: 0.1,
                    format: "{:.0}%".to_string(),
                },
            ))
            .add_setting(SettingsItem::new(
                "music_volume",
                "Music Volume",
                SettingType::Slider {
                    current_value: 0.8,
                    min_value: 0.0,
                    max_value: 1.0,
                    step: 0.1,
                    format: "{:.0}%".to_string(),
                },
            ))
            .add_setting(SettingsItem::new(
                "sfx_volume",
                "Sound Effects",
                SettingType::Slider {
                    current_value: 1.0,
                    min_value: 0.0,
                    max_value: 1.0,
                    step: 0.1,
                    format: "{:.0}%".to_string(),
                },
            ))
            .add_setting(SettingsItem::new(
                "audio_enabled",
                "Enable Audio",
                SettingType::Toggle {
                    current_value: true,
                },
            ))
    }

    /// Create a graphics settings section
    pub fn graphics_section() -> Self {
        Self::new("Graphics Settings")
            .add_setting(SettingsItem::new(
                "resolution",
                "Resolution",
                SettingType::ButtonGroup {
                    options: vec![
                        "1920x1080".to_string(),
                        "1680x1050".to_string(),
                        "1440x900".to_string(),
                        "1280x720".to_string(),
                    ],
                    current_index: 0,
                },
            ))
            .add_setting(SettingsItem::new(
                "fullscreen",
                "Fullscreen",
                SettingType::Toggle {
                    current_value: false,
                },
            ))
            .add_setting(SettingsItem::new(
                "vsync",
                "V-Sync",
                SettingType::Toggle {
                    current_value: true,
                },
            ))
    }

    /// Create a gameplay settings section
    pub fn gameplay_section() -> Self {
        Self::new("Gameplay Settings")
            .add_setting(SettingsItem::new(
                "difficulty",
                "Difficulty",
                SettingType::ButtonGroup {
                    options: vec![
                        "Easy".to_string(),
                        "Normal".to_string(),
                        "Hard".to_string(),
                        "Expert".to_string(),
                    ],
                    current_index: 1,
                },
            ))
            .add_setting(SettingsItem::new(
                "auto_save",
                "Auto Save",
                SettingType::Toggle {
                    current_value: true,
                },
            ))
    }

    /// Create an input settings section
    pub fn input_section() -> Self {
        Self::new("Input Settings")
            .add_setting(SettingsItem::new(
                "mouse_sensitivity",
                "Mouse Sensitivity",
                SettingType::Slider {
                    current_value: 1.0,
                    min_value: 0.1,
                    max_value: 3.0,
                    step: 0.1,
                    format: "{:.1}x".to_string(),
                },
            ))
            .add_setting(SettingsItem::new(
                "invert_mouse",
                "Invert Mouse",
                SettingType::Toggle {
                    current_value: false,
                },
            ))
            .add_setting(SettingsItem::new(
                "gamepad_enabled",
                "Gamepad Support",
                SettingType::Toggle {
                    current_value: true,
                },
            ))
    }
}

impl SettingsConfig {
    /// Create a complete game settings configuration with common sections
    pub fn game_settings(title: impl Into<String>) -> Self {
        Self::new(title)
            .add_section(SettingsSection::audio_section())
            .add_section(SettingsSection::graphics_section())
            .add_section(SettingsSection::gameplay_section())
            .add_section(SettingsSection::input_section())
    }

    /// Create minimal audio-only settings
    pub fn audio_only(title: impl Into<String>) -> Self {
        Self::new(title).add_section(SettingsSection::audio_section())
    }

    /// Create settings focused on educational games
    pub fn educational_game(title: impl Into<String>) -> Self {
        let learning_section = SettingsSection::new("Learning Settings")
            .add_setting(SettingsItem::new(
                "hints_enabled",
                "Show Hints",
                SettingType::Toggle {
                    current_value: true,
                },
            ))
            .add_setting(SettingsItem::new(
                "feedback_level",
                "Feedback Level",
                SettingType::ButtonGroup {
                    options: vec![
                        "Minimal".to_string(),
                        "Standard".to_string(),
                        "Detailed".to_string(),
                    ],
                    current_index: 1,
                },
            ))
            .add_setting(SettingsItem::new(
                "progress_tracking",
                "Track Progress",
                SettingType::Toggle {
                    current_value: true,
                },
            ));

        Self::new(title)
            .add_section(SettingsSection::audio_section())
            .add_section(learning_section)
            .add_section(SettingsSection::gameplay_section())
    }
}
