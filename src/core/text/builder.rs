use bevy::prelude::*;
use crate::core::language::manager::{LanguageResource, get_text};
use crate::core::language::types::LanguagePack;
use super::components::*;
use super::styles::*;

/// Builder สำหรับสร้าง Text components อย่างง่าย
pub struct TextBuilder;

impl TextBuilder {
    /// สร้าง localized text (จะอัพเดตเมื่อเปลี่ยนภาษา)
    pub fn localized(
        commands: &mut Commands,
        text_key: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        language_packs: &Assets<LanguagePack>,
        text_styles: &TextStyleResource,
    ) -> Entity {
        let text_value = get_text(language_resource, language_packs, text_key);
        let style = preset.to_style(text_styles, &language_resource.current_language);

        commands.spawn((
            TextBundle::from_section(text_value, style),
            LocalizedText {
                key: text_key.to_string(),
                style_preset: preset,
            },
        )).id()
    }

    /// สร้าง localized text พร้อม additional components
    pub fn localized_with_components<T: Bundle>(
        commands: &mut Commands,
        text_key: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        language_packs: &Assets<LanguagePack>,
        text_styles: &TextStyleResource,
        additional_components: T,
    ) -> Entity {
        let text_value = get_text(language_resource, language_packs, text_key);
        let style = preset.to_style(text_styles, &language_resource.current_language);

        commands.spawn((
            TextBundle::from_section(text_value, style),
            LocalizedText {
                key: text_key.to_string(),
                style_preset: preset,
            },
            additional_components,
        )).id()
    }

    /// สร้าง static text (ไม่อัพเดตเมื่อเปลี่ยนภาษา)
    pub fn static_text(
        commands: &mut Commands,
        text: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        text_styles: &TextStyleResource,
    ) -> Entity {
        let style = preset.to_style(text_styles, &language_resource.current_language);

        commands.spawn((
            TextBundle::from_section(text.to_string(), style),
            StaticText {
                style_preset: preset,
            },
        )).id()
    }

    /// สร้าง static text พร้อม additional components
    pub fn static_text_with_components<T: Bundle>(
        commands: &mut Commands,
        text: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        text_styles: &TextStyleResource,
        additional_components: T,
    ) -> Entity {
        let style = preset.to_style(text_styles, &language_resource.current_language);

        commands.spawn((
            TextBundle::from_section(text.to_string(), style),
            StaticText {
                style_preset: preset,
            },
            additional_components,
        )).id()
    }

    /// สร้าง localized text ใน parent (กรณีใช้กับ with_children)
    pub fn localized_child(
        parent: &mut ChildBuilder,
        text_key: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        language_packs: &Assets<LanguagePack>,
        text_styles: &TextStyleResource,
    ) -> Entity {
        let text_value = get_text(language_resource, language_packs, text_key);
        let style = preset.to_style(text_styles, &language_resource.current_language);

        parent.spawn((
            TextBundle::from_section(text_value, style),
            LocalizedText {
                key: text_key.to_string(),
                style_preset: preset,
            },
        )).id()
    }

    /// สร้าง localized text ใน parent พร้อม additional components
    pub fn localized_child_with_components<T: Bundle>(
        parent: &mut ChildBuilder,
        text_key: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        language_packs: &Assets<LanguagePack>,
        text_styles: &TextStyleResource,
        additional_components: T,
    ) -> Entity {
        let text_value = get_text(language_resource, language_packs, text_key);
        let style = preset.to_style(text_styles, &language_resource.current_language);

        parent.spawn((
            TextBundle::from_section(text_value, style),
            LocalizedText {
                key: text_key.to_string(),
                style_preset: preset,
            },
            additional_components,
        )).id()
    }

    /// สร้าง static text ใน parent
    pub fn static_child(
        parent: &mut ChildBuilder,
        text: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        text_styles: &TextStyleResource,
    ) -> Entity {
        let style = preset.to_style(text_styles, &language_resource.current_language);

        parent.spawn((
            TextBundle::from_section(text.to_string(), style),
            StaticText {
                style_preset: preset,
            },
        )).id()
    }

    /// สร้าง static text ใน parent พร้อม additional components
    pub fn static_child_with_components<T: Bundle>(
        parent: &mut ChildBuilder,
        text: &str,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        text_styles: &TextStyleResource,
        additional_components: T,
    ) -> Entity {
        let style = preset.to_style(text_styles, &language_resource.current_language);

        parent.spawn((
            TextBundle::from_section(text.to_string(), style),
            StaticText {
                style_preset: preset,
            },
            additional_components,
        )).id()
    }

    /// สร้าง text พร้อม custom style
    pub fn custom_child(
        parent: &mut ChildBuilder,
        text: &str,
        style: Style,
        preset: TextStylePreset,
        language_resource: &LanguageResource,
        text_styles: &TextStyleResource,
    ) -> Entity {
        let text_style = preset.to_style(text_styles, &language_resource.current_language);

        parent.spawn((
            TextBundle::from_section(text.to_string(), text_style)
                .with_style(style),
            StaticText {
                style_preset: preset,
            },
        )).id()
    }
}

/// System สำหรับ auto-update localized text เมื่อเปลี่ยนภาษา
pub fn update_localized_text(
    mut language_events: EventReader<crate::core::language::manager::LanguageChangeEvent>,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    text_styles: Res<TextStyleResource>,
    mut localized_query: Query<(&mut Text, &LocalizedText)>,
    mut static_query: Query<(&mut Text, &StaticText), Without<LocalizedText>>,
) {
    for _event in language_events.read() {
        // Update localized text (both content and style)
        for (mut text, localized) in localized_query.iter_mut() {
            let new_content = get_text(&language_resource, &language_packs, &localized.key);
            let new_style = localized.style_preset.to_style(&text_styles, &language_resource.current_language);

            text.sections[0].value = new_content;
            text.sections[0].style = new_style;
        }

        // Update static text (only style, content stays the same)
        for (mut text, static_text) in static_query.iter_mut() {
            let new_style = static_text.style_preset.to_style(&text_styles, &language_resource.current_language);
            text.sections[0].style = new_style;
        }
    }
}

/// Helper macros สำหรับสร้าง text ได้ง่ายขึ้น
#[macro_export]
macro_rules! localized_text {
    ($commands:expr, $key:expr, $preset:expr, $lang_res:expr, $lang_packs:expr, $text_styles:expr) => {
        crate::core::text::builder::TextBuilder::localized(
            $commands, $key, $preset, $lang_res, $lang_packs, $text_styles
        )
    };
}

#[macro_export]
macro_rules! static_text {
    ($commands:expr, $text:expr, $preset:expr, $lang_res:expr, $text_styles:expr) => {
        crate::core::text::builder::TextBuilder::static_text(
            $commands, $text, $preset, $lang_res, $text_styles
        )
    };
}

pub use localized_text;
pub use static_text;