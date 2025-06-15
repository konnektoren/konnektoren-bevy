use crate::{
    theme::KonnektorenTheme,
    ui::{
        responsive::{ResponsiveFontSize, ResponsiveInfo, ResponsiveSpacing},
        widgets::{ResponsiveText, ThemedButton},
    },
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Widget},
    EguiContexts,
};

/// Plugin for reusable settings screen functionality
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SettingsEvent>()
            .add_systems(Update, (check_settings_config, handle_settings_events))
            .add_systems(bevy_egui::EguiContextPass, render_settings_ui);
    }
}

/// Configuration for the settings screen
#[derive(Component, Clone)]
pub struct SettingsConfig {
    /// Title of the settings screen
    pub title: String,
    /// Settings sections to display
    pub sections: Vec<SettingsSection>,
    /// Allow dismissal (back button/escape)
    pub allow_dismissal: bool,
    /// Back button text
    pub back_button_text: String,
    /// Custom navigation indices for keyboard/gamepad navigation
    pub navigation_enabled: bool,
    /// Custom styling options
    pub mobile_layout: bool,
}

impl Default for SettingsConfig {
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

impl SettingsConfig {
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
    pub settings: Vec<SettingsItem>,
}

impl SettingsSection {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            settings: vec![],
        }
    }

    pub fn with_settings(mut self, settings: Vec<SettingsItem>) -> Self {
        self.settings = settings;
        self
    }

    pub fn add_setting(mut self, setting: SettingsItem) -> Self {
        self.settings.push(setting);
        self
    }
}

/// Individual setting item
#[derive(Clone)]
pub struct SettingsItem {
    pub id: String,
    pub label: String,
    pub setting_type: SettingType,
    pub navigation_index: Option<usize>,
}

impl SettingsItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>, setting_type: SettingType) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            setting_type,
            navigation_index: None,
        }
    }

    pub fn with_navigation_index(mut self, index: usize) -> Self {
        self.navigation_index = Some(index);
        self
    }
}

/// Types of settings
#[derive(Clone)]
pub enum SettingType {
    /// Toggle setting (boolean)
    Toggle { current_value: bool },
    /// Slider setting (float)
    Slider {
        current_value: f32,
        min_value: f32,
        max_value: f32,
        step: f32,
        format: String, // e.g., "{:.0}%" for percentage
    },
    /// Button group (multiple choice)
    ButtonGroup {
        options: Vec<String>,
        current_index: usize,
    },
    /// Custom setting with renderer function
    Custom {
        renderer: fn(&mut egui::Ui, &str, &KonnektorenTheme, &ResponsiveInfo) -> bool,
    },
}

/// Component marking an active settings screen
#[derive(Component)]
pub struct ActiveSettings {
    config: SettingsConfig,
    navigation_state: SettingsNavigationState,
}

/// Navigation state for settings
#[derive(Clone)]
pub struct SettingsNavigationState {
    pub current_index: usize,
    pub max_index: usize,
    pub enabled: bool,
}

impl Default for SettingsNavigationState {
    fn default() -> Self {
        Self {
            current_index: 0,
            max_index: 0,
            enabled: true,
        }
    }
}

/// Events for settings interactions
#[derive(Event)]
pub enum SettingsEvent {
    /// A setting value changed
    ValueChanged {
        entity: Entity,
        setting_id: String,
        value: SettingValue,
    },
    /// Settings screen dismissed
    Dismissed { entity: Entity },
    /// Navigation event
    Navigate { direction: NavigationDirection },
}

/// Setting value types
#[derive(Debug, Clone)]
pub enum SettingValue {
    Bool(bool),
    Float(f32),
    Int(i32),
    String(String),
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

/// System to check for new settings configurations
#[allow(clippy::type_complexity)]
fn check_settings_config(
    mut commands: Commands,
    query: Query<(Entity, &SettingsConfig), (Without<ActiveSettings>, Changed<SettingsConfig>)>,
    existing_settings: Query<Entity, With<ActiveSettings>>,
) {
    for (entity, config) in query.iter() {
        info!("Setting up settings screen for entity {:?}", entity);

        // Clean up any existing settings screens first
        for existing_entity in existing_settings.iter() {
            info!(
                "Cleaning up existing settings screen: {:?}",
                existing_entity
            );
            commands.entity(existing_entity).remove::<ActiveSettings>();
        }

        // Calculate max navigation index
        let mut max_index = 0;
        for section in &config.sections {
            for setting in &section.settings {
                if let Some(nav_index) = setting.navigation_index {
                    max_index = max_index.max(nav_index);
                }
            }
        }
        if config.allow_dismissal {
            max_index += 1; // For back button
        }

        let nav_state = SettingsNavigationState {
            max_index,
            ..Default::default()
        };

        commands.entity(entity).insert(ActiveSettings {
            config: config.clone(),
            navigation_state: nav_state,
        });
    }
}

/// System to render settings UI
fn render_settings_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    mut query: Query<(Entity, &mut ActiveSettings)>,
    mut settings_events: EventWriter<SettingsEvent>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }

    let ctx = contexts.ctx_mut();

    // Only render the first (most recent) settings screen
    if let Some((entity, mut settings)) = query.iter_mut().next() {
        // Check for escape key dismissal
        let should_dismiss = settings.config.allow_dismissal && input.just_pressed(KeyCode::Escape);
        if should_dismiss {
            settings_events.write(SettingsEvent::Dismissed { entity });
            return;
        }

        // Destructure to get separate borrows
        let ActiveSettings {
            config,
            navigation_state,
        } = &mut *settings;

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.base_100))
            .show(ctx, |ui| {
                render_settings_content(
                    ui,
                    config,
                    navigation_state,
                    &theme,
                    &responsive,
                    entity,
                    &mut settings_events,
                );
            });
    }
}

/// Render main settings content
fn render_settings_content(
    ui: &mut egui::Ui,
    config: &SettingsConfig,
    nav_state: &mut SettingsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut EventWriter<SettingsEvent>,
) {
    ui.vertical_centered(|ui| {
        let max_width = if responsive.is_mobile() {
            ui.available_width() * 0.95
        } else {
            800.0_f32.min(ui.available_width() * 0.9)
        };

        ui.set_max_width(max_width);

        // Header
        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
        ResponsiveText::new(&config.title, ResponsiveFontSize::Header, theme.primary)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));

        // Main content scrollable area
        let scroll_height = ui.available_height() - 80.0;
        egui::ScrollArea::vertical()
            .max_height(scroll_height)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if config.mobile_layout || responsive.is_mobile() {
                    render_mobile_settings_layout(
                        ui,
                        config,
                        nav_state,
                        theme,
                        responsive,
                        entity,
                        settings_events,
                    );
                } else {
                    render_desktop_settings_layout(
                        ui,
                        config,
                        nav_state,
                        theme,
                        responsive,
                        entity,
                        settings_events,
                    );
                }
            });

        // Back button
        if config.allow_dismissal {
            ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
            let back_button = ThemedButton::new(&config.back_button_text, theme)
                .responsive(responsive)
                .width(if responsive.is_mobile() { 200.0 } else { 150.0 });

            if ui.add(back_button).clicked() {
                settings_events.write(SettingsEvent::Dismissed { entity });
            }
        }

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
    });
}

/// Render mobile-style settings layout
#[allow(clippy::too_many_arguments)]
fn render_mobile_settings_layout(
    ui: &mut egui::Ui,
    config: &SettingsConfig,
    _nav_state: &SettingsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut EventWriter<SettingsEvent>,
) {
    let section_spacing = responsive.spacing(ResponsiveSpacing::Large);

    for section in &config.sections {
        // Section header
        ResponsiveText::new(&section.title, ResponsiveFontSize::Large, theme.secondary)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

        // Section settings
        for setting in &section.settings {
            render_mobile_setting_item(ui, setting, theme, responsive, entity, settings_events);
            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
        }

        ui.add_space(section_spacing);
        ui.separator();
        ui.add_space(section_spacing);
    }
}

/// Render desktop-style settings layout
#[allow(clippy::too_many_arguments)]
fn render_desktop_settings_layout(
    ui: &mut egui::Ui,
    config: &SettingsConfig,
    _nav_state: &SettingsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut EventWriter<SettingsEvent>,
) {
    for section in &config.sections {
        // Section header
        ResponsiveText::new(&section.title, ResponsiveFontSize::Large, theme.secondary)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

        // Grid layout for desktop
        egui::Grid::new(format!("settings_grid_{}", section.title))
            .num_columns(2)
            .spacing([30.0, 15.0])
            .show(ui, |ui| {
                for setting in &section.settings {
                    // Label column
                    ResponsiveText::new(
                        &setting.label,
                        ResponsiveFontSize::Medium,
                        theme.base_content,
                    )
                    .responsive(responsive)
                    .ui(ui);

                    // Control column
                    render_desktop_setting_control(
                        ui,
                        setting,
                        theme,
                        responsive,
                        entity,
                        settings_events,
                    );
                    ui.end_row();
                }
            });

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
    }
}

/// Render mobile setting item (vertical layout)
fn render_mobile_setting_item(
    ui: &mut egui::Ui,
    setting: &SettingsItem,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut EventWriter<SettingsEvent>,
) {
    ui.vertical_centered(|ui| {
        ResponsiveText::new(
            &setting.label,
            ResponsiveFontSize::Medium,
            theme.base_content,
        )
        .responsive(responsive)
        .strong()
        .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

        render_setting_control(ui, setting, theme, responsive, entity, settings_events);
    });
}

/// Render desktop setting control (horizontal layout)
fn render_desktop_setting_control(
    ui: &mut egui::Ui,
    setting: &SettingsItem,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut EventWriter<SettingsEvent>,
) {
    render_setting_control(ui, setting, theme, responsive, entity, settings_events);
}

/// Render individual setting control
fn render_setting_control(
    ui: &mut egui::Ui,
    setting: &SettingsItem,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut EventWriter<SettingsEvent>,
) {
    match &setting.setting_type {
        SettingType::Toggle { current_value } => {
            let button_text = if *current_value { "ON" } else { "OFF" };
            let button = ThemedButton::new(button_text, theme).responsive(responsive);

            if ui.add(button).clicked() {
                settings_events.write(SettingsEvent::ValueChanged {
                    entity,
                    setting_id: setting.id.clone(),
                    value: SettingValue::Bool(!current_value),
                });
            }
        }

        SettingType::Slider {
            current_value,
            min_value,
            max_value,
            step,
            format,
        } => {
            ui.horizontal(|ui| {
                let dec_button = ThemedButton::new("-", theme)
                    .responsive(responsive)
                    .width(30.0);

                if ui.add(dec_button).clicked() {
                    let new_value = (current_value - step).max(*min_value);
                    settings_events.write(SettingsEvent::ValueChanged {
                        entity,
                        setting_id: setting.id.clone(),
                        value: SettingValue::Float(new_value),
                    });
                }

                ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                // Fixed: Properly format the display text using the current value
                let display_text = if format.contains("{:.0}") {
                    format.replace("{:.0}", &format!("{:.0}", current_value))
                } else if format.contains("{:.1}") {
                    format.replace("{:.1}", &format!("{:.1}", current_value))
                } else if format.contains("{:.2}") {
                    format.replace("{:.2}", &format!("{:.2}", current_value))
                } else {
                    // Fallback: try to parse any format pattern
                    if let Some(start) = format.find('{') {
                        if let Some(end) = format.find('}') {
                            let format_spec = &format[start..=end];
                            if format_spec.contains(":.0") {
                                format.replace(format_spec, &format!("{:.0}", current_value))
                            } else if format_spec.contains(":.1") {
                                format.replace(format_spec, &format!("{:.1}", current_value))
                            } else if format_spec.contains(":.2") {
                                format.replace(format_spec, &format!("{:.2}", current_value))
                            } else {
                                format.replace(format_spec, &format!("{}", current_value))
                            }
                        } else {
                            current_value.to_string()
                        }
                    } else {
                        current_value.to_string()
                    }
                };

                ResponsiveText::new(
                    &display_text,
                    ResponsiveFontSize::Medium,
                    theme.base_content,
                )
                .responsive(responsive)
                .ui(ui);

                ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                let inc_button = ThemedButton::new("+", theme)
                    .responsive(responsive)
                    .width(30.0);

                if ui.add(inc_button).clicked() {
                    let new_value = (current_value + step).min(*max_value);
                    settings_events.write(SettingsEvent::ValueChanged {
                        entity,
                        setting_id: setting.id.clone(),
                        value: SettingValue::Float(new_value),
                    });
                }
            });
        }

        SettingType::ButtonGroup {
            options,
            current_index,
        } => {
            if responsive.is_mobile() {
                // Mobile: Show current option with left/right buttons
                ui.horizontal(|ui| {
                    let left_button = ThemedButton::new("◀", theme)
                        .responsive(responsive)
                        .width(40.0);

                    if ui.add(left_button).clicked() {
                        let new_index = if *current_index > 0 {
                            current_index - 1
                        } else {
                            options.len() - 1
                        };
                        settings_events.write(SettingsEvent::ValueChanged {
                            entity,
                            setting_id: setting.id.clone(),
                            value: SettingValue::Int(new_index as i32),
                        });
                    }

                    ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                    // Fixed: Use get() with unwrap_or_else() or provide a default &String
                    let def = String::new();
                    let current_option = options.get(*current_index).unwrap_or(&def);
                    ResponsiveText::new(current_option, ResponsiveFontSize::Medium, theme.primary)
                        .responsive(responsive)
                        .ui(ui);

                    ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                    let right_button = ThemedButton::new("▶", theme)
                        .responsive(responsive)
                        .width(40.0);

                    if ui.add(right_button).clicked() {
                        let new_index = (*current_index + 1) % options.len();
                        settings_events.write(SettingsEvent::ValueChanged {
                            entity,
                            setting_id: setting.id.clone(),
                            value: SettingValue::Int(new_index as i32),
                        });
                    }
                });
            } else {
                // Desktop: Show all options as buttons
                ui.horizontal_wrapped(|ui| {
                    for (index, option) in options.iter().enumerate() {
                        let is_selected = index == *current_index;
                        let mut button = ThemedButton::new(option, theme).responsive(responsive);

                        if is_selected {
                            button = button.with_style(|btn| {
                                btn.fill(theme.primary)
                                    .stroke(egui::Stroke::new(2.0, theme.primary))
                            });
                        }

                        if ui.add(button).clicked() && !is_selected {
                            settings_events.write(SettingsEvent::ValueChanged {
                                entity,
                                setting_id: setting.id.clone(),
                                value: SettingValue::Int(index as i32),
                            });
                        }
                    }
                });
            }
        }

        SettingType::Custom { renderer } => {
            renderer(ui, &setting.id, theme, responsive);
        }
    }
}

/// System to handle settings events
fn handle_settings_events(mut commands: Commands, mut settings_events: EventReader<SettingsEvent>) {
    for event in settings_events.read() {
        match event {
            SettingsEvent::Dismissed { entity } => {
                info!("Dismissing settings screen for entity {:?}", entity);
                commands.entity(*entity).remove::<ActiveSettings>();
            }
            SettingsEvent::ValueChanged {
                entity: _,
                setting_id,
                value,
            } => {
                info!("Setting '{}' changed to {:?}", setting_id, value);
                // Applications can listen to these events to update their settings
            }
            SettingsEvent::Navigate { direction } => {
                info!("Navigation event: {:?}", direction);
                // Handle navigation if needed
            }
        }
    }
}

/// Helper trait for easy settings screen setup
pub trait SettingsScreenExt {
    /// Add a settings screen with the given configuration
    fn spawn_settings(&mut self, config: SettingsConfig) -> Entity;

    /// Add a simple settings screen with basic audio settings
    fn spawn_simple_settings(&mut self, title: impl Into<String>) -> Entity;
}

impl SettingsScreenExt for Commands<'_, '_> {
    fn spawn_settings(&mut self, config: SettingsConfig) -> Entity {
        self.spawn((Name::new("Settings Screen"), config)).id()
    }

    fn spawn_simple_settings(&mut self, title: impl Into<String>) -> Entity {
        let audio_section = SettingsSection::new("Audio")
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
                "sound_effects",
                "Sound Effects",
                SettingType::Toggle {
                    current_value: true,
                },
            ));

        let config = SettingsConfig::new(title).add_section(audio_section);

        self.spawn_settings(config)
    }
}
