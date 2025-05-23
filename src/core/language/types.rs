use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::asset::{AssetLoader, LoadContext, AsyncReadExt};
use bevy::utils::ConditionalSendFuture;
use serde::{Deserialize, Serialize};
use std::future::Future;

/// Language pack structure
#[derive(Debug, Deserialize, Serialize, TypePath, Asset, Clone)]
pub struct LanguagePack {
    pub ui: UiTexts,
    pub dialog: DialogTexts,
    pub game: GameTexts,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UiTexts {
    pub start_game: String,
    pub settings: String,
    pub exit_game: String,
    pub loading: String,
    pub loading_subtitle: String,
    pub choose_action: String,
    pub game_title: String,
    pub game_subtitle: String,
    pub controls_help: String,
    pub settings_title: String,
    pub language_setting: String,
    pub resolution_setting: String,
    pub fullscreen_setting: String,
    pub enabled: String,
    pub disabled: String,
    pub back: String,
    pub apply_settings: String,
    pub change: String,
    pub toggle: String, // เพิ่มบรรทัดนี้
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DialogTexts {
    pub choose_action: String,
    pub continue_hint: String,
    pub language_indicator: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameTexts {
    pub narrator: String,
    pub you: String,
    pub unknown: String,
}

/// Language codes supported
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguageCode {
    Thai,      // th-th
    English,   // en-us
    Japanese,  // jp-jp
}

impl LanguageCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            LanguageCode::Thai => "th-th",
            LanguageCode::English => "en-us",
            LanguageCode::Japanese => "jp-jp",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "th-th" | "thai" => Some(LanguageCode::Thai),
            "en-us" | "english" => Some(LanguageCode::English),
            "jp-jp" | "japanese" => Some(LanguageCode::Japanese),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            LanguageCode::Thai => "ไทย",
            LanguageCode::English => "English",
            LanguageCode::Japanese => "日本語",
        }
    }
}

/// Language pack asset loader
#[derive(Default)]
pub struct LanguageLoader;

impl AssetLoader for LanguageLoader {
    type Asset = LanguagePack;
    type Settings = ();
    type Error = anyhow::Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture + Future<Output = Result<<Self as AssetLoader>::Asset, <Self as AssetLoader>::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let language_pack: LanguagePack = serde_json::from_slice(&bytes)?;
            Ok(language_pack)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}