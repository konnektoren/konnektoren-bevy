use super::device::{InputDevice, KeyboardScheme};
use bevy::prelude::*;
use std::collections::HashMap;

/// Main input controller component for entities that need input
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct InputController {
    pub player_id: u32,
    pub movement: Vec2,
    pub primary_action: bool,   // Confirm/Select/Activate
    pub secondary_action: bool, // Back/Cancel/Abort
    pub input_source: InputSource,
    pub enabled: bool,
}

impl Default for InputController {
    fn default() -> Self {
        Self {
            player_id: 0,
            movement: Vec2::ZERO,
            primary_action: false,
            secondary_action: false,
            input_source: InputSource::Keyboard(KeyboardScheme::WASD),
            enabled: true,
        }
    }
}

impl InputController {
    pub fn new(player_id: u32) -> Self {
        Self {
            player_id,
            ..Default::default()
        }
    }

    pub fn with_input_source(mut self, source: InputSource) -> Self {
        self.input_source = source;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Clear all input states
    pub fn clear(&mut self) {
        self.movement = Vec2::ZERO;
        self.primary_action = false;
        self.secondary_action = false;
    }

    /// Check if any input is active
    pub fn has_input(&self) -> bool {
        self.movement != Vec2::ZERO || self.primary_action || self.secondary_action
    }
}

/// Input source tracking
#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum InputSource {
    Keyboard(KeyboardScheme),
    Gamepad(Entity),
    Mouse,
    Touch,
}

impl InputSource {
    pub fn name(&self) -> String {
        match self {
            InputSource::Keyboard(scheme) => format!("Keyboard ({})", scheme.name()),
            InputSource::Gamepad(_) => "Gamepad".to_string(),
            InputSource::Mouse => "Mouse".to_string(),
            InputSource::Touch => "Touch".to_string(),
        }
    }
}

/// Component to map specific inputs to a player
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
pub struct PlayerInputMapping {
    pub player_id: u32,
    pub primary_device: Option<InputDevice>,
    pub secondary_device: Option<InputDevice>, // For fallback/dual input
    pub enabled: bool,
}

impl Default for PlayerInputMapping {
    fn default() -> Self {
        Self {
            player_id: 0,
            primary_device: Some(InputDevice::Keyboard(KeyboardScheme::WASD)),
            secondary_device: None,
            enabled: true,
        }
    }
}

impl PlayerInputMapping {
    pub fn new(player_id: u32) -> Self {
        Self {
            player_id,
            ..Default::default()
        }
    }

    pub fn with_primary_device(mut self, device: InputDevice) -> Self {
        self.primary_device = Some(device);
        self
    }

    pub fn with_secondary_device(mut self, device: Option<InputDevice>) -> Self {
        self.secondary_device = device;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get the currently assigned device (primary takes precedence)
    pub fn get_active_device(&self) -> Option<&InputDevice> {
        self.primary_device
            .as_ref()
            .or(self.secondary_device.as_ref())
    }
}

/// Resource for tracking device assignments
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct InputDeviceAssignment {
    pub assignments: HashMap<u32, InputDevice>, // player_id -> device
    pub max_players: u32,
}

impl InputDeviceAssignment {
    pub fn new(max_players: u32) -> Self {
        Self {
            assignments: HashMap::new(),
            max_players,
        }
    }

    /// Assign a device to a player
    pub fn assign_device(&mut self, player_id: u32, device: InputDevice) {
        // Remove device from other players to prevent conflicts
        self.assignments
            .retain(|_, assigned_device| assigned_device != &device);

        // Assign to the specified player
        self.assignments.insert(player_id, device);

        info!(
            "Assigned device to player {}: {:?}",
            player_id,
            self.assignments.get(&player_id)
        );
    }

    /// Get the device assigned to a player
    pub fn get_device_for_player(&self, player_id: u32) -> Option<&InputDevice> {
        self.assignments.get(&player_id)
    }

    /// Check if a device is assigned to any player
    pub fn is_device_assigned(&self, device: &InputDevice) -> bool {
        self.assignments
            .values()
            .any(|assigned_device| assigned_device == device)
    }

    /// Get the player ID that has the device assigned
    pub fn get_player_for_device(&self, device: &InputDevice) -> Option<u32> {
        self.assignments
            .iter()
            .find(|(_, assigned_device)| *assigned_device == device)
            .map(|(player_id, _)| *player_id)
    }

    /// Remove device assignment from a player
    pub fn unassign_player(&mut self, player_id: u32) {
        self.assignments.remove(&player_id);
    }

    /// Clear all assignments
    pub fn clear(&mut self) {
        self.assignments.clear();
    }

    /// Get all assigned players
    pub fn get_assigned_players(&self) -> Vec<u32> {
        self.assignments.keys().copied().collect()
    }
}

/// Input configuration settings
#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct InputSettings {
    pub gamepad_deadzone: f32,
    pub movement_threshold: f32,
    pub auto_assign_devices: bool,
    pub allow_keyboard_sharing: bool, // Allow multiple players to use different keyboard schemes
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            gamepad_deadzone: 0.2,
            movement_threshold: 0.1,
            auto_assign_devices: true,
            allow_keyboard_sharing: true,
        }
    }
}

/// Events for input system
#[derive(Event, Debug, Clone)]
pub enum InputEvent {
    /// Device assigned to player
    DeviceAssigned { player_id: u32, device: InputDevice },
    /// Device unassigned from player
    DeviceUnassigned { player_id: u32 },
    /// Primary action pressed
    PrimaryAction { player_id: u32, source: InputSource },
    /// Secondary action pressed
    SecondaryAction { player_id: u32, source: InputSource },
    /// Movement input
    Movement {
        player_id: u32,
        direction: Vec2,
        source: InputSource,
    },
}

/// Component marker for input configuration UI
#[derive(Component)]
pub struct InputConfigurationMarker;
