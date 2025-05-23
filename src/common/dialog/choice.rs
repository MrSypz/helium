use bevy::prelude::*;
use crate::common::helium::{VNState, DialogHistory};
use crate::common::dialog::types::DialogChoice;
use crate::client::render::ui::choice::{ChoiceContainer, ChoiceOverlay};

/// Component สำหรับปุ่มตัวเลือก
#[derive(Component)]
pub struct ChoiceButton {
    pub choice_index: usize,
    pub target_stage: usize,
}

/// สถานะการแสดงตัวเลือก - ปรับปรุงแล้ว
#[derive(Resource, Default)]
pub struct ChoiceState {
    pub active: bool,
    pub choices: Vec<DialogChoice>,
    pub history: Vec<usize>, // เก็บประวัติการเลือก
}

impl ChoiceState {
    pub fn add_choice(&mut self, choice_index: usize) {
        self.history.push(choice_index);
    }

    pub fn get_last_choice(&self) -> Option<usize> {
        self.history.last().copied()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// เปิดใช้งานตัวเลือกพร้อมกับ choices ใหม่
    pub fn activate(&mut self, choices: Vec<DialogChoice>) {
        self.active = true;
        self.choices = choices;
        info!("เปิดใช้งานตัวเลือก {} ทางเลือก", self.choices.len());
    }

    /// ปิดการใช้งานตัวเลือก
    pub fn deactivate(&mut self) {
        self.active = false;
        self.choices.clear();
        info!("ปิดการใช้งานตัวเลือก");
    }
}

/// ระบบจัดการการกดปุ่มตัวเลือก - ปรับปรุงใหม่
pub fn handle_choice_selection(
    mut commands: Commands,
    mut state: ResMut<VNState>,
    mut choice_state: ResMut<ChoiceState>,
    mut history: ResMut<DialogHistory>,
    choice_query: Query<(&ChoiceButton, &Interaction), Changed<Interaction>>,
    container_query: Query<Entity, With<ChoiceContainer>>,
    overlay_query: Query<Entity, With<ChoiceOverlay>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !choice_state.active {
        return;
    }

    let mut selected_choice: Option<(usize, usize)> = None;

    // ตรวจจับการกดปุ่มเลข 1-9 เพื่อเลือกตัวเลือกได้เร็วขึ้น
    let number_keys = [
        KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3,
        KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6,
        KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
    ];

    for (i, key) in number_keys.iter().enumerate() {
        if keyboard.just_pressed(*key) && i < choice_state.choices.len() {
            let target_stage = choice_state.choices[i].target_stage;
            selected_choice = Some((i, target_stage));
            break;
        }
    }

    // ตรวจจับการคลิกปกติ (ถ้ายังไม่ได้เลือกจากปุ่ม)
    if selected_choice.is_none() {
        for (choice, interaction) in choice_query.iter() {
            if *interaction == Interaction::Pressed {
                selected_choice = Some((choice.choice_index, choice.target_stage));
                break;
            }
        }
    }

    // ประมวลผลการเลือก
    if let Some((choice_index, target_stage)) = selected_choice {
        execute_choice_selection(
            &mut commands,
            &mut state,
            &mut choice_state,
            &mut history,
            &container_query,
            &overlay_query,
            choice_index,
            target_stage,
        );
    }
}

/// ประมวลผลการเลือก choice - แยกออกมาเป็นฟังก์ชันแยก
fn execute_choice_selection(
    commands: &mut Commands,
    state: &mut VNState,
    choice_state: &mut ChoiceState,
    history: &mut DialogHistory,
    container_query: &Query<Entity, With<ChoiceContainer>>,
    overlay_query: &Query<Entity, With<ChoiceOverlay>>,
    choice_index: usize,
    target_stage: usize,
) {
    info!("เลือกตัวเลือกที่ {} ไปยัง stage {}", choice_index, target_stage);

    // เก็บประวัติการเลือก
    choice_state.add_choice(choice_index);
    history.add_choice(state.stage, choice_index, target_stage);

    // ปิดการใช้งานตัวเลือก
    choice_state.deactivate();

    // เปลี่ยน stage - ใช้ centralized method ที่จะทำให้ dialog รีเซ็ตอัตโนมัติ
    state.change_stage(target_stage);

    // ลบ UI elements ของตัวเลือก
    cleanup_choice_ui(commands, container_query, overlay_query);
}

/// ลบ UI elements ของตัวเลือก
fn cleanup_choice_ui(
    commands: &mut Commands,
    container_query: &Query<Entity, With<ChoiceContainer>>,
    overlay_query: &Query<Entity, With<ChoiceOverlay>>,
) {
    // ลบตัวเลือกทั้งหมด
    for entity in container_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // ลบ overlay ด้วย
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// ระบบสำหรับ debug ตัวเลือก - เหมือนเดิม
pub fn debug_choice_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    choice_state: Res<ChoiceState>,
    choice_query: Query<Entity, With<ChoiceButton>>,
    container_query: Query<Entity, With<ChoiceContainer>>,
) {
    // กด D เพื่อดู debug
    if keyboard.just_pressed(KeyCode::KeyD) {
        info!("=== Choice Debug ===");
        info!("สถานะตัวเลือก: {}", if choice_state.active { "active" } else { "inactive" });
        info!("จำนวนปุ่มตัวเลือก: {}", choice_query.iter().count());
        info!("จำนวน container: {}", container_query.iter().count());
        info!("ประวัติการเลือก: {:?}", choice_state.history);
        info!("ตัวเลือกที่มี: {}", choice_state.choices.len());
    }
}