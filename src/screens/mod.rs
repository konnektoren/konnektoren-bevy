pub mod about;
pub mod settings;
pub mod splash;

pub use about::*;
pub use settings::*;
pub use splash::*;

use bevy::prelude::*;

/// Main screens plugin that includes all screen functionality
pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SplashPlugin)
            .add_plugins(AboutPlugin)
            .add_plugins(SettingsPlugin)
            .add_event::<SplashDismissed>()
            .add_event::<AboutDismissed>()
            .add_event::<SettingsEvent>();

        info!("ScreensPlugin loaded with splash, about, and settings screen support");
    }
}
