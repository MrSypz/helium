use bevy::prelude::*;
use crate::core::language::manager::{LanguageResource, LanguageChangeEvent};
use crate::core::language::types::LanguageCode;
use crate::core::resources::VNState;

/// Sync ระหว่าง Language System กับ VNState
pub fn sync_language_with_vn_state(
    mut language_events: EventReader<LanguageChangeEvent>,
    mut vn_state: ResMut<VNState>,
) {
    for event in language_events.read() {
        vn_state.sync_with_language_system(&event.new_language);
    }
}

/// Sync ระหว่าง VNState กับ Language System (กรณี external change)
pub fn sync_vn_state_with_language(
    vn_state: Res<VNState>,
    mut language_resource: ResMut<LanguageResource>,
    mut language_events: EventWriter<LanguageChangeEvent>,
    mut last_vn_language: Local<String>,
) {
    // ตรวจสอบว่า VNState language เปลี่ยนหรือไม่
    if *last_vn_language != vn_state.language {
        *last_vn_language = vn_state.language.clone();

        if let Some(language_code) = string_to_language_code(&vn_state.language) {
            if language_resource.current_language != language_code {
                if language_resource.change_language(language_code.clone()) {
                    language_events.send(LanguageChangeEvent {
                        new_language: language_code
                    });
                }
            }
        }
    }
}

/// Helper function แปลง string เป็น LanguageCode
fn string_to_language_code(lang_str: &str) -> Option<LanguageCode> {
    match lang_str {
        "thai" => Some(LanguageCode::Thai),
        "english" => Some(LanguageCode::English),
        "japanese" => Some(LanguageCode::Japanese),
        _ => None,
    }
}