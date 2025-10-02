use bevy::prelude::*;
use std::hash::Hash; // Add this import

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

        // Always add keyboard schemes if keyboard is available
        if self.keyboard {
            devices.push(InputDevice::Keyboard(KeyboardScheme::WASD));
            devices.push(InputDevice::Keyboard(KeyboardScheme::Arrows));
            devices.push(InputDevice::Keyboard(KeyboardScheme::IJKL));

            // Add some common custom schemes
            devices.push(InputDevice::Keyboard(KeyboardScheme::Custom {
                up: KeyCode::KeyT,
                down: KeyCode::KeyG,
                left: KeyCode::KeyF,
                right: KeyCode::KeyH,
            }));
        }

        // Add gamepad devices
        for (index, _) in self.gamepads.iter().enumerate() {
            devices.push(InputDevice::Gamepad(index as u32));
        }

        // Add mouse if available
        if self.mouse {
            devices.push(InputDevice::Mouse);
        }

        // Add touch if available (typically on mobile/web)
        if self.touch {
            devices.push(InputDevice::Touch);
        }

        devices
    }

    /// Update device availability based on platform and capabilities
    pub fn update_availability(&mut self) {
        // Keyboard is almost always available except on some restricted platforms
        self.keyboard = true;

        // Mouse availability - true on desktop, false on mobile-only devices
        #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
        {
            self.mouse = true;
        }

        #[cfg(target_family = "wasm")]
        {
            // On web, mouse is usually available
            self.mouse = true;
            // Touch might be available on touch devices
            self.touch = true;
        }

        #[cfg(target_os = "android")]
        {
            self.mouse = false;
            self.touch = true;
        }

        #[cfg(target_os = "ios")]
        {
            self.mouse = false;
            self.touch = true;
        }

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
            target_family = "wasm",
            target_os = "android",
            target_os = "ios"
        )))]
        {
            // Default fallback
            self.mouse = true;
            self.touch = false;
        }
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
            KeyboardScheme::Custom { .. } => "Custom Keys",
        }
    }

    /// Get a user-friendly description of the key layout
    pub fn description(&self) -> String {
        match self {
            KeyboardScheme::WASD => "W/A/S/D keys".to_string(),
            KeyboardScheme::Arrows => "Arrow keys".to_string(),
            KeyboardScheme::IJKL => "I/J/K/L keys".to_string(),
            KeyboardScheme::Custom {
                up,
                down,
                left,
                right,
            } => {
                format!("{:?}/{:?}/{:?}/{:?}", up, down, left, right)
            }
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

    /// Get a detailed description of the input device
    pub fn description(&self) -> String {
        match self {
            InputDevice::Keyboard(scheme) => {
                format!("Keyboard - {}", scheme.description())
            }
            InputDevice::Gamepad(id) => {
                format!("Gamepad {} (D-Pad + Analog Stick)", id + 1)
            }
            InputDevice::Mouse => "Mouse (Click and drag for movement)".to_string(),
            InputDevice::Touch => "Touch (Tap and swipe gestures)".to_string(),
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

    /// Get the category of this input device for grouping in UI
    pub fn category(&self) -> InputDeviceCategory {
        match self {
            InputDevice::Keyboard(_) => InputDeviceCategory::Keyboard,
            InputDevice::Gamepad(_) => InputDeviceCategory::Gamepad,
            InputDevice::Mouse => InputDeviceCategory::Pointing,
            InputDevice::Touch => InputDeviceCategory::Touch,
        }
    }
}

/// Categories for grouping input devices in UI
/// Add Hash and Eq traits for HashMap usage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputDeviceCategory {
    Keyboard,
    Gamepad,
    Pointing, // Mouse, trackpad, etc.
    Touch,
}

impl InputDeviceCategory {
    pub fn name(&self) -> &'static str {
        match self {
            InputDeviceCategory::Keyboard => "Keyboard",
            InputDeviceCategory::Gamepad => "Gamepad",
            InputDeviceCategory::Pointing => "Mouse",
            InputDeviceCategory::Touch => "Touch",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            InputDeviceCategory::Keyboard => "⌨️",
            InputDeviceCategory::Gamepad => "🎮",
            InputDeviceCategory::Pointing => "🖱️",
            InputDeviceCategory::Touch => "👆",
        }
    }

    /// Get the display order for consistent UI layout
    pub fn order(&self) -> u8 {
        match self {
            InputDeviceCategory::Keyboard => 0,
            InputDeviceCategory::Gamepad => 1,
            InputDeviceCategory::Pointing => 2,
            InputDeviceCategory::Touch => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_input_devices() {
        let devices = AvailableInputDevices {
            gamepads: vec![
                Entity::from_raw_u32(1).unwrap(),
                Entity::from_raw_u32(2).unwrap(),
            ],
            mouse: true,
            touch: false,
            keyboard: true,
        };

        let available = devices.get_available_devices();

        // Should have keyboard schemes + gamepads + mouse
        assert!(available.len() >= 6); // 4 keyboard schemes + 2 gamepads + mouse

        // Check that we have keyboard schemes
        assert!(available
            .iter()
            .any(|d| matches!(d, InputDevice::Keyboard(KeyboardScheme::WASD))));
        assert!(available
            .iter()
            .any(|d| matches!(d, InputDevice::Keyboard(KeyboardScheme::Arrows))));
        assert!(available
            .iter()
            .any(|d| matches!(d, InputDevice::Keyboard(KeyboardScheme::IJKL))));

        // Check that we have gamepads
        assert!(available
            .iter()
            .any(|d| matches!(d, InputDevice::Gamepad(0))));
        assert!(available
            .iter()
            .any(|d| matches!(d, InputDevice::Gamepad(1))));

        // Check that we have mouse
        assert!(available.iter().any(|d| matches!(d, InputDevice::Mouse)));

        // Check that we don't have touch (disabled)
        assert!(!available.iter().any(|d| matches!(d, InputDevice::Touch)));
    }

    #[test]
    fn test_keyboard_schemes() {
        let wasd = KeyboardScheme::WASD;
        let arrows = KeyboardScheme::Arrows;
        let custom = KeyboardScheme::Custom {
            up: KeyCode::KeyT,
            down: KeyCode::KeyG,
            left: KeyCode::KeyF,
            right: KeyCode::KeyH,
        };

        assert_eq!(wasd.name(), "WASD");
        assert_eq!(arrows.name(), "Arrow Keys");
        assert_eq!(custom.name(), "Custom Keys");

        let (up, down, left, right) = wasd.get_keys();
        assert_eq!(
            (up, down, left, right),
            (KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD)
        );
    }

    #[test]
    fn test_device_categories() {
        let keyboard_device = InputDevice::Keyboard(KeyboardScheme::WASD);
        let gamepad_device = InputDevice::Gamepad(0);
        let mouse_device = InputDevice::Mouse;
        let touch_device = InputDevice::Touch;

        assert_eq!(keyboard_device.category(), InputDeviceCategory::Keyboard);
        assert_eq!(gamepad_device.category(), InputDeviceCategory::Gamepad);
        assert_eq!(mouse_device.category(), InputDeviceCategory::Pointing);
        assert_eq!(touch_device.category(), InputDeviceCategory::Touch);
    }

    #[test]
    fn test_category_hash() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(InputDeviceCategory::Keyboard, "keyboard");
        map.insert(InputDeviceCategory::Gamepad, "gamepad");

        assert_eq!(map.get(&InputDeviceCategory::Keyboard), Some(&"keyboard"));
        assert_eq!(map.get(&InputDeviceCategory::Gamepad), Some(&"gamepad"));
    }
}
