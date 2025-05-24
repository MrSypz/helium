use bevy::prelude::*;
use crate::core::language::types::LanguageCode;

/// ระบบสร้าง TextStyle ที่รวม font + language + size + color
#[derive(Resource)]
pub struct TextStyleResource {
    thai_regular: Handle<Font>,
    thai_bold: Handle<Font>,
    english_regular: Handle<Font>,
    english_bold: Handle<Font>,
    japanese_regular: Handle<Font>,
    japanese_bold: Handle<Font>,
    initialized: bool,
}

impl Default for TextStyleResource {
    fn default() -> Self {
        Self {
            thai_regular: Handle::default(),
            thai_bold: Handle::default(),
            english_regular: Handle::default(),
            english_bold: Handle::default(),
            japanese_regular: Handle::default(),
            japanese_bold: Handle::default(),
            initialized: false,
        }
    }
}

impl TextStyleResource {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            thai_regular: asset_server.load("fonts/thai/NotoSansThai-Regular.ttf"),
            thai_bold: asset_server.load("fonts/thai/NotoSansThai-Bold.ttf"),
            english_regular: asset_server.load("fonts/thai/NotoSansThai-Regular.ttf"),
            english_bold: asset_server.load("fonts/thai/NotoSansThai-Bold.ttf"),
            japanese_regular: asset_server.load("fonts/japanese/NotoSansJP-Regular.ttf"),
            japanese_bold: asset_server.load("fonts/japanese/NotoSansJP-Bold.ttf"),
            initialized: true,
        }
    }

    /// Initialize fonts if not already done (lazy initialization)
    pub fn ensure_initialized(&mut self, asset_server: &AssetServer) {
        if !self.initialized {
            *self = Self::new(asset_server);
        }
    }

    /// สร้าง TextStyle สำหรับ regular font
    pub fn regular(&self, language: &LanguageCode, size: f32, color: Color) -> TextStyle {
        TextStyle {
            font: self.get_regular_font(language),
            font_size: size,
            color,
        }
    }

    /// สร้าง TextStyle สำหรับ bold font
    pub fn bold(&self, language: &LanguageCode, size: f32, color: Color) -> TextStyle {
        TextStyle {
            font: self.get_bold_font(language),
            font_size: size,
            color,
        }
    }

    /// ดึง regular font ตามภาษา
    pub fn get_regular_font(&self, language: &LanguageCode) -> Handle<Font> {
        match language {
            LanguageCode::Thai => self.thai_regular.clone(),
            LanguageCode::English => self.english_regular.clone(),
            LanguageCode::Japanese => self.japanese_regular.clone(),
        }
    }

    /// ดึง bold font ตามภาษา
    pub fn get_bold_font(&self, language: &LanguageCode) -> Handle<Font> {
        match language {
            LanguageCode::Thai => self.thai_bold.clone(),
            LanguageCode::English => self.english_bold.clone(),
            LanguageCode::Japanese => self.japanese_bold.clone(),
        }
    }
}

// Common text colors
pub const TEXT_WHITE: Color = Color::WHITE;
pub const TEXT_TITLE: Color = Color::srgb(1.0, 0.8, 0.2);
pub const TEXT_SUBTITLE: Color = Color::srgba(0.8, 0.8, 0.9, 0.8);
pub const TEXT_HINT: Color = Color::srgba(0.7, 0.7, 0.8, 0.7);
pub const TEXT_DIALOG_NAME: Color = Color::srgb(1.0, 0.8, 0.2);

/// System สำหรับ lazy initialization ของ TextStyleResource
pub fn ensure_text_styles_initialized(
    mut text_styles: ResMut<TextStyleResource>,
    asset_server: Res<AssetServer>,
) {
    text_styles.ensure_initialized(&asset_server);
}
