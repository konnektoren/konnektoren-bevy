use bevy::prelude::*;

/// Event sent when a setting value changes
#[derive(Message, Debug, Clone)]
pub struct SettingChangedEvent {
    pub setting_id: String,
    pub old_value: SettingValue,
    pub new_value: SettingValue,
}

/// Component that defines a setting
#[derive(Component, Debug, Clone)]
pub struct Setting {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub value: SettingValue,
    pub setting_type: SettingType,
    pub tab_index: Option<usize>,
    pub category: Option<String>,
    pub enabled: bool,
}

impl Setting {
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        value: SettingValue,
        setting_type: SettingType,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            value,
            setting_type,
            tab_index: None,
            category: None,
            enabled: true,
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_tab_index(mut self, index: usize) -> Self {
        self.tab_index = Some(index);
        self
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Different types of setting values
#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Selection(usize), // Index of selected option
}

impl SettingValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SettingValue::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            SettingValue::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            SettingValue::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            SettingValue::String(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_selection(&self) -> Option<usize> {
        match self {
            SettingValue::Selection(v) => Some(*v),
            _ => None,
        }
    }
}

/// Different types of settings and their constraints
#[derive(Debug, Clone)]
pub enum SettingType {
    /// Boolean toggle
    Toggle,
    /// Integer with min/max bounds
    IntRange { min: i32, max: i32, step: i32 },
    /// Float with min/max bounds
    FloatRange { min: f32, max: f32, step: f32 },
    /// Text input
    Text { max_length: Option<usize> },
    /// Selection from a list of options
    Selection { options: Vec<String> },
    /// Custom setting type with validation function
    Custom {
        validator: fn(&SettingValue) -> bool,
        display_fn: fn(&SettingValue) -> String,
    },
}

/// Component that marks a setting as changed
#[derive(Component)]
pub struct SettingChanged {
    pub old_value: SettingValue,
}
