use bevy::prelude::*;
use crate::core::language::types::LanguageCode;
use crate::{TEXT_DIALOG_NAME, TEXT_HINT, TEXT_WHITE, TEXT_SUBTITLE, TEXT_TITLE, TextStyleResource};

/// Component สำหรับ Text ที่ต้องการ auto-update เมื่อเปลี่ยนภาษา
#[derive(Component)]
pub struct LocalizedText {
    pub key: String,
    pub style_preset: TextStylePreset,
}

/// Component สำหรับ Text ที่มีข้อความคงที่
#[derive(Component)]
pub struct StaticText {
    pub style_preset: TextStylePreset,
}

/// Preset รูปแบบ text ที่ใช้บ่อย
#[derive(Clone, Debug)]
pub enum TextStylePreset {
    Title,           // ขนาดใหญ่, bold, สีทอง
    Subtitle,        // ขนาดกลาง, regular, สีขาวอ่อน
    Body,            // ขนาดปกติ, regular, สีขาว
    Button,          // ขนาดปกติ, regular, สีขาว
    DialogName,      // ขนาดกลาง, bold, สีทอง
    DialogText,      // ขนาดปกติ, regular, สีขาว
    Hint,            // ขนาดเล็ก, regular, สีเทา
    Custom(f32, bool, Color), // (size, is_bold, color)
}

impl TextStylePreset {
    pub fn to_style(&self, text_styles: &TextStyleResource, language: &LanguageCode) -> TextStyle {
        match self {
            TextStylePreset::Title => text_styles.bold(language, 48.0, TEXT_TITLE),
            TextStylePreset::Subtitle => text_styles.regular(language, 24.0, TEXT_SUBTITLE),
            TextStylePreset::Body => text_styles.regular(language, 18.0, TEXT_WHITE),
            TextStylePreset::Button => text_styles.regular(language, 20.0, TEXT_WHITE),
            TextStylePreset::DialogName => text_styles.bold(language, 32.0, TEXT_DIALOG_NAME),
            TextStylePreset::DialogText => text_styles.regular(language, 30.0, TEXT_WHITE),
            TextStylePreset::Hint => text_styles.regular(language, 16.0, TEXT_HINT),
            TextStylePreset::Custom(size, is_bold, color) => {
                if *is_bold {
                    text_styles.bold(language, *size, *color)
                } else {
                    text_styles.regular(language, *size, *color)
                }
            }
        }
    }
}