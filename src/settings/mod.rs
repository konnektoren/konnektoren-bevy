pub mod builders;
pub mod components;
pub mod systems;

#[cfg(test)]
mod tests;

pub use builders::*;
pub use components::*;
pub use systems::*;

use bevy::prelude::*;

/// Main settings plugin that provides core settings functionality
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SettingChangedEvent>()
            .add_systems(Update, update_settings_from_components);
    }
}
