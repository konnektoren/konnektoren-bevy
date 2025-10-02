use super::{
    components::*,
    device::{AvailableInputDevices, InputDevice},
};
use bevy::prelude::*;

/// System to handle keyboard input
pub fn handle_keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<(&mut InputController, &PlayerInputMapping)>,
    settings: Res<InputSettings>,
    mut input_events: MessageWriter<InputEvent>,
) {
    for (mut controller, mapping) in &mut controller_query {
        if !controller.enabled || !mapping.enabled {
            continue;
        }

        // Check if this player has a keyboard device assigned
        let keyboard_scheme = match &mapping.primary_device {
            Some(InputDevice::Keyboard(scheme)) => Some(scheme.clone()),
            _ => match &mapping.secondary_device {
                Some(InputDevice::Keyboard(scheme)) => Some(scheme.clone()),
                _ => None,
            },
        };

        let Some(scheme) = keyboard_scheme else {
            continue;
        };

        let (up, down, left, right) = scheme.get_keys();

        // Handle continuous movement input
        let mut movement = Vec2::ZERO;
        if keyboard.pressed(up) {
            movement.y += 1.0;
        }
        if keyboard.pressed(down) {
            movement.y -= 1.0;
        }
        if keyboard.pressed(left) {
            movement.x -= 1.0;
        }
        if keyboard.pressed(right) {
            movement.x += 1.0;
        }

        // Normalize diagonal movement
        if movement != Vec2::ZERO {
            movement = movement.normalize();
        }

        // Apply movement threshold
        if movement.length() > settings.movement_threshold {
            controller.movement = movement;
            controller.input_source = InputSource::Keyboard(scheme.clone());

            input_events.write(InputEvent::Movement {
                player_id: controller.player_id,
                direction: movement,
                source: controller.input_source.clone(),
            });
        } else if matches!(controller.input_source, InputSource::Keyboard(_)) {
            controller.movement = Vec2::ZERO;
        }

        // Handle action input
        let primary_pressed =
            keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter);
        let secondary_pressed =
            keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Backspace);

        if primary_pressed {
            controller.primary_action = true;
            controller.input_source = InputSource::Keyboard(scheme.clone());

            input_events.write(InputEvent::PrimaryAction {
                player_id: controller.player_id,
                source: controller.input_source.clone(),
            });
        }

        if secondary_pressed {
            controller.secondary_action = true;
            controller.input_source = InputSource::Keyboard(scheme.clone());

            input_events.write(InputEvent::SecondaryAction {
                player_id: controller.player_id,
                source: controller.input_source.clone(),
            });
        }
    }
}

/// System to handle gamepad input
pub fn handle_gamepad_input(
    gamepads: Query<(Entity, &Gamepad)>,
    mut controller_query: Query<(&mut InputController, &PlayerInputMapping)>,
    settings: Res<InputSettings>,
    mut input_events: MessageWriter<InputEvent>,
) {
    for (mut controller, mapping) in &mut controller_query {
        if !controller.enabled || !mapping.enabled {
            continue;
        }

        // Check if this player has a gamepad device assigned
        let gamepad_id = match &mapping.primary_device {
            Some(InputDevice::Gamepad(id)) => Some(*id),
            _ => match &mapping.secondary_device {
                Some(InputDevice::Gamepad(id)) => Some(*id),
                _ => None,
            },
        };

        let Some(target_gamepad_id) = gamepad_id else {
            continue;
        };

        // Find the gamepad entity - Fixed the gamepad matching logic
        let gamepad_entities: Vec<(Entity, &Gamepad)> = gamepads.iter().collect();

        let Some((gamepad_entity, gamepad)) = gamepad_entities.get(target_gamepad_id as usize)
        else {
            continue;
        };

        let mut movement = Vec2::ZERO;

        // D-Pad input
        if gamepad.pressed(GamepadButton::DPadUp) {
            movement.y += 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadDown) {
            movement.y -= 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadLeft) {
            movement.x -= 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadRight) {
            movement.x += 1.0;
        }

        // Analog stick input (with deadzone)
        let left_stick = gamepad.left_stick();
        if left_stick.length() > settings.gamepad_deadzone {
            movement += left_stick;
        }

        // Normalize and clamp movement
        if movement.length() > 1.0 {
            movement = movement.normalize();
        }

        if movement.length() > settings.movement_threshold {
            controller.movement = movement;
            controller.input_source = InputSource::Gamepad(*gamepad_entity);

            input_events.write(InputEvent::Movement {
                player_id: controller.player_id,
                direction: movement,
                source: controller.input_source.clone(),
            });
        } else if matches!(controller.input_source, InputSource::Gamepad(_)) {
            controller.movement = Vec2::ZERO;
        }

        // Handle action input
        if gamepad.just_pressed(GamepadButton::South) || gamepad.just_pressed(GamepadButton::Start)
        {
            controller.primary_action = true;
            controller.input_source = InputSource::Gamepad(*gamepad_entity);

            input_events.write(InputEvent::PrimaryAction {
                player_id: controller.player_id,
                source: controller.input_source.clone(),
            });
        }

        if gamepad.just_pressed(GamepadButton::East) || gamepad.just_pressed(GamepadButton::Select)
        {
            controller.secondary_action = true;
            controller.input_source = InputSource::Gamepad(*gamepad_entity);

            input_events.write(InputEvent::SecondaryAction {
                player_id: controller.player_id,
                source: controller.input_source.clone(),
            });
        }
    }
}

/// System to detect and track connected gamepads
pub fn detect_gamepads(
    mut available_devices: ResMut<AvailableInputDevices>,
    gamepads: Query<Entity, With<Gamepad>>,
) {
    let old_count = available_devices.gamepads.len();

    // Update connected gamepads list
    available_devices.gamepads.clear();
    for gamepad_entity in gamepads.iter() {
        available_devices.gamepads.push(gamepad_entity);
    }

    let new_count = available_devices.gamepads.len();

    if old_count != new_count {
        info!("Gamepad count changed: {} -> {}", old_count, new_count);
    }

    // Update general device availability
    available_devices.update_availability();
}

/// System to assign devices to players automatically if enabled
pub fn auto_assign_devices(
    mut assignment: ResMut<InputDeviceAssignment>,
    available_devices: Res<AvailableInputDevices>,
    settings: Res<InputSettings>,
    controllers: Query<&InputController>,
) {
    if !settings.auto_assign_devices || !assignment.assignments.is_empty() {
        return;
    }

    let player_count = controllers.iter().map(|c| c.player_id).max().unwrap_or(0) + 1;
    let available = available_devices.get_available_devices();

    for player_id in 0..player_count.min(assignment.max_players) {
        if assignment.get_device_for_player(player_id).is_some() {
            continue; // Already assigned
        }

        // Find an unassigned device
        if let Some(device) = available
            .iter()
            .find(|device| !assignment.is_device_assigned(device))
        {
            assignment.assign_device(player_id, device.clone());
        }
    }
}

/// System to update player input mappings from assignments
pub fn update_player_mappings(
    assignment: Res<InputDeviceAssignment>,
    mut mappings: Query<&mut PlayerInputMapping>,
) {
    if !assignment.is_changed() {
        return;
    }

    for mut mapping in mappings.iter_mut() {
        if let Some(device) = assignment.get_device_for_player(mapping.player_id) {
            mapping.primary_device = Some(device.clone());
        } else {
            mapping.primary_device = None;
        }
    }
}

/// System to clear input states at the end of each frame
pub fn clear_input_states(mut controllers: Query<&mut InputController>) {
    for mut controller in controllers.iter_mut() {
        controller.primary_action = false;
        controller.secondary_action = false;
    }
}
