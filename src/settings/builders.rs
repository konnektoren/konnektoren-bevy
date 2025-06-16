use super::components::*;
use bevy::prelude::*;

/// Resource that provides default settings configurations
#[derive(Resource, Default)]
pub struct SettingsRegistry {
    pub categories: Vec<SettingsCategory>,
}

/// A category of settings
#[derive(Debug, Clone)]
pub struct SettingsCategory {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub settings: Vec<SettingDefinition>,
}

/// Definition for creating a setting
#[derive(Debug, Clone)]
pub struct SettingDefinition {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub default_value: SettingValue,
    pub setting_type: SettingType,
    pub tab_index: Option<usize>,
}

impl SettingsRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_category(mut self, category: SettingsCategory) -> Self {
        self.categories.push(category);
        self
    }

    /// Create default audio settings category
    pub fn audio_category() -> SettingsCategory {
        SettingsCategory {
            name: "audio".to_string(),
            display_name: "Audio".to_string(),
            description: Some("Audio and sound settings".to_string()),
            settings: vec![
                SettingDefinition {
                    id: "master_volume".to_string(),
                    label: "Master Volume".to_string(),
                    description: Some("Overall audio volume".to_string()),
                    default_value: SettingValue::Float(1.0),
                    setting_type: SettingType::FloatRange {
                        min: 0.0,
                        max: 1.0,
                        step: 0.1,
                    },
                    tab_index: Some(0),
                },
                SettingDefinition {
                    id: "music_volume".to_string(),
                    label: "Music Volume".to_string(),
                    description: Some("Background music volume".to_string()),
                    default_value: SettingValue::Float(0.8),
                    setting_type: SettingType::FloatRange {
                        min: 0.0,
                        max: 1.0,
                        step: 0.1,
                    },
                    tab_index: Some(1),
                },
                SettingDefinition {
                    id: "sfx_volume".to_string(),
                    label: "Sound Effects".to_string(),
                    description: Some("Sound effects volume".to_string()),
                    default_value: SettingValue::Float(1.0),
                    setting_type: SettingType::FloatRange {
                        min: 0.0,
                        max: 1.0,
                        step: 0.1,
                    },
                    tab_index: Some(2),
                },
                SettingDefinition {
                    id: "audio_enabled".to_string(),
                    label: "Enable Audio".to_string(),
                    description: Some("Enable or disable all audio".to_string()),
                    default_value: SettingValue::Bool(true),
                    setting_type: SettingType::Toggle,
                    tab_index: Some(3),
                },
            ],
        }
    }

    /// Create default graphics settings category
    pub fn graphics_category() -> SettingsCategory {
        SettingsCategory {
            name: "graphics".to_string(),
            display_name: "Graphics".to_string(),
            description: Some("Visual and display settings".to_string()),
            settings: vec![
                SettingDefinition {
                    id: "vsync".to_string(),
                    label: "V-Sync".to_string(),
                    description: Some("Vertical synchronization".to_string()),
                    default_value: SettingValue::Bool(true),
                    setting_type: SettingType::Toggle,
                    tab_index: Some(0),
                },
                SettingDefinition {
                    id: "resolution".to_string(),
                    label: "Resolution".to_string(),
                    description: Some("Screen resolution".to_string()),
                    default_value: SettingValue::Selection(0),
                    setting_type: SettingType::Selection {
                        options: vec![
                            "1920x1080".to_string(),
                            "1680x1050".to_string(),
                            "1440x900".to_string(),
                            "1280x720".to_string(),
                        ],
                    },
                    tab_index: Some(1),
                },
            ],
        }
    }

    /// Create a complete game settings registry
    pub fn game_settings() -> Self {
        Self::new()
            .add_category(Self::audio_category())
            .add_category(Self::graphics_category())
    }
}

/// Builder for creating settings entities
#[derive(Default)]
pub struct SettingsBuilder {
    pub categories: Vec<SettingsCategory>,
}

impl SettingsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_category(mut self, category: SettingsCategory) -> Self {
        self.categories.push(category);
        self
    }

    pub fn with_audio_settings(self) -> Self {
        self.add_category(SettingsRegistry::audio_category())
    }

    pub fn with_graphics_settings(self) -> Self {
        self.add_category(SettingsRegistry::graphics_category())
    }

    /// Spawn setting entities from this builder
    pub fn spawn_settings(self, commands: &mut Commands) -> Vec<Entity> {
        let mut entities = Vec::new();

        for category in &self.categories {
            for setting_def in &category.settings {
                let setting = Setting::new(
                    setting_def.id.clone(),
                    setting_def.label.clone(),
                    setting_def.default_value.clone(),
                    setting_def.setting_type.clone(),
                )
                .with_category(category.name.clone());

                let entity = if let Some(tab_index) = setting_def.tab_index {
                    commands
                        .spawn((
                            Name::new(format!("Setting: {}", setting_def.id)),
                            setting.with_tab_index(tab_index),
                        ))
                        .id()
                } else {
                    commands
                        .spawn((Name::new(format!("Setting: {}", setting_def.id)), setting))
                        .id()
                };

                entities.push(entity);
            }
        }

        entities
    }
}

/// Helper trait for easy settings creation
pub trait SettingsExt {
    /// Spawn basic audio settings
    fn spawn_audio_settings(&mut self) -> Vec<Entity>;

    /// Spawn basic graphics settings
    fn spawn_graphics_settings(&mut self) -> Vec<Entity>;

    /// Spawn complete game settings
    fn spawn_game_settings(&mut self) -> Vec<Entity>;
}

impl SettingsExt for Commands<'_, '_> {
    fn spawn_audio_settings(&mut self) -> Vec<Entity> {
        SettingsBuilder::new()
            .with_audio_settings()
            .spawn_settings(self)
    }

    fn spawn_graphics_settings(&mut self) -> Vec<Entity> {
        SettingsBuilder::new()
            .with_graphics_settings()
            .spawn_settings(self)
    }

    fn spawn_game_settings(&mut self) -> Vec<Entity> {
        SettingsBuilder::new()
            .with_audio_settings()
            .with_graphics_settings()
            .spawn_settings(self)
    }
}
