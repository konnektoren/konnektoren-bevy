use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

#[cfg(feature = "assets")]
use konnektoren_core::challenges::challenge_type::ChallengeType;
#[cfg(feature = "assets")]
use serde_yaml;
#[cfg(feature = "assets")]
use thiserror::Error;

/// Asset representation of a challenge
#[derive(Asset, TypePath, Debug, Clone)]
pub struct ChallengeAsset {
    #[cfg(feature = "assets")]
    pub challenge_type: ChallengeType,
    #[cfg(not(feature = "assets"))]
    pub challenge_type: String, // Fallback for when assets feature is disabled
    pub file_path: String,
}

#[cfg(feature = "assets")]
impl ChallengeAsset {
    /// Get the challenge ID
    pub fn id(&self) -> &str {
        self.challenge_type.id()
    }

    /// Get the challenge name
    pub fn name(&self) -> &str {
        self.challenge_type.name()
    }
}

#[cfg(not(feature = "assets"))]
impl ChallengeAsset {
    /// Get the challenge ID (fallback)
    pub fn id(&self) -> &str {
        &self.challenge_type
    }

    /// Get the challenge name (fallback)
    pub fn name(&self) -> &str {
        &self.challenge_type
    }
}

/// Loader for challenge files in YAML format
#[derive(Default)]
pub struct ChallengeAssetLoader;

/// Possible errors that can be produced by ChallengeAssetLoader
#[non_exhaustive]
#[derive(Debug, Error)]
#[cfg(feature = "assets")]
pub enum ChallengeAssetLoaderError {
    /// An IO Error
    #[error("Could not load challenge asset: {0}")]
    Io(#[from] std::io::Error),

    /// A YAML parsing error
    #[error("Could not parse YAML challenge: {0}")]
    YamlError(#[from] serde_yaml::Error),
}

#[cfg(not(feature = "assets"))]
#[derive(Debug)]
pub enum ChallengeAssetLoaderError {
    Io(std::io::Error),
    YamlError(String),
}

#[cfg(not(feature = "assets"))]
impl std::fmt::Display for ChallengeAssetLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChallengeAssetLoaderError::Io(e) => write!(f, "IO Error: {}", e),
            ChallengeAssetLoaderError::YamlError(e) => write!(f, "YAML Error: {}", e),
        }
    }
}

#[cfg(not(feature = "assets"))]
impl std::error::Error for ChallengeAssetLoaderError {}

#[cfg(feature = "assets")]
impl AssetLoader for ChallengeAssetLoader {
    type Asset = ChallengeAsset;
    type Settings = ();
    type Error = ChallengeAssetLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let challenge_type = serde_yaml::from_slice::<ChallengeType>(&bytes)?;
        let file_path = load_context.path().to_string_lossy().to_string();

        info!(
            "Loaded challenge '{}' ({}) from {}",
            challenge_type.name(),
            challenge_type.id(),
            file_path
        );

        Ok(ChallengeAsset {
            challenge_type,
            file_path,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }
}

#[cfg(not(feature = "assets"))]
impl AssetLoader for ChallengeAssetLoader {
    type Asset = ChallengeAsset;
    type Settings = ();
    type Error = ChallengeAssetLoaderError;

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
            .map_err(ChallengeAssetLoaderError::Io)?;

        let file_path = load_context.path().to_string_lossy().to_string();
        let challenge_type = "unknown".to_string(); // Fallback

        Ok(ChallengeAsset {
            challenge_type,
            file_path,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["yml", "yaml"]
    }
}
