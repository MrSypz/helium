use bevy::prelude::*;

/// Utility สำหรับการโหลด Asset
#[derive(Clone)]
pub struct Identifier {
    path: String,
}

impl Identifier {
    pub fn new(asset: &str, path: &str) -> Self {
        Self {
            path: format!("{}/{}", asset, path),
        }
    }

    pub fn load<T: Asset>(&self, asset_server: &AssetServer) -> Handle<T> {
        asset_server.load(&self.path)
    }
}

pub fn of(asset: &str, path: &str) -> Identifier {
    Identifier::new(asset, path)
}

pub fn texture(path: &str) -> Identifier {
    of("textures", path)
}

pub fn dialog(path: &str) -> Identifier {
    of("dialogs", path)
}