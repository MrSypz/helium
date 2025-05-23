use bevy::prelude::*;
use std::collections::HashMap;
use crate::common::dialog::types::DialogScene;

/// เก็บประวัติการเลือกเพื่อย้อนกลับได้
#[derive(Resource, Default)]
pub struct DialogHistory {
    // เก็บประวัติเป็น (stage ที่เลือก, ตัวเลือกที่เลือก, stage ที่ไป)
    history: Vec<(usize, usize, usize)>,
    // เก็บ stage ก่อนหน้า
    previous_stages: Vec<usize>,
}

impl DialogHistory {
    /// เพิ่มประวัติการเลือก
    pub fn add_choice(&mut self, from_stage: usize, choice_index: usize, to_stage: usize) {
        self.history.push((from_stage, choice_index, to_stage));
        self.previous_stages.push(from_stage);
    }

    /// เพิ่ม stage ปกติ (ไม่มีตัวเลือก)
    pub fn add_stage(&mut self, stage: usize) {
        self.previous_stages.push(stage);
    }

    /// ย้อนกลับไป stage ก่อนหน้า
    pub fn go_back(&mut self) -> Option<usize> {
        if self.previous_stages.is_empty() {
            return None;
        }

        // ลบประวัติล่าสุด
        if !self.history.is_empty() && self.history.last().unwrap().2 == *self.previous_stages.last().unwrap() {
            self.history.pop();
        }

        self.previous_stages.pop()
    }

    /// ล้างประวัติทั้งหมด
    pub fn clear(&mut self) {
        self.history.clear();
        self.previous_stages.clear();
    }
}

/// Event สำหรับการเปลี่ยน stage
#[derive(Event)]
pub struct StageChangeEvent {
    pub new_stage: usize,
    pub scene_name: Option<String>,
}

/// Event สำหรับการรีเซ็ต dialog
#[derive(Event)]
pub struct DialogResetEvent;

/// Resource สำหรับสถานะของ Visual Novel - ปรับปรุงแล้ว
#[derive(Resource)]
pub struct VNState {
    pub stage: usize,
    pub language: String,
    pub current_scene: String,
    pub current_scene_handle: Option<Handle<DialogScene>>,
    /// เพิ่ม flag เพื่อบอกว่า stage เปลี่ยนแปลงแล้ว
    pub stage_changed: bool,
    /// เพิ่ม flag เพื่อบอกว่าต้องรีเซ็ต dialog
    pub dialog_needs_reset: bool,
}

impl Default for VNState {
    fn default() -> Self {
        Self {
            stage: 0,
            language: "thai".to_string(),
            current_scene: "intro".to_string(),
            current_scene_handle: None,
            stage_changed: false,
            dialog_needs_reset: true, // เริ่มต้นต้องรีเซ็ต
        }
    }
}

impl VNState {
    /// เปลี่ยน stage และทำเครื่องหมายว่าต้องรีเซ็ต dialog
    pub fn change_stage(&mut self, new_stage: usize) {
        if self.stage != new_stage {
            self.stage = new_stage;
            self.stage_changed = true;
            self.dialog_needs_reset = true;
            info!("เปลี่ยน stage ไปที่: {}", new_stage);
        }
    }

    /// เปลี่ยนภาษาและทำเครื่องหมายว่าต้องรีเซ็ต dialog
    pub fn change_language(&mut self, new_language: String) {
        if self.language != new_language {
            self.language = new_language;
            self.dialog_needs_reset = true;
            info!("เปลี่ยนภาษาเป็น: {}", self.language);
        }
    }

    /// ทำเครื่องหมายว่า dialog ได้รีเซ็ตแล้ว
    pub fn mark_dialog_reset(&mut self) {
        self.stage_changed = false;
        self.dialog_needs_reset = false;
    }

    /// ตรวจสอบว่าต้องรีเซ็ต dialog หรือไม่
    pub fn should_reset_dialog(&self) -> bool {
        self.dialog_needs_reset
    }
}

/// Resource สำหรับเก็บและจัดการ Dialog Scenes - ปรับปรุงแล้ว
#[derive(Resource, Default)]
pub struct DialogResource {
    pub scenes: HashMap<String, Handle<DialogScene>>,
    pub current_scene: Option<Handle<DialogScene>>,
}

impl DialogResource {
    /// เปลี่ยน scene ปัจจุบัน
    pub fn change_scene(&mut self, scene_name: &str, vn_state: &mut VNState) -> bool {
        if let Some(handle) = self.scenes.get(scene_name) {
            self.current_scene = Some(handle.clone());
            vn_state.current_scene = scene_name.to_string();
            vn_state.current_scene_handle = Some(handle.clone());
            vn_state.change_stage(0); // รีเซ็ต stage เป็น 0 และทำเครื่องหมายให้รีเซ็ต dialog
            true
        } else {
            false
        }
    }
}

/// Central Dialog Manager สำหรับจัดการ dialog state - ใหม่
#[derive(Resource, Default)]
pub struct DialogManager {
    pub current_character_name: String,
    pub current_dialog_text: String,
    pub is_processing_stage_change: bool,
}

impl DialogManager {
    /// รีเซ็ต dialog text และชื่อตัวละคร
    pub fn reset(&mut self) {
        self.current_character_name.clear();
        self.current_dialog_text.clear();
        self.is_processing_stage_change = true;
    }

    /// ตั้งค่า dialog content ใหม่
    pub fn set_content(&mut self, character_name: String, dialog_text: String) {
        self.current_character_name = character_name;
        self.current_dialog_text = dialog_text;
        self.is_processing_stage_change = false;
    }

    /// ตรวจสอบว่ากำลังประมวลผลการเปลี่ยน stage หรือไม่
    pub fn is_processing(&self) -> bool {
        self.is_processing_stage_change
    }
}