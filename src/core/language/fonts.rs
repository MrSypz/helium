use bevy::prelude::*;
use crate::core::language::types::LanguageCode;

/// Font resource สำหรับจัดการ fonts ของแต่ละภาษา
#[derive(Resource)]
pub struct FontResource {
    // Thai fonts
    pub thai_regular: Handle<Font>,
    pub thai_bold: Handle<Font>,

    // English fonts
    pub english_regular: Handle<Font>,
    pub english_bold: Handle<Font>,

    // Japanese fonts
    pub japanese_regular: Handle<Font>,
    pub japanese_bold: Handle<Font>,
}

impl FontResource {
    /// สร้าง FontResource ใหม่
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            // Thai fonts - NotoSansThai
            thai_regular: asset_server.load("fonts/thai/NotoSansThai-Regular.ttf"),
            thai_bold: asset_server.load("fonts/thai/NotoSansThai-Bold.ttf"),

            // English fonts - Roboto หรือ font อื่นสำหรับ English
            english_regular: asset_server.load("fonts/thai/NotoSansThai-Regular.ttf"),
            english_bold: asset_server.load("fonts/thai/NotoSansThai-Bold.ttf"),

            // Japanese fonts - NotoSansJP
            japanese_regular: asset_server.load("fonts/japanese/NotoSansJP-Regular.ttf"),
            japanese_bold: asset_server.load("fonts/japanese/NotoSansJP-Bold.ttf"),
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

/// โหลด fonts ทั้งหมด
pub fn setup_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font_resource = FontResource::new(&asset_server);
    commands.insert_resource(font_resource);
}

/// Helper function สำหรับดึง font styles ตามภาษา
pub fn get_font_styles(
    font_resource: &FontResource,
    language: &LanguageCode,
    regular_size: f32,
    bold_size: f32,
    color: Color,
) -> (TextStyle, TextStyle) {
    let regular_style = TextStyle {
        font: font_resource.get_regular_font(language),
        font_size: regular_size,
        color,
    };

    let bold_style = TextStyle {
        font: font_resource.get_bold_font(language),
        font_size: bold_size,
        color,
    };

    (regular_style, bold_style)
}