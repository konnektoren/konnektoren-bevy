use crate::demo::{DemoState, DemoType};
use bevy::prelude::*;
use konnektoren_bevy::prelude::SettingValue;
use konnektoren_bevy::prelude::*;

pub fn handle_splash_dismissed(
    mut splash_events: MessageReader<SplashDismissed>,
    mut demo_query: Query<&mut DemoState>,
) {
    for _event in splash_events.read() {
        if let Ok(mut demo_state) = demo_query.single_mut() {
            demo_state.splash_shown = true;
            demo_state.current_demo = DemoType::Complete;
            info!("Splash dismissed, showing main demo");
        }
    }
}

pub fn handle_about_dismissed(mut about_events: MessageReader<AboutDismissed>) {
    for event in about_events.read() {
        info!("About screen dismissed for entity {:?}", event.entity);
    }
}

pub fn handle_settings_events(mut settings_events: MessageReader<SettingsScreenEvent>) {
    for event in settings_events.read() {
        match event {
            SettingsScreenEvent::ValueChanged {
                entity: _,
                setting_id,
                value,
            } => {
                info!("Setting '{}' changed to {:?}", setting_id, value);
                // Here you would typically update your application's settings

                match (setting_id.as_str(), value) {
                    ("master_volume", SettingValue::Float(volume)) => {
                        info!("Master volume changed to: {:.1}", volume);
                    }
                    ("audio_enabled", SettingValue::Bool(enabled)) => {
                        info!("Audio enabled changed to: {}", enabled);
                    }
                    ("difficulty", SettingValue::Selection(index)) => {
                        let difficulties = ["Easy", "Normal", "Hard", "Expert"];
                        if let Some(difficulty) = difficulties.get(*index) {
                            info!("Difficulty changed to: {}", difficulty);
                        }
                    }
                    _ => {
                        info!("Unhandled setting change: {} = {:?}", setting_id, value);
                    }
                }
            }
            SettingsScreenEvent::Dismissed { entity } => {
                info!("Settings dismissed for entity {:?}", entity);
            }
            SettingsScreenEvent::Navigate { direction } => {
                info!("Settings navigation: {:?}", direction);
            }
        }
    }
}
