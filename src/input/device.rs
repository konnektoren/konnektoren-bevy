use bevy::prelude::*;

/// Resource to track available input devices
#[derive(Resource, Reflect, Default, Clone)]
#[reflect(Resource)]
pub struct AvailableInputDevices {
    pub gamepads: Vec<Entity>,
    pub mouse: bool,
    pub touch: bool,
    pub keyboard: bool,
}

impl AvailableInputDevices {
    pub fn get_available_devices(&self) -> Vec<InputDevice> {
        let mut devices = Vec::new();

        if self.keyboard {
            devices.push(InputDevice::Keyboard(KeyboardScheme::WASD));
            devices.push(InputDevice::Keyboard(KeyboardScheme::Arrows));
            devices.push(InputDevice::Keyboard(KeyboardScheme::IJKL));
        }

        for (index, _) in self.gamepads.iter().enumerate() {
            devices.push(InputDevice::Gamepad(index as u32));
        }

        if self.mouse {
            devices.push(InputDevice::Mouse);
        }

        if self.touch {
            devices.push(InputDevice::Touch);
        }

        devices
    }
}

#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum InputDevice {
    Keyboard(KeyboardScheme),
    Gamepad(u32),
    Mouse,
    Touch,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Reflect, Clone, Debug, PartialEq)]
pub enum KeyboardScheme {
    WASD,
    Arrows,
    IJKL,
    Custom {
        up: KeyCode,
        down: KeyCode,
        left: KeyCode,
        right: KeyCode,
    },
}

impl KeyboardScheme {
    pub fn get_keys(&self) -> (KeyCode, KeyCode, KeyCode, KeyCode) {
        match self {
            KeyboardScheme::WASD => (KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD),
            KeyboardScheme::Arrows => (
                KeyCode::ArrowUp,
                KeyCode::ArrowDown,
                KeyCode::ArrowLeft,
                KeyCode::ArrowRight,
            ),
            KeyboardScheme::IJKL => (KeyCode::KeyI, KeyCode::KeyK, KeyCode::KeyJ, KeyCode::KeyL),
            KeyboardScheme::Custom {
                up,
                down,
                left,
                right,
            } => (*up, *down, *left, *right),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            KeyboardScheme::WASD => "WASD",
            KeyboardScheme::Arrows => "Arrow Keys",
            KeyboardScheme::IJKL => "IJKL",
            KeyboardScheme::Custom { .. } => "Custom",
        }
    }
}

impl InputDevice {
    pub fn name(&self) -> String {
        match self {
            InputDevice::Keyboard(scheme) => format!("Keyboard ({})", scheme.name()),
            InputDevice::Gamepad(id) => format!("Gamepad {}", id + 1),
            InputDevice::Mouse => "Mouse".to_string(),
            InputDevice::Touch => "Touch".to_string(),
        }
    }

    pub fn is_available(&self, available_devices: &AvailableInputDevices) -> bool {
        match self {
            InputDevice::Keyboard(_) => available_devices.keyboard,
            InputDevice::Gamepad(id) => (*id as usize) < available_devices.gamepads.len(),
            InputDevice::Mouse => available_devices.mouse,
            InputDevice::Touch => available_devices.touch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_input_devices() {
        let devices = AvailableInputDevices {
            gamepads: vec![Entity::from_raw(1), Entity::from_raw(2)],
            mouse: true,
            touch: false,
            keyboard: true,
        };

        let available = devices.get_available_devices();
        assert_eq!(available.len(), 6);
        assert!(available.iter().any(|d| matches!(d, InputDevice::Mouse)));
        assert!(available
            .iter()
            .any(|d| matches!(d, InputDevice::Touch) == false));
    }
}
