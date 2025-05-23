use bevy::prelude::*;
use std::collections::HashMap;
use crate::types::DialogScene;

#[derive(Resource, Default)]
pub struct DialogHistory {
    history: Vec<(usize, usize, usize)>,
    previous_stages: Vec<usize>,
}

impl DialogHistory {
    pub fn add_choice(&mut self, from_stage: usize, choice_index: usize, to_stage: usize) {
        self.history.push((from_stage, choice_index, to_stage));
        self.previous_stages.push(from_stage);
    }

    pub fn add_stage(&mut self, stage: usize) {
        self.previous_stages.push(stage);
    }

    pub fn go_back(&mut self) -> Option<usize> {
        if self.previous_stages.is_empty() {
            return None;
        }

        if !self.history.is_empty() &&
            self.history.last().unwrap().2 == *self.previous_stages.last().unwrap() {
            self.history.pop();
        }

        self.previous_stages.pop()
    }

    pub fn clear(&mut self) {
        self.history.clear();
        self.previous_stages.clear();
    }
}

#[derive(Event)]
pub struct StageChangeEvent {
    pub new_stage: usize,
    pub scene_name: Option<String>,
}

#[derive(Event)]
pub struct DialogResetEvent;

/// VNState ที่ใช้ language system ใหม่
#[derive(Resource)]
pub struct VNState {
    pub stage: usize,
    pub language: String, // ยังคงใช้ string เพื่อ backward compatibility
    pub current_scene: String,
    pub current_scene_handle: Option<Handle<DialogScene>>,
    pub stage_changed: bool,
    pub dialog_needs_reset: bool,
}

impl Default for VNState {
    fn default() -> Self {
        Self {
            stage: 0,
            language: "thai".to_string(), // default ไทย
            current_scene: "intro".to_string(),
            current_scene_handle: None,
            stage_changed: false,
            dialog_needs_reset: true,
        }
    }
}

impl VNState {
    pub fn change_stage(&mut self, new_stage: usize) {
        if self.stage != new_stage {
            self.stage = new_stage;
            self.stage_changed = true;
            self.dialog_needs_reset = true;
        }
    }

    /// เปลี่ยนภาษาและแจ้ง dialog ว่าต้องรีเซ็ต
    pub fn change_language(&mut self, new_language: String) {
        if self.language != new_language {
            self.language = new_language;
            self.dialog_needs_reset = true;
        }
    }

    /// สำหรับ sync กับ language system
    pub fn sync_with_language_system(&mut self, language_code: &crate::core::language::types::LanguageCode) {
        let lang_str = match language_code {
            crate::core::language::types::LanguageCode::Thai => "thai",
            crate::core::language::types::LanguageCode::English => "english",
            crate::core::language::types::LanguageCode::Japanese => "japanese",
        };
        self.change_language(lang_str.to_string());
    }

    pub fn mark_dialog_reset(&mut self) {
        self.stage_changed = false;
        self.dialog_needs_reset = false;
    }

    pub fn should_reset_dialog(&self) -> bool {
        self.dialog_needs_reset
    }
}

#[derive(Resource, Default)]
pub struct DialogResource {
    pub scenes: HashMap<String, Handle<DialogScene>>,
    pub current_scene: Option<Handle<DialogScene>>,
}

impl DialogResource {
    pub fn change_scene(&mut self, scene_name: &str, vn_state: &mut VNState) -> bool {
        if let Some(handle) = self.scenes.get(scene_name) {
            self.current_scene = Some(handle.clone());
            vn_state.current_scene = scene_name.to_string();
            vn_state.current_scene_handle = Some(handle.clone());
            vn_state.change_stage(0);
            true
        } else {
            false
        }
    }
}

#[derive(Resource, Default)]
pub struct DialogManager {
    pub current_character_name: String,
    pub current_dialog_text: String,
    pub is_processing_stage_change: bool,
}

impl DialogManager {
    pub fn reset(&mut self) {
        self.current_character_name.clear();
        self.current_dialog_text.clear();
        self.is_processing_stage_change = true;
    }

    pub fn set_content(&mut self, character_name: String, dialog_text: String) {
        self.current_character_name = character_name;
        self.current_dialog_text = dialog_text;
        self.is_processing_stage_change = false;
    }

    pub fn is_processing(&self) -> bool {
        self.is_processing_stage_change
    }
}