use bevy::prelude::*;
use std::collections::HashMap;
use crate::core::language::types::{LanguagePack, LanguageCode};
use crate::core::resources::VNState;
use crate::util::identifier::language;

/// Language resource - จัดการภาษาทั้งหมด
#[derive(Resource)]
pub struct LanguageResource {
    pub packs: HashMap<LanguageCode, Handle<LanguagePack>>,
    pub current_language: LanguageCode,
    pub current_pack: Option<Handle<LanguagePack>>,
    pub loaded: bool,
    pub initialized: bool,
}

impl Default for LanguageResource {
    fn default() -> Self {
        Self {
            packs: HashMap::new(),
            current_language: LanguageCode::Thai, // default language
            current_pack: None,
            loaded: false,
            initialized: false,
        }
    }
}

impl LanguageResource {
    /// เปลี่ยนภาษา
    pub fn change_language(&mut self, language: LanguageCode) -> bool {
        if let Some(pack_handle) = self.packs.get(&language) {
            self.current_language = language;
            self.current_pack = Some(pack_handle.clone());
            true
        } else {
            false
        }
    }

    /// ได้ภาษาถัดไป (สำหรับ cycle)
    pub fn next_language(&self) -> LanguageCode {
        match self.current_language {
            LanguageCode::Thai => LanguageCode::English,
            LanguageCode::English => LanguageCode::Japanese,
            LanguageCode::Japanese => LanguageCode::Thai,
        }
    }

    /// ตั้งค่าภาษาเริ่มต้นจาก VNState
    pub fn initialize_from_vn_state(&mut self, vn_state: &VNState) -> bool {
        if self.initialized {
            return false;
        }

        let language_code = match vn_state.language.as_str() {
            "thai" => LanguageCode::Thai,
            "english" => LanguageCode::English,
            "japanese" => LanguageCode::Japanese,
            _ => LanguageCode::Thai, // fallback
        };

        if self.change_language(language_code) {
            self.initialized = true;
            info!("เริ่มต้น Language System ด้วยภาษา: {:?}", self.current_language);
            true
        } else {
            false
        }
    }
}

/// Event สำหรับการเปลี่ยนภาษา
#[derive(Event)]
pub struct LanguageChangeEvent {
    pub new_language: LanguageCode,
}

/// โหลด language packs ทั้งหมด
pub fn load_language_packs(
    asset_server: Res<AssetServer>,
    mut language_resource: ResMut<LanguageResource>,
) {
    // โหลดภาษาทั้งหมด
    let thai_pack = language("th-th.json").load::<LanguagePack>(&asset_server);
    let english_pack = language("en-us.json").load::<LanguagePack>(&asset_server);
    let japanese_pack = language("jp-jp.json").load::<LanguagePack>(&asset_server);

    language_resource.packs.insert(LanguageCode::Thai, thai_pack.clone());
    language_resource.packs.insert(LanguageCode::English, english_pack);
    language_resource.packs.insert(LanguageCode::Japanese, japanese_pack);

    // ตั้งค่า default
    language_resource.current_pack = Some(thai_pack);
    language_resource.loaded = true;

    info!("โหลด Language Packs เสร็จแล้ว");
}

/// ตรวจสอบว่า language packs โหลดเสร็จแล้ว และ initialize
pub fn check_language_loading(
    asset_server: Res<AssetServer>,
    mut language_resource: ResMut<LanguageResource>,
    vn_state: Res<VNState>,
    mut language_events: EventWriter<LanguageChangeEvent>,
    mut loading_complete: Local<bool>,
) {
    if *loading_complete || !language_resource.loaded {
        return;
    }

    let mut all_loaded = true;
    for (_lang, handle) in &language_resource.packs {
        match asset_server.load_state(handle.id()) {
            bevy::asset::LoadState::Loaded => {}
            bevy::asset::LoadState::Failed(e) => {
                warn!("ไม่สามารถโหลด language pack ได้: {}", e);
            }
            _ => {
                all_loaded = false;
                break;
            }
        }
    }

    if all_loaded {
        *loading_complete = true;

        // Initialize language จาก VNState
        if language_resource.initialize_from_vn_state(&vn_state) {
            language_events.send(LanguageChangeEvent {
                new_language: language_resource.current_language.clone(),
            });
        }

        info!("Language System พร้อมใช้งาน");
    }
}

/// จัดการการเปลี่ยนภาษาด้วย L key
pub fn handle_language_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut language_resource: ResMut<LanguageResource>,
    mut language_events: EventWriter<LanguageChangeEvent>,
) {
    if keyboard.just_pressed(KeyCode::KeyL) {
        let new_language = language_resource.next_language();
        if language_resource.change_language(new_language.clone()) {
            info!("เปลี่ยนภาษาเป็น: {:?}", new_language);
            language_events.send(LanguageChangeEvent { new_language });
        }
    }
}

/// Helper function - ดึงข้อความจาก current language pack
pub fn get_text(
    language_resource: &LanguageResource,
    language_packs: &Assets<LanguagePack>,
    path: &str,
) -> String {
    if let Some(pack_handle) = &language_resource.current_pack {
        if let Some(pack) = language_packs.get(pack_handle) {
            return get_text_from_pack(pack, path);
        }
    }
    format!("[{}]", path) // fallback
}

/// ดึงข้อความจาก language pack ตาม path
fn get_text_from_pack(pack: &LanguagePack, path: &str) -> String {
    let parts: Vec<&str> = path.split('.').collect();

    match parts.as_slice() {
        ["ui", "start_game"] => pack.ui.start_game.clone(),
        ["ui", "settings"] => pack.ui.settings.clone(),
        ["ui", "exit_game"] => pack.ui.exit_game.clone(),
        ["ui", "loading"] => pack.ui.loading.clone(),
        ["ui", "loading_subtitle"] => pack.ui.loading_subtitle.clone(),
        ["ui", "choose_action"] => pack.ui.choose_action.clone(),
        ["ui", "game_title"] => pack.ui.game_title.clone(),
        ["ui", "game_subtitle"] => pack.ui.game_subtitle.clone(),
        ["ui", "controls_help"] => pack.ui.controls_help.clone(),

        ["dialog", "choose_action"] => pack.dialog.choose_action.clone(),
        ["dialog", "continue_hint"] => pack.dialog.continue_hint.clone(),
        ["dialog", "language_indicator"] => pack.dialog.language_indicator.clone(),

        ["game", "narrator"] => pack.game.narrator.clone(),
        ["game", "you"] => pack.game.you.clone(),
        ["game", "unknown"] => pack.game.unknown.clone(),

        _ => format!("[Missing: {}]", path),
    }
}