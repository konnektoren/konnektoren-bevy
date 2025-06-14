pub mod responsive;
pub mod widgets;

pub use responsive::*;
pub use widgets::*;

use bevy::prelude::*;

/// Main UI plugin that includes responsive and widget functionality
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ResponsivePlugin);

        info!("UIPlugin loaded with responsive and widget support");
    }
}
