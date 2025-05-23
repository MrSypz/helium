use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::asset::{AssetLoader, LoadContext, AsyncReadExt};
use bevy::utils::ConditionalSendFuture;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::future::Future;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DialogCharacter {
    pub name: String,
    pub display_name: HashMap<String, String>,
    #[serde(default)]
    pub sprite: String,
    #[serde(default)]
    pub positions: HashMap<String, Vec2>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DialogChoice {
    pub text: HashMap<String, String>,
    pub target_stage: usize,
    #[serde(default)]
    pub conditions: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CharacterState {
    pub name: String,
    pub position: String,
    pub expression: String,
    pub highlight: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DialogEntry {
    pub character: String,
    pub text: HashMap<String, String>,
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(default)]
    pub choices: Vec<DialogChoice>,
    #[serde(default)]
    pub auto_proceed: Option<usize>,
    #[serde(default)]
    pub character_states: Vec<CharacterState>,
    #[serde(default)]
    pub background: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, TypePath, Asset, Clone)]
pub struct DialogScene {
    pub characters: Vec<DialogCharacter>,
    pub entries: Vec<DialogEntry>,
    #[serde(default)]
    pub default_background: String,
}

#[derive(Default)]
pub struct DialogLoader;

impl AssetLoader for DialogLoader {
    type Asset = DialogScene;
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
            let dialog_scene: DialogScene = serde_json::from_slice(&bytes)?;
            Ok(dialog_scene)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["dialog.json"]
    }
}