#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "theme")]
pub mod theme;

#[cfg(feature = "ui")]
pub mod ui;

#[cfg(feature = "screens")]
pub mod screens;

#[cfg(feature = "settings")]
pub mod settings;

pub mod prelude {
    #[cfg(feature = "theme")]
    pub use crate::theme::{KonnektorenThemePlugin, *};

    #[cfg(feature = "ui")]
    pub use crate::ui::{UIPlugin, *};

    #[cfg(feature = "settings")]
    pub use crate::settings::{builders::*, components::*, systems::*, SettingsPlugin};

    #[cfg(feature = "screens")]
    pub use crate::screens::{about::*, credits::*, settings::*, splash::*, ScreensPlugin};

    #[cfg(feature = "input")]
    pub use crate::input::{components::*, device::*, plugin::*, systems::*, InputPlugin};
}
