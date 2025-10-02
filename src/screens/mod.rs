pub mod about;
pub mod credits;
pub mod settings;
pub mod splash;

pub use about::*;
pub use credits::*;
pub use settings::*;
pub use splash::*;

use bevy::prelude::*;

/// Main screens plugin that includes all screen functionality
pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SplashPlugin)
            .add_plugins(AboutPlugin)
            .add_plugins(CreditsPlugin)
            .add_plugins(SettingsScreenPlugin)
            .add_message::<SplashDismissed>()
            .add_message::<CreditsDismissed>()
            .add_message::<AboutDismissed>();

        info!("ScreensPlugin loaded with splash, about, and settings screen support");
    }
}
