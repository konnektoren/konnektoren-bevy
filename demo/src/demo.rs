use bevy::prelude::*;
use konnektoren_bevy::prelude::*;

#[derive(Component)]
pub struct DemoState {
    pub current_demo: DemoType,
    pub splash_shown: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DemoType {
    Splash,
    Complete,
}

pub fn setup_demo(mut commands: Commands) {
    // Initialize Camera
    commands.spawn(Camera2d);
    // Initialize demo state
    commands.spawn(DemoState {
        current_demo: DemoType::Splash,
        splash_shown: false,
    });

    // Show initial splash screen
    commands.spawn_konnektoren_splash();
}
