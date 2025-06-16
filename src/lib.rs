#[cfg(feature = "theme")]
pub mod theme;

#[cfg(feature = "ui")]
pub mod ui;

#[cfg(feature = "screens")]
pub mod screens;

#[cfg(feature = "settings")]
pub mod settings;

#[cfg(feature = "theme")]
pub use theme::*;

#[cfg(feature = "ui")]
pub use ui::*;

#[cfg(feature = "settings")]
pub use settings::*;

#[cfg(feature = "screens")]
pub use screens::*;

pub mod prelude {
    #[cfg(feature = "theme")]
    pub use crate::theme::*;

    #[cfg(feature = "ui")]
    pub use crate::ui::*;

    #[cfg(feature = "settings")]
    pub use crate::settings::*;

    #[cfg(feature = "screens")]
    pub use crate::screens::*;
}
