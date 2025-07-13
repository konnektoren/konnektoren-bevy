use super::{
    ChallengeAsset, KonnektorenAssetLoader, KonnektorenAssetRegistry, KonnektorenAssetsPlugin,
    LevelAsset,
};
use bevy::asset::AssetPlugin;
use bevy::prelude::*;
use std::time::Duration;

fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(), // Add the AssetPlugin to provide AssetServer
    ))
    .add_plugins(KonnektorenAssetsPlugin);
    app
}

#[test]
fn test_plugin_initialization() {
    let app = create_test_app();

    // Check that asset types are registered
    assert!(app.world().contains_resource::<Assets<ChallengeAsset>>());
    assert!(app.world().contains_resource::<Assets<LevelAsset>>());
    assert!(app.world().contains_resource::<KonnektorenAssetRegistry>());
}

#[test]
fn test_asset_registry() {
    let app = create_test_app();

    // Get the registry
    let registry = app.world().resource::<KonnektorenAssetRegistry>();

    // Initially empty
    assert!(registry.challenges.is_empty());
    assert!(registry.levels.is_empty());
    assert!(registry.get_loaded_challenges().is_empty());
    assert!(registry.get_loaded_levels().is_empty());
}

#[test]
fn test_asset_loading_app() {
    let mut app = create_test_app();

    // Use the helper trait to load assets on App
    let _challenge_handle = app.load_challenge("test", "test.yml");
    let _level_handle = app.load_level("test_level", "test.level.yml");

    // Check that handles are registered
    let registry = app.world().resource::<KonnektorenAssetRegistry>();
    assert!(registry.get_challenge_handle("test").is_some());
    assert!(registry.get_level_handle("test_level").is_some());
    assert!(!registry.is_challenge_loaded("test")); // Not loaded yet (no actual file)
    assert!(!registry.is_level_loaded("test_level")); // Not loaded yet (no actual file)
}

#[cfg(feature = "assets")]
#[test]
fn test_challenge_asset_creation() {
    use konnektoren_core::challenges::{
        challenge_type::ChallengeType, multiple_choice::MultipleChoice,
    };

    let challenge_type = ChallengeType::MultipleChoice(MultipleChoice {
        id: "test".to_string(),
        name: "Test Challenge".to_string(),
        lang: "en".to_string(),
        options: vec![],
        questions: vec![],
    });

    let asset = ChallengeAsset {
        challenge_type,
        file_path: "test.yml".to_string(),
    };

    assert_eq!(asset.id(), "test");
    assert_eq!(asset.name(), "Test Challenge");
    assert_eq!(asset.file_path, "test.yml");
}

#[cfg(feature = "assets")]
#[test]
fn test_level_asset_creation() {
    use konnektoren_core::game::GamePath;

    let game_path = GamePath {
        id: "test-level".to_string(),
        name: "Test Level".to_string(),
        challenges: vec![],
        map: None, // Add the missing field
    };

    let asset = LevelAsset {
        game_path,
        file_path: "test.level.yml".to_string(),
    };

    assert_eq!(asset.id(), "test-level");
    assert_eq!(asset.name(), "Test Level");
    assert_eq!(asset.file_path, "test.level.yml");
    assert!(asset.get_challenge_ids().is_empty());
}

// Integration test that requires actual asset files
#[test]
fn test_real_asset_loading() {
    let mut app = create_test_app();

    // Load real assets (requires the asset files to exist)
    app.load_common_assets();

    // Run a few update cycles to allow asset loading
    for _ in 0..10 {
        app.update();
        std::thread::sleep(Duration::from_millis(10));
    }

    let registry = app.world().resource::<KonnektorenAssetRegistry>();

    // Check that assets were registered
    assert!(registry.get_challenge_handle("articles").is_some());
    assert!(registry.get_level_handle("a1").is_some());

    // Note: Assets might not be loaded yet due to async loading
    // In a real scenario, you'd wait for asset events or use asset loading states
}

// Test the asset registry methods more thoroughly
#[test]
fn test_asset_registry_methods() {
    let mut app = create_test_app();

    // Load some test assets
    let _challenge_handle = app.load_challenge("test_challenge", "test.yml");
    let _level_handle = app.load_level("test_level", "test.level.yml");

    // Get the registry and test its methods
    let registry = app.world().resource::<KonnektorenAssetRegistry>();

    // Test handle retrieval
    assert!(registry.get_challenge_handle("test_challenge").is_some());
    assert!(registry.get_level_handle("test_level").is_some());
    assert!(registry.get_challenge_handle("nonexistent").is_none());
    assert!(registry.get_level_handle("nonexistent").is_none());

    // Test loading status (should be false since files don't exist)
    assert!(!registry.is_challenge_loaded("test_challenge"));
    assert!(!registry.is_level_loaded("test_level"));

    // Test collections
    assert!(registry.get_loaded_challenges().is_empty());
    assert!(registry.get_loaded_levels().is_empty());
}

// Test the update system behavior
#[test]
fn test_update_system() {
    let mut app = create_test_app();

    // Load test assets
    app.load_challenge("test_challenge", "test.yml");
    app.load_level("test_level", "test.level.yml");

    // Run one update cycle
    app.update();

    // The update system should have run, but assets won't be loaded since files don't exist
    // This test just ensures the system doesn't crash
    let registry = app.world().resource::<KonnektorenAssetRegistry>();
    assert!(!registry.is_challenge_loaded("test_challenge"));
    assert!(!registry.is_level_loaded("test_level"));
}
