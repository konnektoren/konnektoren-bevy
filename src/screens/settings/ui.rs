use super::config::*;
use super::input_configuration::{ActiveInputConfiguration, InputConfigurationEvent};
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

#[cfg(feature = "settings")]
use crate::settings::{SettingType, SettingValue};

/// Component marking an active settings screen
#[derive(Component)]
pub struct ActiveSettingsScreen {
    config: SettingsScreenConfig,
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

/// System to check for new settings configurations
#[allow(clippy::type_complexity)]
pub fn check_settings_screen_config(
    mut commands: Commands,
    query: Query<
        (Entity, &SettingsScreenConfig),
        (Without<ActiveSettingsScreen>, Changed<SettingsScreenConfig>),
    >,
    existing_settings: Query<Entity, With<ActiveSettingsScreen>>,
) {
    for (entity, config) in query.iter() {
        info!("Setting up settings screen for entity {:?}", entity);

        // Clean up any existing settings screens first
        for existing_entity in existing_settings.iter() {
            info!(
                "Cleaning up existing settings screen: {:?}",
                existing_entity
            );
            commands
                .entity(existing_entity)
                .remove::<ActiveSettingsScreen>();
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

        commands.entity(entity).insert(ActiveSettingsScreen {
            config: config.clone(),
            navigation_state: nav_state,
        });
    }
}

/// System to handle settings value changes and update the active screen
pub fn update_settings_screen_values(
    mut settings_events: MessageReader<SettingsScreenEvent>,
    mut active_settings_query: Query<&mut ActiveSettingsScreen>,
) {
    for event in settings_events.read() {
        if let SettingsScreenEvent::ValueChanged {
            setting_id,
            value,
            entity: _,
        } = event
        {
            // Update all active settings screens (there should only be one)
            for mut active_settings in active_settings_query.iter_mut() {
                let mut updated = false;

                for section in &mut active_settings.config.sections {
                    for setting in &mut section.settings {
                        if setting.id == *setting_id {
                            #[cfg(feature = "settings")]
                            {
                                setting.current_value = value.clone();
                                updated = true;
                                info!("Updated active setting '{}' to {:?}", setting_id, value);
                            }
                            #[cfg(not(feature = "settings"))]
                            {
                                setting.current_value = value.clone();
                                updated = true;
                                info!("Updated active setting '{}' to {:?}", setting_id, value);
                            }
                            break;
                        }
                    }
                    if updated {
                        break;
                    }
                }
            }
        }
    }
}

// At the top, change the egui context handling:
pub fn render_settings_screen_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    mut query: Query<(Entity, &mut ActiveSettingsScreen)>,
    mut settings_events: MessageWriter<SettingsScreenEvent>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }

    // Get the egui context directly
    if let Ok(ctx) = contexts.ctx_mut() {
        // Only render the first (most recent) settings screen
        if let Some((entity, mut settings)) = query.iter_mut().next() {
            // Check for escape key dismissal
            let should_dismiss =
                settings.config.allow_dismissal && input.just_pressed(KeyCode::Escape);
            if should_dismiss {
                settings_events.write(SettingsScreenEvent::Dismissed { entity });
                return;
            }

            // Destructure to get separate borrows
            let ActiveSettingsScreen {
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
}

/// Render main settings content
fn render_settings_content(
    ui: &mut egui::Ui,
    config: &SettingsScreenConfig,
    nav_state: &mut SettingsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut MessageWriter<SettingsScreenEvent>,
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
                settings_events.write(SettingsScreenEvent::Dismissed { entity });
            }
        }

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
    });
}

/// Render mobile-style settings layout
#[allow(clippy::too_many_arguments)]
fn render_mobile_settings_layout(
    ui: &mut egui::Ui,
    config: &SettingsScreenConfig,
    _nav_state: &SettingsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut MessageWriter<SettingsScreenEvent>,
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
    config: &SettingsScreenConfig,
    _nav_state: &SettingsNavigationState,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut MessageWriter<SettingsScreenEvent>,
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
    setting: &ScreenSettingsItem,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut MessageWriter<SettingsScreenEvent>,
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
    setting: &ScreenSettingsItem,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut MessageWriter<SettingsScreenEvent>,
) {
    render_setting_control(ui, setting, theme, responsive, entity, settings_events);
}

/// Render individual setting control
fn render_setting_control(
    ui: &mut egui::Ui,
    setting: &ScreenSettingsItem,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    entity: Entity,
    settings_events: &mut MessageWriter<SettingsScreenEvent>,
) {
    #[cfg(feature = "settings")]
    {
        match &setting.setting_type {
            crate::settings::SettingType::Toggle => {
                if let Some(current_value) = setting.current_value.as_bool() {
                    let button_text = if current_value { "ON" } else { "OFF" };
                    let button = ThemedButton::new(button_text, theme).responsive(responsive);

                    if ui.add(button).clicked() {
                        settings_events.write(SettingsScreenEvent::ValueChanged {
                            entity,
                            setting_id: setting.id.clone(),
                            value: SettingValue::Bool(!current_value),
                        });
                    }
                }
            }

            SettingType::FloatRange { min, max, step } => {
                if let Some(current_value) = setting.current_value.as_float() {
                    ui.horizontal(|ui| {
                        let dec_button = ThemedButton::new("-", theme)
                            .responsive(responsive)
                            .width(30.0);

                        if ui.add(dec_button).clicked() {
                            let new_value = (current_value - step).max(*min);
                            settings_events.write(SettingsScreenEvent::ValueChanged {
                                entity,
                                setting_id: setting.id.clone(),
                                value: SettingValue::Float(new_value),
                            });
                        }

                        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                        // Display current value as percentage for volume controls
                        let display_text = if setting.id.contains("volume") {
                            format!("{:.0}%", current_value * 100.0)
                        } else {
                            format!("{:.1}", current_value)
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
                            let new_value = (current_value + step).min(*max);
                            settings_events.write(SettingsScreenEvent::ValueChanged {
                                entity,
                                setting_id: setting.id.clone(),
                                value: SettingValue::Float(new_value),
                            });
                        }
                    });
                }
            }

            SettingType::Selection { options } => {
                if let Some(current_index) = setting.current_value.as_selection() {
                    if responsive.is_mobile() {
                        ui.horizontal(|ui| {
                            let left_button = ThemedButton::new("◀", theme)
                                .responsive(responsive)
                                .width(40.0);

                            if ui.add(left_button).clicked() {
                                let new_index = if current_index > 0 {
                                    current_index - 1
                                } else {
                                    options.len() - 1
                                };
                                settings_events.write(SettingsScreenEvent::ValueChanged {
                                    entity,
                                    setting_id: setting.id.clone(),
                                    value: SettingValue::Selection(new_index),
                                });
                            }

                            ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                            let def = String::new();
                            let current_option = options.get(current_index).unwrap_or(&def);
                            ResponsiveText::new(
                                current_option,
                                ResponsiveFontSize::Medium,
                                theme.primary,
                            )
                            .responsive(responsive)
                            .ui(ui);

                            ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                            let right_button = ThemedButton::new("▶", theme)
                                .responsive(responsive)
                                .width(40.0);

                            if ui.add(right_button).clicked() {
                                let new_index = (current_index + 1) % options.len();
                                settings_events.write(SettingsScreenEvent::ValueChanged {
                                    entity,
                                    setting_id: setting.id.clone(),
                                    value: SettingValue::Selection(new_index),
                                });
                            }
                        });
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            for (index, option) in options.iter().enumerate() {
                                let is_selected = index == current_index;
                                let mut button =
                                    ThemedButton::new(option, theme).responsive(responsive);

                                if is_selected {
                                    button = button.with_style(|btn| {
                                        btn.fill(theme.primary)
                                            .stroke(egui::Stroke::new(2.0, theme.primary))
                                    });
                                }

                                if ui.add(button).clicked() && !is_selected {
                                    settings_events.write(SettingsScreenEvent::ValueChanged {
                                        entity,
                                        setting_id: setting.id.clone(),
                                        value: SettingValue::Selection(index),
                                    });
                                }
                            }
                        });
                    }
                }
            }

            SettingType::IntRange { min, max, step } => {
                if let Some(current_value) = setting.current_value.as_int() {
                    ui.horizontal(|ui| {
                        let dec_button = ThemedButton::new("-", theme)
                            .responsive(responsive)
                            .width(30.0);

                        if ui.add(dec_button).clicked() {
                            let new_value = (current_value - step).max(*min);
                            settings_events.write(SettingsScreenEvent::ValueChanged {
                                entity,
                                setting_id: setting.id.clone(),
                                value: SettingValue::Int(new_value),
                            });
                        }

                        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                        ResponsiveText::new(
                            &current_value.to_string(),
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
                            let new_value = (current_value + step).min(*max);
                            settings_events.write(SettingsScreenEvent::ValueChanged {
                                entity,
                                setting_id: setting.id.clone(),
                                value: SettingValue::Int(new_value),
                            });
                        }
                    });
                }
            }

            SettingType::Text { max_length } => {
                if let Some(current_text) = setting.current_value.as_string() {
                    let mut text = current_text.to_string();
                    let text_edit = egui::TextEdit::singleline(&mut text);

                    if ui.add(text_edit).changed() {
                        // Apply max_length if specified
                        if let Some(max_len) = max_length {
                            text.truncate(*max_len);
                        }

                        settings_events.write(SettingsScreenEvent::ValueChanged {
                            entity,
                            setting_id: setting.id.clone(),
                            value: SettingValue::String(text),
                        });
                    }
                }
            }

            SettingType::Custom { display_fn, .. } => {
                let display_text = display_fn(&setting.current_value);

                // Check if this is a button-like custom setting (like "Configure Players")
                if setting.id == "configure_players" {
                    let button = ThemedButton::new(&display_text, theme).responsive(responsive);

                    if ui.add(button).clicked() {
                        settings_events.write(SettingsScreenEvent::ValueChanged {
                            entity,
                            setting_id: setting.id.clone(),
                            value: setting.current_value.clone(),
                        });
                    }
                } else {
                    // For other custom types, just display the text
                    ResponsiveText::new(
                        &display_text,
                        ResponsiveFontSize::Medium,
                        theme.base_content,
                    )
                    .responsive(responsive)
                    .ui(ui);
                }
            }
        }
    }

    #[cfg(not(feature = "settings"))]
    {
        match &setting.setting_type {
            ScreenOnlySettingType::Toggle => {
                if let ScreenSettingValue::Bool(current_value) = &setting.current_value {
                    let button_text = if *current_value { "ON" } else { "OFF" };
                    let button = ThemedButton::new(button_text, theme).responsive(responsive);

                    if ui.add(button).clicked() {
                        settings_events.write(SettingsScreenEvent::ValueChanged {
                            entity,
                            setting_id: setting.id.clone(),
                            value: ScreenSettingValue::Bool(!current_value),
                        });
                    }
                }
            }
            ScreenOnlySettingType::Custom { display_fn } => {
                let display_text = display_fn(&setting.current_value);

                if setting.id == "configure_players" {
                    let button = ThemedButton::new(&display_text, theme).responsive(responsive);

                    if ui.add(button).clicked() {
                        settings_events.write(SettingsScreenEvent::ValueChanged {
                            entity,
                            setting_id: setting.id.clone(),
                            value: setting.current_value.clone(),
                        });
                    }
                } else {
                    ResponsiveText::new(
                        &display_text,
                        ResponsiveFontSize::Medium,
                        theme.base_content,
                    )
                    .responsive(responsive)
                    .ui(ui);
                }
            }
            _ => {
                ui.label("Setting type not implemented for screen-only mode");
            }
        }
    }
}

/// System to handle settings events
pub fn handle_settings_screen_events(
    mut commands: Commands,
    mut settings_events: MessageReader<SettingsScreenEvent>,
    mut input_config_events: MessageWriter<InputConfigurationEvent>,
) {
    for event in settings_events.read() {
        match event {
            SettingsScreenEvent::Dismissed { entity } => {
                info!("Dismissing settings screen for entity {:?}", entity);
                commands.entity(*entity).remove::<ActiveSettingsScreen>();
            }
            SettingsScreenEvent::ValueChanged {
                entity: _,
                setting_id,
                value,
            } => {
                info!("Setting '{}' changed to {:?}", setting_id, value);

                // Handle input configuration button
                if setting_id == "configure_players" {
                    input_config_events.write(InputConfigurationEvent::Open);
                    // Spawn the input configuration screen directly
                    commands.spawn((
                        Name::new("Input Configuration Screen"),
                        ActiveInputConfiguration {
                            max_players: 4,
                            current_players: 4,
                        },
                    ));
                }
            }
            SettingsScreenEvent::Navigate { direction } => {
                info!("Navigation event: {:?}", direction);
            }
        }
    }
}
