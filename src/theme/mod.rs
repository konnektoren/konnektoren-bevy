pub mod colors;
mod plugin;
pub mod resource;

pub use colors::*;
use plugin::*;
pub use resource::*;

use bevy::prelude::*;

/// Main theme plugin that provides Konnektoren theme functionality
pub struct KonnektorenThemePlugin;

impl Plugin for KonnektorenThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiThemePlugin);
    }
}
