use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

#[cfg(feature = "assets")]
use konnektoren_core::game::GamePath;
#[cfg(feature = "assets")]
use serde_yaml;
#[cfg(feature = "assets")]
use thiserror::Error;

/// Asset representation of a game level
#[derive(Asset, TypePath, Debug, Clone)]
pub struct LevelAsset {
    #[cfg(feature = "assets")]
    pub game_path: GamePath,
    #[cfg(not(feature = "assets"))]
    pub game_path: String, // Fallback
    pub file_path: String,
}

#[cfg(feature = "assets")]
impl LevelAsset {
    /// Get the level ID
    pub fn id(&self) -> &str {
        &self.game_path.id
    }

    /// Get the level name
    pub fn name(&self) -> &str {
        &self.game_path.name
    }

    /// Get all challenge IDs referenced in this level
    pub fn get_challenge_ids(&self) -> Vec<String> {
        self.game_path
            .challenges
            .iter()
            .map(|c| c.challenge.clone())
            .collect()
    }
}

#[cfg(not(feature = "assets"))]
impl LevelAsset {
    /// Get the level ID (fallback)
    pub fn id(&self) -> &str {
        &self.game_path
    }

    /// Get the level name (fallback)
    pub fn name(&self) -> &str {
        &self.game_path
    }

    /// Get challenge IDs (fallback)
    pub fn get_challenge_ids(&self) -> Vec<String> {
        vec![]
    }
}

/// Loader for level files in YAML format
#[derive(Default)]
pub struct LevelAssetLoader;

/// Possible errors that can be produced by LevelAssetLoader
#[non_exhaustive]
#[derive(Debug, Error)]
#[cfg(feature = "assets")]
pub enum LevelAssetLoaderError {
    /// An IO Error
    #[error("Could not load level asset: {0}")]
    Io(#[from] std::io::Error),

    /// A YAML parsing error
    #[error("Could not parse YAML level: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

#[cfg(not(feature = "assets"))]
#[derive(Debug)]
pub enum LevelAssetLoaderError {
    Io(std::io::Error),
    YamlError(String),
}

#[cfg(not(feature = "assets"))]
impl std::fmt::Display for LevelAssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LevelAssetLoaderError::Io(e) => write!(f, "IO Error: {}", e),
            LevelAssetLoaderError::YamlError(e) => write!(f, "YAML Error: {}", e),
        }
    }
}

#[cfg(not(feature = "assets"))]
impl std::error::Error for LevelAssetLoaderError {}

#[cfg(feature = "assets")]
impl AssetLoader for LevelAssetLoader {
    type Asset = LevelAsset;
    type Settings = ();
    type Error = LevelAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let game_path = serde_yaml::from_slice::<GamePath>(&bytes)?;
        let file_path = load_context.path().to_string_lossy().to_string();

        info!(
            "Loaded level '{}' ({}) with {} challenges from {}",
            game_path.name,
            game_path.id,
            game_path.challenges.len(),
            file_path
        );

        Ok(LevelAsset {
            game_path,
            file_path,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level.yml", "level.yaml"]
    }
}

#[cfg(not(feature = "assets"))]
impl AssetLoader for LevelAssetLoader {
    type Asset = LevelAsset;
    type Settings = ();
    type Error = LevelAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader
            .read_to_end(&mut bytes)
            .await
            .map_err(LevelAssetLoaderError::Io)?;

        let file_path = load_context.path().to_string_lossy().to_string();
        let game_path = "unknown".to_string(); // Fallback

        Ok(LevelAsset {
            game_path,
            file_path,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["level.yml", "level.yaml"]
    }
}
