#[cfg(feature = "settings")]
use crate::settings::{Setting, SettingChanged, SettingType, SettingValue};
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
use std::collections::HashMap;

/// Events for component-based settings
#[derive(Event)]
pub enum ComponentSettingsEvent {
    /// Settings screen dismissed
    Dismissed { entity: Entity },
}

/// Component that marks an active component-based settings screen
#[derive(Component)]
pub struct ActiveComponentSettings {
    pub title: String,
    pub allow_dismissal: bool,
    pub back_button_text: String,
    pub navigation_state: ComponentSettingsNavigationState,
}

/// Navigation state for component-based settings
#[derive(Clone)]
pub struct ComponentSettingsNavigationState {
    pub current_index: usize,
    pub max_index: usize,
    pub enabled: bool,
}

impl Default for ComponentSettingsNavigationState {
    fn default() -> Self {
        Self {
            current_index: 0,
            max_index: 0,
            enabled: true,
        }
    }
}

/// Component to mark a setting that needs to be updated
#[derive(Component)]
pub struct PendingSettingUpdate {
    new_value: SettingValue,
}

#[cfg(feature = "settings")]
pub fn process_pending_setting_updates(
    mut settings_query: Query<(Entity, &mut Setting, &PendingSettingUpdate)>,
    mut commands: Commands,
) {
    for (entity, mut setting, update) in settings_query.iter_mut() {
        let old_value = setting.value.clone();
        setting.value = update.new_value.clone();

        commands
            .entity(entity)
            .insert(SettingChanged { old_value })
            .remove::<PendingSettingUpdate>();
    }
}

#[cfg(feature = "settings")]
pub fn check_component_settings(
    mut commands: Commands,
    settings_query: Query<&Setting>,
    existing_config: Query<Entity, With<ActiveComponentSettings>>,
) {
    if settings_query.is_empty() || !existing_config.is_empty() {
        return;
    }

    let max_index = settings_query
        .iter()
        .filter_map(|setting| setting.tab_index)
        .max()
        .unwrap_or(0)
        + 1;

    commands.spawn((
        Name::new("Component-Based Settings Screen"),
        ActiveComponentSettings {
            title: "Settings".to_string(),
            allow_dismissal: true,
            back_button_text: "Back".to_string(),
            navigation_state: ComponentSettingsNavigationState {
                max_index,
                ..Default::default()
            },
        },
    ));
}

#[cfg(feature = "settings")]
#[allow(clippy::too_many_arguments)]
pub fn render_component_settings_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    mut config_query: Query<&mut ActiveComponentSettings>,
    settings_query: Query<(Entity, &Setting)>,
    mut settings_events: EventWriter<ComponentSettingsEvent>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if config_query.is_empty() {
        return;
    }

    let ctx = contexts.ctx_mut();

    if let Ok(mut config) = config_query.single_mut() {
        let should_dismiss = config.allow_dismissal && input.just_pressed(KeyCode::Escape);
        if should_dismiss {
            settings_events.write(ComponentSettingsEvent::Dismissed {
                entity: Entity::PLACEHOLDER,
            });
            return;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(theme.base_100))
            .show(ctx, |ui| {
                render_component_settings_content(
                    ui,
                    &mut config,
                    &theme,
                    &responsive,
                    &settings_query,
                    &mut settings_events,
                    &mut commands,
                );
            });
    }
}

#[cfg(feature = "settings")]
fn render_component_settings_content(
    ui: &mut egui::Ui,
    config: &mut ActiveComponentSettings,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    settings_query: &Query<(Entity, &Setting)>,
    settings_events: &mut EventWriter<ComponentSettingsEvent>,
    commands: &mut Commands,
) {
    ui.vertical_centered(|ui| {
        let max_width = if responsive.is_mobile() {
            ui.available_width() * 0.95
        } else {
            800.0_f32.min(ui.available_width() * 0.9)
        };

        ui.set_max_width(max_width);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
        ResponsiveText::new(&config.title, ResponsiveFontSize::Header, theme.primary)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));

        let mut categories: HashMap<String, Vec<(Entity, &Setting)>> = HashMap::new();

        for (entity, setting) in settings_query.iter() {
            let category = setting
                .category
                .clone()
                .unwrap_or_else(|| "General".to_string());
            categories
                .entry(category)
                .or_default()
                .push((entity, setting));
        }

        for settings_list in categories.values_mut() {
            settings_list.sort_by_key(|(_, setting)| setting.tab_index.unwrap_or(usize::MAX));
        }

        let scroll_height = ui.available_height() - 80.0;
        egui::ScrollArea::vertical()
            .max_height(scroll_height)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if responsive.is_mobile() {
                    render_mobile_component_layout(
                        ui,
                        theme,
                        responsive,
                        &categories,
                        &config.navigation_state,
                        commands,
                    );
                } else {
                    render_desktop_component_layout(
                        ui,
                        theme,
                        &categories,
                        &config.navigation_state,
                        commands,
                    );
                }
            });

        if config.allow_dismissal {
            ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
            let back_button = ThemedButton::new(&config.back_button_text, theme)
                .responsive(responsive)
                .width(if responsive.is_mobile() { 200.0 } else { 150.0 });

            if ui.add(back_button).clicked() {
                settings_events.write(ComponentSettingsEvent::Dismissed {
                    entity: Entity::PLACEHOLDER,
                });
            }
        }

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
    });
}

#[cfg(feature = "settings")]
fn render_mobile_component_layout(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    categories: &HashMap<String, Vec<(Entity, &Setting)>>,
    _nav_state: &ComponentSettingsNavigationState,
    commands: &mut Commands,
) {
    let section_spacing = responsive.spacing(ResponsiveSpacing::Large);

    for (category_name, settings) in categories.iter() {
        ResponsiveText::new(category_name, ResponsiveFontSize::Large, theme.secondary)
            .responsive(responsive)
            .strong()
            .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

        for (entity, setting) in settings.iter() {
            render_mobile_component_setting_item(ui, *entity, setting, theme, responsive, commands);
            ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
        }

        ui.add_space(section_spacing);
        ui.separator();
        ui.add_space(section_spacing);
    }
}

#[cfg(feature = "settings")]
fn render_desktop_component_layout(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    categories: &HashMap<String, Vec<(Entity, &Setting)>>,
    _nav_state: &ComponentSettingsNavigationState,
    commands: &mut Commands,
) {
    for (category_name, settings) in categories.iter() {
        ResponsiveText::new(category_name, ResponsiveFontSize::Large, theme.secondary)
            .strong()
            .ui(ui);

        ui.add_space(10.0);

        egui::Grid::new(format!("component_settings_grid_{}", category_name))
            .num_columns(2)
            .spacing([30.0, 15.0])
            .show(ui, |ui| {
                for (entity, setting) in settings.iter() {
                    ResponsiveText::new(
                        &setting.label,
                        ResponsiveFontSize::Medium,
                        theme.base_content,
                    )
                    .ui(ui);

                    render_desktop_component_setting_control(ui, *entity, setting, theme, commands);
                    ui.end_row();
                }
            });

        ui.add_space(20.0);
    }
}

#[cfg(feature = "settings")]
fn render_mobile_component_setting_item(
    ui: &mut egui::Ui,
    entity: Entity,
    setting: &Setting,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    commands: &mut Commands,
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

        if let Some(description) = &setting.description {
            ui.add_space(responsive.spacing(ResponsiveSpacing::XSmall));
            ResponsiveText::new(description, ResponsiveFontSize::Small, theme.accent)
                .responsive(responsive)
                .ui(ui);
        }

        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

        render_component_setting_control(ui, entity, setting, theme, responsive, commands);
    });
}

#[cfg(feature = "settings")]
fn render_desktop_component_setting_control(
    ui: &mut egui::Ui,
    entity: Entity,
    setting: &Setting,
    theme: &KonnektorenTheme,
    commands: &mut Commands,
) {
    render_component_setting_control(
        ui,
        entity,
        setting,
        theme,
        &ResponsiveInfo::default(),
        commands,
    );
}

#[cfg(feature = "settings")]
fn render_component_setting_control(
    ui: &mut egui::Ui,
    entity: Entity,
    setting: &Setting,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    commands: &mut Commands,
) {
    if !setting.enabled {
        ui.add_enabled(false, egui::Label::new("Disabled"));
        return;
    }

    match &setting.setting_type {
        SettingType::Toggle => {
            if let Some(value) = setting.value.as_bool() {
                let button_text = if value { "ON" } else { "OFF" };
                let button = ThemedButton::new(button_text, theme).responsive(responsive);

                if ui.add(button).clicked() {
                    update_component_setting_value(entity, SettingValue::Bool(!value), commands);
                }
            }
        }

        SettingType::FloatRange { min, max, step } => {
            if let Some(current_value) = setting.value.as_float() {
                ui.horizontal(|ui| {
                    let dec_button = ThemedButton::new("-", theme)
                        .responsive(responsive)
                        .width(30.0);

                    if ui.add(dec_button).clicked() {
                        let new_value = (current_value - step).max(*min);
                        update_component_setting_value(
                            entity,
                            SettingValue::Float(new_value),
                            commands,
                        );
                    }

                    ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                    ResponsiveText::new(
                        &format!("{:.1}", current_value),
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
                        update_component_setting_value(
                            entity,
                            SettingValue::Float(new_value),
                            commands,
                        );
                    }
                });
            }
        }

        SettingType::IntRange { min, max, step } => {
            if let Some(current_value) = setting.value.as_int() {
                ui.horizontal(|ui| {
                    let dec_button = ThemedButton::new("-", theme)
                        .responsive(responsive)
                        .width(30.0);

                    if ui.add(dec_button).clicked() {
                        let new_value = (current_value - step).max(*min);
                        update_component_setting_value(
                            entity,
                            SettingValue::Int(new_value),
                            commands,
                        );
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
                        update_component_setting_value(
                            entity,
                            SettingValue::Int(new_value),
                            commands,
                        );
                    }
                });
            }
        }

        SettingType::Selection { options } => {
            if let Some(current_index) = setting.value.as_selection() {
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
                            update_component_setting_value(
                                entity,
                                SettingValue::Selection(new_index),
                                commands,
                            );
                        }

                        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                        let binding = String::new();
                        let current_option = options.get(current_index).unwrap_or(&binding);
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
                            update_component_setting_value(
                                entity,
                                SettingValue::Selection(new_index),
                                commands,
                            );
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
                                update_component_setting_value(
                                    entity,
                                    SettingValue::Selection(index),
                                    commands,
                                );
                            }
                        }
                    });
                }
            }
        }

        SettingType::Text { max_length: _ } => {
            if let Some(current_text) = setting.value.as_string() {
                let mut text = current_text.to_string();
                if ui.text_edit_singleline(&mut text).changed() {
                    update_component_setting_value(entity, SettingValue::String(text), commands);
                }
            }
        }

        SettingType::Custom { display_fn, .. } => {
            let display_text = display_fn(&setting.value);
            ui.label(display_text);
        }
    }
}

#[cfg(feature = "settings")]
fn update_component_setting_value(
    entity: Entity,
    new_value: SettingValue,
    commands: &mut Commands,
) {
    commands
        .entity(entity)
        .insert(PendingSettingUpdate { new_value });
}

pub fn cleanup_component_settings(
    mut commands: Commands,
    mut settings_events: EventReader<ComponentSettingsEvent>,
    config_query: Query<Entity, With<ActiveComponentSettings>>,
) {
    for _event in settings_events.read() {
        for entity in config_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
