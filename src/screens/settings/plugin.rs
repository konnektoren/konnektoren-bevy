use super::*;
use bevy::prelude::*;

/// Plugin for reusable settings screen functionality
pub struct SettingsScreenPlugin;

impl Plugin for SettingsScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SettingsScreenEvent>()
            .add_event::<ComponentSettingsEvent>()
            .add_systems(
                Update,
                (
                    check_settings_screen_config,
                    handle_settings_screen_events,
                    update_settings_screen_values,
                    cleanup_component_settings,
                    render_settings_screen_ui, // Move to Update
                ),
            )
            // Add input configuration plugin
            .add_plugins(InputConfigurationPlugin);

        // Add component-based settings systems if settings feature is enabled
        #[cfg(feature = "settings")]
        {
            app.add_systems(
                Update,
                (
                    check_component_settings,
                    process_pending_setting_updates,
                    render_component_settings_ui, // Move to Update
                ),
            );
        }
    }
}

/// Helper trait for easy settings screen setup
pub trait SettingsScreenExt {
    /// Add a settings screen with the given configuration
    fn spawn_settings_screen(&mut self, config: SettingsScreenConfig) -> Entity;

    /// Add a simple settings screen with basic audio settings
    fn spawn_simple_settings_screen(&mut self, title: impl Into<String>) -> Entity;

    /// Spawn a component-based settings screen that uses Setting components
    fn spawn_component_settings_screen(&mut self, title: impl Into<String>) -> Entity;

    /// Spawn input configuration screen
    fn spawn_input_configuration(&mut self, max_players: u32) -> Entity;
}

impl SettingsScreenExt for Commands<'_, '_> {
    fn spawn_settings_screen(&mut self, config: SettingsScreenConfig) -> Entity {
        self.spawn((Name::new("Settings Screen"), config)).id()
    }

    fn spawn_simple_settings_screen(&mut self, title: impl Into<String>) -> Entity {
        let audio_section = SettingsSection::audio_section();
        let config = SettingsScreenConfig::new(title).add_section(audio_section);
        self.spawn_settings_screen(config)
    }

    fn spawn_component_settings_screen(&mut self, title: impl Into<String>) -> Entity {
        self.spawn((
            Name::new("Component-Based Settings Trigger"),
            ActiveComponentSettings {
                title: title.into(),
                allow_dismissal: true,
                back_button_text: "Back".to_string(),
                navigation_state: ComponentSettingsNavigationState::default(),
            },
        ))
        .id()
    }

    fn spawn_input_configuration(&mut self, max_players: u32) -> Entity {
        self.spawn((
            Name::new("Input Configuration Screen"),
            ActiveInputConfiguration {
                max_players,
                current_players: max_players.min(4), // Default to 4 players max
            },
        ))
        .id()
    }
}
