pub mod splash;

pub use splash::*;

use bevy::prelude::*;

/// Main screens plugin that includes all screen functionality
pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SplashPlugin).add_event::<SplashDismissed>();

        info!("ScreensPlugin loaded with splash screen support");
    }
}
