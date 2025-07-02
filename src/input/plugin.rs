use super::{components::*, device::AvailableInputDevices, systems::*};
use bevy::prelude::*;

/// Main input plugin that provides all input functionality
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize resources
            .init_resource::<AvailableInputDevices>()
            .init_resource::<InputDeviceAssignment>()
            .init_resource::<InputSettings>()
            // Register types for reflection
            .register_type::<InputController>()
            .register_type::<PlayerInputMapping>()
            .register_type::<InputDeviceAssignment>()
            .register_type::<InputSettings>()
            // Add events
            .add_event::<InputEvent>()
            // Add core input systems
            .add_systems(
                Update,
                (
                    detect_gamepads,
                    auto_assign_devices,
                    update_player_mappings,
                    handle_keyboard_input,
                    handle_gamepad_input,
                    clear_input_states,
                )
                    .chain(),
            );

        info!("InputPlugin loaded");
    }
}

/// Helper trait for easy input controller setup
pub trait InputControllerExt {
    /// Spawn an input controller for a player
    fn spawn_input_controller(&mut self, player_id: u32) -> Entity;

    /// Spawn multiple input controllers
    fn spawn_input_controllers(&mut self, player_count: u32) -> Vec<Entity>;
}

impl InputControllerExt for Commands<'_, '_> {
    fn spawn_input_controller(&mut self, player_id: u32) -> Entity {
        self.spawn((
            Name::new(format!("Input Controller P{}", player_id + 1)),
            InputController::new(player_id),
            PlayerInputMapping::new(player_id),
        ))
        .id()
    }

    fn spawn_input_controllers(&mut self, player_count: u32) -> Vec<Entity> {
        (0..player_count)
            .map(|player_id| self.spawn_input_controller(player_id))
            .collect()
    }
}
