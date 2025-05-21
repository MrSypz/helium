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

/// Resource สำหรับสถานะของ Visual Novel
#[derive(Resource)]
pub struct VNState {
    pub stage: usize,
    pub language: String,
    pub current_scene: String,
    pub current_scene_handle: Option<Handle<DialogScene>>,
}

impl Default for VNState {
    fn default() -> Self {
        Self {
            stage: 0,
            language: "thai".to_string(),
            current_scene: "intro".to_string(),
            current_scene_handle: None,
        }
    }
}

/// Resource สำหรับเก็บและจัดการ Dialog Scenes
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
            vn_state.stage = 0;
            true
        } else {
            false
        }
    }
}