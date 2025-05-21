use bevy::prelude::*;
use crate::common::helium::{VNState, DialogHistory};
use crate::common::dialog::types::DialogChoice;
use crate::client::render::ui::choice::ChoiceContainer;

/// Component สำหรับปุ่มตัวเลือก
#[derive(Component)]
pub struct ChoiceButton {
    pub choice_index: usize,
    pub target_stage: usize,
}

/// สถานะการแสดงตัวเลือก
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
}

/// ระบบจัดการการกดปุ่มตัวเลือก - แบบ modern
pub fn handle_choice_click(
    mut commands: Commands,
    mut state: ResMut<VNState>,
    mut choice_state: ResMut<ChoiceState>,
    mut history: ResMut<DialogHistory>,
    choice_query: Query<(&ChoiceButton, &Interaction), Changed<Interaction>>,
    container_query: Query<Entity, With<ChoiceContainer>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !choice_state.active {
        return;
    }

    // ตรวจจับการกดปุ่มเลข 1-9 เพื่อเลือกตัวเลือกได้เร็วขึ้น
    let number_keys = [
        KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3,
        KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6,
        KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
    ];

    for (i, key) in number_keys.iter().enumerate() {
        if keyboard.just_pressed(*key) && i < choice_state.choices.len() {
            // Get the target stage from the choices directly
            let target_stage = choice_state.choices[i].target_stage;

            // Process the selection
            process_choice_selection(&mut commands, &mut state, &mut choice_state,
                                     &mut history, &container_query, i, target_stage);
            return;
        }
    }

    // ตรวจจับการคลิกปกติ
    for (choice, interaction) in choice_query.iter() {
        if *interaction == Interaction::Pressed {
            process_choice_selection(&mut commands, &mut state, &mut choice_state,
                                     &mut history, &container_query,
                                     choice.choice_index, choice.target_stage);
            return;
        }
    }
}

// Helper function เพื่อจัดการการเลือกตัวเลือก
fn process_choice_selection(
    commands: &mut Commands,
    state: &mut VNState,
    choice_state: &mut ChoiceState,
    history: &mut DialogHistory,
    container_query: &Query<Entity, With<ChoiceContainer>>,
    choice_index: usize,
    target_stage: usize,
) {
    info!("เลือกตัวเลือกที่ {} ไปยัง stage {}", choice_index, target_stage);

    // เก็บประวัติการเลือก
    choice_state.add_choice(choice_index);
    history.add_choice(state.stage, choice_index, target_stage);

    // เปลี่ยน stage ไปตาม target
    state.stage = target_stage;

    // ลบตัวเลือกทั้งหมด
    for entity in container_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    choice_state.active = false;
}

/// ระบบย้อนกลับไปการเลือกก่อนหน้า - แบบ modern
pub fn handle_back_button(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<VNState>,
    mut history: ResMut<DialogHistory>,
    mut choice_state: ResMut<ChoiceState>,
    query: Query<Entity, With<ChoiceContainer>>,
    mut commands: Commands,
) {
    // กดปุ่ม B, Backspace หรือ Escape เพื่อย้อนกลับ
    let back_pressed = keyboard.just_pressed(KeyCode::KeyB) ||
        keyboard.just_pressed(KeyCode::Backspace) ||
        keyboard.just_pressed(KeyCode::Escape);

    if back_pressed {
        if let Some(previous) = history.go_back() {
            info!("ย้อนกลับไปยัง stage: {}", previous);

            // ลบตัวเลือกปัจจุบันถ้ามี
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // กลับไปที่ stage ก่อนหน้า
            state.stage = previous;
            choice_state.active = false;
        }
    }
}

/// Renamed from choice_system to match what's in plugin.rs
pub fn debug_choice_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    choice_state: Res<ChoiceState>,
    choice_query: Query<Entity, With<ChoiceButton>>,
    container_query: Query<Entity, With<ChoiceContainer>>,
) {
    // กด D เพื่อดู debug
    if keyboard.just_pressed(KeyCode::KeyD) {
        info!("สถานะตัวเลือก: {}", if choice_state.active { "active" } else { "inactive" });
        info!("จำนวนปุ่มตัวเลือก: {}", choice_query.iter().count());
        info!("จำนวน container: {}", container_query.iter().count());
        info!("ประวัติการเลือก: {:?}", choice_state.history);
    }
}