mod panels;
mod screens;

pub use panels::*;

use crate::demo::DemoState;
use bevy::prelude::*;
use bevy_egui::EguiContexts;
use konnektoren_bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn demo_ui(
    mut contexts: EguiContexts,
    theme: Res<KonnektorenTheme>,
    responsive: Res<ResponsiveInfo>,
    demo_query: Query<&DemoState>,
    mut commands: Commands,
    active_splash_query: Query<Entity, With<ActiveSplash>>,
    active_about_query: Query<Entity, With<ActiveAbout>>,
    active_settings_query: Query<Entity, With<ActiveSettings>>,
    existing_about_configs: Query<Entity, With<AboutConfig>>,
    existing_settings_configs: Query<Entity, With<SettingsConfig>>,
) {
    if let Ok(demo_state) = demo_query.single() {
        // Don't show UI if any overlay is active
        if !active_splash_query.is_empty()
            || !active_about_query.is_empty()
            || !active_settings_query.is_empty()
        {
            return;
        }

        if !demo_state.splash_shown {
            return; // Wait for first splash to finish
        }

        render_side_panel(
            &mut contexts,
            &theme,
            &responsive,
            &mut commands,
            &active_splash_query,
            &active_about_query,
            &active_settings_query,
            &existing_about_configs,
            &existing_settings_configs,
        );

        render_main_panel(&mut contexts, &theme, &responsive, demo_state);
    }
}
