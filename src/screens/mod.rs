pub mod about;
pub mod splash;

pub use about::*;
pub use splash::*;

use bevy::prelude::*;

/// Main screens plugin that includes all screen functionality
pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SplashPlugin)
            .add_plugins(AboutPlugin)
            .add_event::<SplashDismissed>()
            .add_event::<AboutDismissed>();

        info!("ScreensPlugin loaded with splash and about screen support");
    }
}
