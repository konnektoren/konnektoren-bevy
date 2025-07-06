use crate::{
    input::{
        components::{InputController, InputDeviceAssignment, InputEvent},
        device::{AvailableInputDevices, InputDevice},
    },
    theme::KonnektorenTheme,
    ui::{
        responsive::{ResponsiveFontSize, ResponsiveInfo, ResponsiveSpacing},
        widgets::{ResponsiveText, ThemedButton},
    },
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Id, Widget},
    EguiContexts,
};

/// Plugin for input configuration within settings
pub struct InputConfigurationPlugin;

impl Plugin for InputConfigurationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InputConfigurationEvent>()
            .add_systems(
                Update,
                (
                    handle_input_configuration_events,
                    cleanup_input_configuration,
                ),
            )
            .add_systems(bevy_egui::EguiContextPass, render_input_configuration_ui);
    }
}

/// Events for input configuration
#[derive(Event)]
pub enum InputConfigurationEvent {
    /// Open input configuration
    Open,
    /// Close input configuration
    Close,
    /// Device button clicked
    DeviceAssigned { player_id: u32, device: InputDevice },
    /// Player device unassigned
    DeviceUnassigned { player_id: u32 },
}

/// Component marking an active input configuration screen
#[derive(Component)]
pub struct ActiveInputConfiguration {
    pub max_players: u32,
    pub current_players: u32,
}

/// System to handle input configuration events
pub fn handle_input_configuration_events(
    mut config_events: EventReader<InputConfigurationEvent>,
    assignment: Option<ResMut<InputDeviceAssignment>>,
    mut input_events: EventWriter<InputEvent>,
    available_devices: Option<Res<AvailableInputDevices>>,
) {
    // Early return if input resources aren't available
    let (mut assignment, available_devices) = match (assignment, available_devices) {
        (Some(assignment), Some(available_devices)) => (assignment, available_devices),
        _ => {
            // If input resources aren't available, just consume events and warn
            for event in config_events.read() {
                match event {
                    InputConfigurationEvent::Open => {
                        warn!("Input configuration opened but InputPlugin not loaded");
                    }
                    InputConfigurationEvent::Close => {
                        info!("Closing input configuration");
                    }
                    _ => {
                        warn!("Input configuration event received but InputPlugin not loaded");
                    }
                }
            }
            return;
        }
    };

    for event in config_events.read() {
        match event {
            InputConfigurationEvent::Open => {
                info!("Opening input configuration");
            }
            InputConfigurationEvent::Close => {
                info!("Closing input configuration");
            }
            InputConfigurationEvent::DeviceAssigned { player_id, device } => {
                if !device.is_available(&available_devices) {
                    warn!("Device {} is not available", device.name());
                    continue;
                }

                if assignment.is_device_assigned(device)
                    && assignment.get_player_for_device(device) != Some(*player_id)
                {
                    warn!(
                        "Device {} is already assigned to another player",
                        device.name()
                    );
                    continue;
                }

                assignment.assign_device(*player_id, device.clone());
                input_events.write(InputEvent::DeviceAssigned {
                    player_id: *player_id,
                    device: device.clone(),
                });

                info!("Assigned {} to player {}", device.name(), player_id + 1);
            }
            InputConfigurationEvent::DeviceUnassigned { player_id } => {
                assignment.unassign_player(*player_id);
                input_events.write(InputEvent::DeviceUnassigned {
                    player_id: *player_id,
                });

                info!("Unassigned device from player {}", player_id + 1);
            }
        }
    }
}

/// System to render input configuration UI
#[allow(clippy::too_many_arguments)]
pub fn render_input_configuration_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    query: Query<(Entity, &ActiveInputConfiguration)>,
    assignment: Option<Res<InputDeviceAssignment>>,
    available_devices: Option<Res<AvailableInputDevices>>,
    mut config_events: EventWriter<InputConfigurationEvent>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if query.is_empty() {
        return;
    }

    let Ok((_screen_entity, config)) = query.single() else {
        return;
    };

    // Check if input resources are available
    let (assignment, available_devices) = match (assignment, available_devices) {
        (Some(assignment), Some(available_devices)) => (assignment, available_devices),
        _ => {
            // If input resources aren't available, show error message
            render_input_unavailable_ui(
                &mut contexts,
                &theme,
                &responsive,
                &mut config_events,
                &input,
            );
            return;
        }
    };

    let ctx = contexts.ctx_mut();

    // Handle escape to close
    if input.just_pressed(KeyCode::Escape) {
        config_events.write(InputConfigurationEvent::Close);
        return;
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(theme.base_100))
        .show(ctx, |ui| {
            render_input_configuration_content(
                ui,
                config,
                &theme,
                &responsive,
                &assignment,
                &available_devices,
                &mut config_events,
            );
        });
}

/// Render UI when input resources are not available
fn render_input_unavailable_ui(
    contexts: &mut EguiContexts,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    config_events: &mut EventWriter<InputConfigurationEvent>,
    input: &Res<ButtonInput<KeyCode>>,
) {
    let ctx = contexts.ctx_mut();

    // Handle escape to close
    if input.just_pressed(KeyCode::Escape) {
        config_events.write(InputConfigurationEvent::Close);
        return;
    }

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE.fill(theme.base_100))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let max_width = if responsive.is_mobile() {
                    ui.available_width() * 0.95
                } else {
                    600.0_f32.min(ui.available_width() * 0.9)
                };

                ui.set_max_width(max_width);

                // Header
                ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
                ResponsiveText::new(
                    "Input Configuration Not Available",
                    ResponsiveFontSize::Header,
                    theme.error,
                )
                .responsive(responsive)
                .strong()
                .ui(ui);

                ui.add_space(responsive.spacing(ResponsiveSpacing::Large));

                // Error message
                ResponsiveText::new(
                    "Input configuration is not available because the InputPlugin is not loaded.",
                    ResponsiveFontSize::Medium,
                    theme.base_content,
                )
                .responsive(responsive)
                .ui(ui);

                ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

                ResponsiveText::new(
                    "To enable input configuration, add InputPlugin to your app.",
                    ResponsiveFontSize::Medium,
                    theme.accent,
                )
                .responsive(responsive)
                .ui(ui);

                ui.add_space(responsive.spacing(ResponsiveSpacing::XLarge));

                // Back button with unique ID using scope and push_id
                ui.scope(|ui| {
                    ui.push_id(Id::new("input_unavailable_back_button"), |ui| {
                        let back_button = ThemedButton::new("← Back to Settings", theme)
                            .responsive(responsive)
                            .width(if responsive.is_mobile() { 200.0 } else { 180.0 });

                        if ui.add(back_button).clicked() {
                            config_events.write(InputConfigurationEvent::Close);
                        }
                    });
                });

                ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
            });
        });
}

fn render_input_configuration_content(
    ui: &mut egui::Ui,
    config: &ActiveInputConfiguration,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    assignment: &InputDeviceAssignment,
    available_devices: &AvailableInputDevices,
    config_events: &mut EventWriter<InputConfigurationEvent>,
) {
    ui.vertical_centered(|ui| {
        let max_width = if responsive.is_mobile() {
            ui.available_width() * 0.95
        } else {
            900.0_f32.min(ui.available_width() * 0.9)
        };

        ui.set_max_width(max_width);

        // Header
        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));
        ResponsiveText::new(
            "Configure Input Devices",
            ResponsiveFontSize::Header,
            theme.primary,
        )
        .responsive(responsive)
        .strong()
        .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

        // Instructions
        ResponsiveText::new(
            "Assign input devices to players. Each device can only be used by one player.",
            ResponsiveFontSize::Medium,
            theme.base_content,
        )
        .responsive(responsive)
        .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));

        // Device status section with unique ID
        ui.scope(|ui| {
            ui.push_id("device_status_section", |ui| {
                render_device_status_section(ui, theme, responsive, available_devices);
            });
        });

        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));

        // Player configuration grid with unique scroll area ID
        let scroll_height = ui.available_height() - 120.0;
        egui::ScrollArea::vertical()
            .id_salt("input_config_scroll")
            .max_height(scroll_height)
            .show(ui, |ui| {
                ui.scope(|ui| {
                    ui.push_id("player_config_grid", |ui| {
                        render_player_configuration_grid(
                            ui,
                            config,
                            theme,
                            responsive,
                            assignment,
                            available_devices,
                            config_events,
                        );
                    });
                });
            });

        // Footer with back button
        ui.add_space(responsive.spacing(ResponsiveSpacing::Large));

        ui.scope(|ui| {
            ui.push_id("input_config_back_button", |ui| {
                let back_button = ThemedButton::new("← Back to Settings", theme)
                    .responsive(responsive)
                    .width(if responsive.is_mobile() { 200.0 } else { 180.0 });

                if ui.add(back_button).clicked() {
                    config_events.write(InputConfigurationEvent::Close);
                }
            });
        });

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
    });
}

fn render_device_status_section(
    ui: &mut egui::Ui,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    available_devices: &AvailableInputDevices,
) {
    let frame = egui::Frame {
        inner_margin: responsive.margin_all(crate::ui::responsive::ResponsiveMargin::Medium),
        corner_radius: egui::CornerRadius::same(8),
        fill: theme.base_200,
        stroke: egui::Stroke::new(1.0, theme.accent.linear_multiply(0.3)),
        ..Default::default()
    };

    frame.show(ui, |ui| {
        ResponsiveText::new(
            "Available Devices",
            ResponsiveFontSize::Large,
            theme.secondary,
        )
        .responsive(responsive)
        .strong()
        .ui(ui);

        ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

        // Group devices by category
        let devices = available_devices.get_available_devices();
        let mut categories = std::collections::HashMap::new();

        for device in devices {
            let category = device.category();
            categories
                .entry(category)
                .or_insert_with(Vec::new)
                .push(device);
        }

        // Sort categories by their display order
        let mut sorted_categories: Vec<_> = categories.into_iter().collect();
        sorted_categories.sort_by_key(|(category, _)| category.order());

        // Display devices by category
        for (category, devices_in_category) in sorted_categories {
            ui.horizontal(|ui| {
                // Category icon and name
                ResponsiveText::new(
                    &format!("{} {}", category.icon(), category.name()),
                    ResponsiveFontSize::Medium,
                    theme.primary,
                )
                .responsive(responsive)
                .strong()
                .ui(ui);

                ResponsiveText::new(
                    &format!("({})", devices_in_category.len()),
                    ResponsiveFontSize::Small,
                    theme.base_content,
                )
                .responsive(responsive)
                .ui(ui);
            });

            ui.add_space(responsive.spacing(ResponsiveSpacing::XSmall));

            // Show individual devices in this category
            ui.horizontal_wrapped(|ui| {
                for (device_index, device) in devices_in_category.iter().enumerate() {
                    let is_available = device.is_available(available_devices);
                    let (bg_color, text_color) = if is_available {
                        (theme.success.linear_multiply(0.2), theme.success)
                    } else {
                        (theme.error.linear_multiply(0.2), theme.error)
                    };

                    let device_frame = egui::Frame {
                        inner_margin: egui::Margin::symmetric(6, 3),
                        corner_radius: egui::CornerRadius::same(4),
                        fill: bg_color,
                        ..Default::default()
                    };

                    ui.push_id(
                        format!("device_status_{}_{}", category.name(), device_index),
                        |ui| {
                            device_frame.show(ui, |ui| {
                                ResponsiveText::new(
                                    &device.name(),
                                    ResponsiveFontSize::Small,
                                    text_color,
                                )
                                .responsive(responsive)
                                .ui(ui);
                            });
                        },
                    );

                    ui.add_space(responsive.spacing(ResponsiveSpacing::XSmall));
                }
            });

            ui.add_space(responsive.spacing(ResponsiveSpacing::Small));
        }
    });
}

fn render_player_configuration_grid(
    ui: &mut egui::Ui,
    config: &ActiveInputConfiguration,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    assignment: &InputDeviceAssignment,
    available_devices: &AvailableInputDevices,
    config_events: &mut EventWriter<InputConfigurationEvent>,
) {
    let panel_width = if responsive.is_mobile() {
        ui.available_width() * 0.95
    } else {
        380.0
    };

    let mut current_player = 0;

    while current_player < config.current_players {
        // Add unique ID salt for each row of players
        ui.push_id(format!("player_row_{}", current_player / 2), |ui| {
            if responsive.is_mobile() {
                // Mobile: one column
                ui.push_id(format!("player_panel_mobile_{}", current_player), |ui| {
                    render_player_panel(
                        ui,
                        current_player,
                        panel_width,
                        theme,
                        responsive,
                        assignment,
                        available_devices,
                        config_events,
                    );
                });
                current_player += 1;
            } else {
                // Desktop: two columns
                ui.horizontal(|ui| {
                    ui.push_id(format!("player_panel_left_{}", current_player), |ui| {
                        render_player_panel(
                            ui,
                            current_player,
                            panel_width,
                            theme,
                            responsive,
                            assignment,
                            available_devices,
                            config_events,
                        );
                    });

                    if current_player + 1 < config.current_players {
                        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
                        ui.push_id(format!("player_panel_right_{}", current_player + 1), |ui| {
                            render_player_panel(
                                ui,
                                current_player + 1,
                                panel_width,
                                theme,
                                responsive,
                                assignment,
                                available_devices,
                                config_events,
                            );
                        });
                        current_player += 2;
                    } else {
                        current_player += 1;
                    }
                });
            }
        });

        ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));
    }
}

#[allow(clippy::too_many_arguments)]
fn render_player_panel(
    ui: &mut egui::Ui,
    player_id: u32,
    width: f32,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    assignment: &InputDeviceAssignment,
    available_devices: &AvailableInputDevices,
    config_events: &mut EventWriter<InputConfigurationEvent>,
) {
    let current_device = assignment.get_device_for_player(player_id);

    let frame = egui::Frame {
        inner_margin: responsive.margin_all(crate::ui::responsive::ResponsiveMargin::Medium),
        corner_radius: egui::CornerRadius::same(8),
        fill: theme.base_200,
        stroke: egui::Stroke::new(2.0, theme.primary.linear_multiply(0.5)),
        ..Default::default()
    };

    // Use player-specific ID for the entire panel
    ui.push_id(format!("player_panel_content_{}", player_id), |ui| {
        frame.show(ui, |ui| {
            ui.set_min_width(width);

            ui.vertical(|ui| {
                // Player header
                ResponsiveText::new(
                    &format!("Player {}", player_id + 1),
                    ResponsiveFontSize::Large,
                    theme.primary,
                )
                .responsive(responsive)
                .strong()
                .ui(ui);

                ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                // Current device display with unique ID
                ui.push_id(format!("current_device_display_{}", player_id), |ui| {
                    let (device_text, device_desc) = if let Some(device) = current_device {
                        (device.name(), device.description())
                    } else {
                        (
                            "No device assigned".to_string(),
                            "Select a device below".to_string(),
                        )
                    };

                    let device_color = if current_device.is_some() {
                        theme.success
                    } else {
                        theme.error
                    };

                    ResponsiveText::new(
                        &format!("Current: {}", device_text),
                        ResponsiveFontSize::Medium,
                        device_color,
                    )
                    .responsive(responsive)
                    .ui(ui);

                    if current_device.is_some() {
                        ResponsiveText::new(
                            &device_desc,
                            ResponsiveFontSize::Small,
                            theme.base_content,
                        )
                        .responsive(responsive)
                        .ui(ui);
                    }
                });

                ui.add_space(responsive.spacing(ResponsiveSpacing::Medium));

                // Device selection section with unique ID
                ui.push_id(format!("device_selection_{}", player_id), |ui| {
                    ResponsiveText::new(
                        "Available Devices:",
                        ResponsiveFontSize::Medium,
                        theme.base_content,
                    )
                    .responsive(responsive)
                    .ui(ui);

                    ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                    render_device_categories_for_player(
                        ui,
                        player_id,
                        width,
                        theme,
                        responsive,
                        assignment,
                        available_devices,
                        config_events,
                    );
                });

                // Unassign button with unique ID
                if current_device.is_some() {
                    ui.add_space(responsive.spacing(ResponsiveSpacing::Small));

                    ui.push_id(format!("unassign_section_{}", player_id), |ui| {
                        let unassign_button = ThemedButton::new("Unassign Device", theme)
                            .responsive(responsive)
                            .width(width - 40.0)
                            .with_style(|btn| btn.fill(theme.accent));

                        if ui.add(unassign_button).clicked() {
                            config_events
                                .write(InputConfigurationEvent::DeviceUnassigned { player_id });
                        }
                    });
                }
            });
        });
    });
}

fn render_device_categories_for_player(
    ui: &mut egui::Ui,
    player_id: u32,
    width: f32,
    theme: &KonnektorenTheme,
    responsive: &ResponsiveInfo,
    assignment: &InputDeviceAssignment,
    available_devices: &AvailableInputDevices,
    config_events: &mut EventWriter<InputConfigurationEvent>,
) {
    let devices = available_devices.get_available_devices();

    // Group devices by category for organized display
    let mut categories = std::collections::HashMap::new();
    for (index, device) in devices.iter().enumerate() {
        let category = device.category();
        categories
            .entry(category)
            .or_insert_with(Vec::new)
            .push((index, device));
    }

    // Sort categories by their display order
    let mut sorted_categories: Vec<_> = categories.into_iter().collect();
    sorted_categories.sort_by_key(|(category, _)| category.order());

    // Render each category with unique IDs
    for (category, devices_in_category) in sorted_categories {
        ui.push_id(
            format!("category_{}_{}", category.name(), player_id),
            |ui| {
                // Category header
                ResponsiveText::new(
                    &format!("{} {}", category.icon(), category.name()),
                    ResponsiveFontSize::Small,
                    theme.secondary,
                )
                .responsive(responsive)
                .ui(ui);

                ui.add_space(responsive.spacing(ResponsiveSpacing::XSmall));

                // Devices in this category with unique IDs
                ui.push_id(
                    format!("devices_in_category_{}_{}", category.name(), player_id),
                    |ui| {
                        for (device_index, device) in devices_in_category {
                            let is_selected =
                                assignment.get_device_for_player(player_id) == Some(device);
                            let is_available = device.is_available(available_devices);
                            let is_used_by_other = assignment.is_device_assigned(device)
                                && assignment.get_player_for_device(device) != Some(player_id);

                            let device_name = device.name();
                            let mut button = ThemedButton::new(&device_name, theme)
                                .responsive(responsive)
                                .width(width - 40.0);

                            // Style the button based on state
                            if is_selected {
                                button = button.with_style(|btn| btn.fill(theme.success));
                            } else if !is_available || is_used_by_other {
                                button = button.enabled(false).opacity(0.5);
                            }

                            // Use comprehensive unique ID for each device button
                            let button_id = format!(
                                "device_btn_p{}_cat{}_dev{}_idx{}",
                                player_id,
                                category.name(),
                                device_index,
                                device.name().replace(' ', "_")
                            );

                            ui.push_id(button_id, |ui| {
                                if ui.add(button).clicked() && is_available && !is_used_by_other {
                                    config_events.write(InputConfigurationEvent::DeviceAssigned {
                                        player_id,
                                        device: device.clone(),
                                    });
                                }
                            });

                            ui.add_space(responsive.spacing(ResponsiveSpacing::XSmall));
                        }
                    },
                );

                ui.add_space(responsive.spacing(ResponsiveSpacing::Small));
            },
        );
    }
}

/// System to cleanup input configuration
pub fn cleanup_input_configuration(
    mut commands: Commands,
    mut config_events: EventReader<InputConfigurationEvent>,
    config_query: Query<Entity, With<ActiveInputConfiguration>>,
) {
    for event in config_events.read() {
        if matches!(event, InputConfigurationEvent::Close) {
            for entity in config_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Helper function to spawn input configuration screen
pub fn spawn_input_configuration_screen(
    commands: &mut Commands,
    max_players: u32,
    controllers: &Query<&InputController>,
) -> Entity {
    let current_players = controllers.iter().map(|c| c.player_id).max().unwrap_or(0) + 1;

    commands
        .spawn((
            Name::new("Input Configuration Screen"),
            ActiveInputConfiguration {
                max_players,
                current_players,
            },
        ))
        .id()
}
