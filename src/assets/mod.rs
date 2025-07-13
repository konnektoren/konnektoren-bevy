pub mod challenge_asset;
pub mod level_asset;

use bevy::prelude::*;
pub use challenge_asset::*;
pub use level_asset::*;
use std::collections::HashMap;

/// Plugin for loading Konnektoren assets (challenges, levels)
/// This plugin is focused on data loading only - no game logic
pub struct KonnektorenAssetsPlugin;

impl Plugin for KonnektorenAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register asset types and loaders
            .init_asset::<ChallengeAsset>()
            .init_asset_loader::<ChallengeAssetLoader>()
            .init_asset::<LevelAsset>()
            .init_asset_loader::<LevelAssetLoader>()
            // Initialize shared asset registry
            .init_resource::<KonnektorenAssetRegistry>()
            // Add asset tracking system
            .add_systems(Update, update_asset_registry);
    }
}

/// Central registry for all Konnektoren assets
/// Maps logical IDs to asset handles for easy access
#[derive(Resource, Default, Debug)]
pub struct KonnektorenAssetRegistry {
    pub challenges: HashMap<String, Handle<ChallengeAsset>>,
    pub levels: HashMap<String, Handle<LevelAsset>>,
    pub loaded_challenges: HashMap<String, bool>,
    pub loaded_levels: HashMap<String, bool>,
    // Add strong references to keep assets alive
    pub challenge_holders: HashMap<String, Handle<ChallengeAsset>>,
    pub level_holders: HashMap<String, Handle<LevelAsset>>,
}

impl KonnektorenAssetRegistry {
    /// Register a challenge asset with a logical ID
    pub fn register_challenge(&mut self, id: String, handle: Handle<ChallengeAsset>) {
        // Store both in the registry and as a holder to prevent unloading
        self.challenges.insert(id.clone(), handle.clone());
        self.challenge_holders.insert(id, handle);
    }

    /// Register a level asset with a logical ID
    pub fn register_level(&mut self, id: String, handle: Handle<LevelAsset>) {
        // Store both in the registry and as a holder to prevent unloading
        self.levels.insert(id.clone(), handle.clone());
        self.level_holders.insert(id, handle);
    }

    /// Get a challenge handle by ID
    pub fn get_challenge_handle(&self, id: &str) -> Option<&Handle<ChallengeAsset>> {
        self.challenges.get(id)
    }

    /// Get a level handle by ID
    pub fn get_level_handle(&self, id: &str) -> Option<&Handle<LevelAsset>> {
        self.levels.get(id)
    }

    /// Check if a challenge is loaded
    pub fn is_challenge_loaded(&self, id: &str) -> bool {
        self.loaded_challenges.get(id).copied().unwrap_or(false)
    }

    /// Check if a level is loaded
    pub fn is_level_loaded(&self, id: &str) -> bool {
        self.loaded_levels.get(id).copied().unwrap_or(false)
    }

    /// Get all loaded challenge IDs
    pub fn get_loaded_challenges(&self) -> Vec<String> {
        self.loaded_challenges
            .iter()
            .filter_map(|(id, &loaded)| if loaded { Some(id.clone()) } else { None })
            .collect()
    }

    /// Get all loaded level IDs
    pub fn get_loaded_levels(&self) -> Vec<String> {
        self.loaded_levels
            .iter()
            .filter_map(|(id, &loaded)| if loaded { Some(id.clone()) } else { None })
            .collect()
    }

    /// Check if all registered assets are loaded
    pub fn are_all_assets_loaded(&self) -> bool {
        let all_challenges_loaded = self
            .challenges
            .keys()
            .all(|id| self.is_challenge_loaded(id));
        let all_levels_loaded = self.levels.keys().all(|id| self.is_level_loaded(id));

        all_challenges_loaded && all_levels_loaded && !self.challenges.is_empty()
    }

    /// Get loading progress (0.0 to 1.0)
    pub fn get_loading_progress(&self) -> f32 {
        let total_assets = self.challenges.len() + self.levels.len();
        if total_assets == 0 {
            return 0.0;
        }

        let loaded_challenges = self
            .loaded_challenges
            .values()
            .filter(|&&loaded| loaded)
            .count();
        let loaded_levels = self
            .loaded_levels
            .values()
            .filter(|&&loaded| loaded)
            .count();
        let loaded_total = loaded_challenges + loaded_levels;

        loaded_total as f32 / total_assets as f32
    }
}

/// System to update the asset registry when assets finish loading
fn update_asset_registry(
    mut registry: ResMut<KonnektorenAssetRegistry>,
    challenge_assets: Res<Assets<ChallengeAsset>>,
    level_assets: Res<Assets<LevelAsset>>,
) {
    // Collect changes first to avoid borrowing conflicts
    let mut challenge_updates = Vec::new();
    let mut level_updates = Vec::new();

    // Check challenge loading status
    for (id, handle) in &registry.challenges {
        let is_loaded = challenge_assets.get(handle).is_some();
        let was_loaded = registry.loaded_challenges.get(id).copied().unwrap_or(false);

        if is_loaded && !was_loaded {
            challenge_updates.push(id.clone());
        }
    }

    // Check level loading status
    for (id, handle) in &registry.levels {
        let is_loaded = level_assets.get(handle).is_some();
        let was_loaded = registry.loaded_levels.get(id).copied().unwrap_or(false);

        if is_loaded && !was_loaded {
            level_updates.push(id.clone());
        }
    }

    // Apply updates
    for id in challenge_updates {
        registry.loaded_challenges.insert(id.clone(), true);
        info!("Challenge '{}' finished loading and is held in memory", id);
    }

    for id in level_updates {
        registry.loaded_levels.insert(id.clone(), true);
        info!("Level '{}' finished loading and is held in memory", id);
    }

    // Log overall progress periodically
    if !registry.challenges.is_empty() || !registry.levels.is_empty() {
        let progress = registry.get_loading_progress();
        if progress > 0.0 && progress < 1.0 {
            debug!("Asset loading progress: {:.1}%", progress * 100.0);
        }
    }
}

/// Helper trait for easy asset loading
pub trait KonnektorenAssetLoader {
    /// Load a challenge asset by ID and path
    fn load_challenge(&mut self, id: &str, path: &str) -> Handle<ChallengeAsset>;

    /// Load a level asset by ID and path
    fn load_level(&mut self, id: &str, path: &str) -> Handle<LevelAsset>;

    /// Load common Konnektoren assets
    fn load_common_assets(&mut self) -> &mut Self;
}

impl KonnektorenAssetLoader for App {
    fn load_challenge(&mut self, id: &str, path: &str) -> Handle<ChallengeAsset> {
        let asset_server = self.world().resource::<AssetServer>();
        let handle = asset_server.load(path);

        let mut registry = self.world_mut().resource_mut::<KonnektorenAssetRegistry>();
        registry.register_challenge(id.to_string(), handle.clone());

        info!(
            "Registered challenge '{}' from '{}' (handle will be held)",
            id, path
        );
        handle
    }

    fn load_level(&mut self, id: &str, path: &str) -> Handle<LevelAsset> {
        let asset_server = self.world().resource::<AssetServer>();
        let handle = asset_server.load(path);

        let mut registry = self.world_mut().resource_mut::<KonnektorenAssetRegistry>();
        registry.register_level(id.to_string(), handle.clone());

        info!(
            "Registered level '{}' from '{}' (handle will be held)",
            id, path
        );
        handle
    }

    fn load_common_assets(&mut self) -> &mut Self {
        // Load common challenges
        self.load_challenge("articles", "challenges/articles.yml");

        // Load common levels
        self.load_level("a1", "a1.level.yml");

        self
    }
}

#[cfg(test)]
mod tests;
