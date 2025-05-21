use bevy::prelude::*;
use crate::common::helium::{VNState, DialogHistory};
use crate::common::dialog::types::{DialogScene, DialogChoice};
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
}

/// ระบบจัดการการกดปุ่มตัวเลือก
pub fn handle_choice_click(
    mut commands: Commands,
    mut state: ResMut<VNState>,
    mut choice_state: ResMut<ChoiceState>,
    mut history: ResMut<DialogHistory>,
    choice_query: Query<(Entity, &ChoiceButton, &Interaction)>,
    container_query: Query<Entity, With<ChoiceContainer>>,
) {
    if !choice_state.active {
        return;
    }

    for (entity, choice, interaction) in choice_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("เลือกตัวเลือกที่ {} ไปยัง stage {}", choice.choice_index, choice.target_stage);

            // บันทึกประวัติการเลือก
            history.add_choice(state.stage, choice.choice_index, choice.target_stage);

            // เปลี่ยน stage ไปตาม target
            state.stage = choice.target_stage;

            // ลบตัวเลือกทั้งหมด
            for entity in container_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            choice_state.active = false;
            break;
        }
    }
}

/// ระบบย้อนกลับไปการเลือกก่อนหน้า
pub fn handle_back_button(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<VNState>,
    mut history: ResMut<DialogHistory>,
    mut choice_state: ResMut<ChoiceState>,
    query: Query<Entity, With<ChoiceContainer>>,
    mut commands: Commands,
) {
    // กดปุ่ม B เพื่อย้อนกลับ
    if keyboard.just_pressed(KeyCode::KeyB) {
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

/// ระบบตรวจสอบปุ่มอื่นๆ (ชั่วคราวสำหรับ Debug)
pub fn debug_choice_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    choice_state: ResMut<ChoiceState>,
    choice_query: Query<Entity, With<ChoiceButton>>,
    container_query: Query<Entity, With<ChoiceContainer>>,
) {
    // กด D เพื่อดู debug
    if keyboard.just_pressed(KeyCode::KeyD) {
        info!("สถานะตัวเลือก: {}", if choice_state.active { "active" } else { "inactive" });
        info!("จำนวนปุ่มตัวเลือก: {}", choice_query.iter().count());
        info!("จำนวน container: {}", container_query.iter().count());
    }
}