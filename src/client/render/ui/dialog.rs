use bevy::prelude::*;
use crate::common::dialog::choice::ChoiceState;
use crate::common::helium::{VNState, DialogResource, DialogHistory};
use crate::common::dialog::typewriter::TypewriterText;
use crate::common::dialog::types::DialogScene;

/// ระบบสำหรับ setup UI
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _state: Res<VNState>,
) {
    // กล่องข้อความ - ข้อความเริ่มต้นจะถูกเติมในระบบ update_dialog
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(25.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
            ..default()
        },
        Name::new("dialog_box"),
    ));

    // ชื่อตัวละคร
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/NotoSansThai-Bold.ttf"),
                font_size: 32.0,
                color: Color::srgb(1.0, 1.0, 0.0),
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(100.0),
                left: Val::Px(50.0),
                ..default()
            }),
        Name::new("character_name"),
    ));

    // ข้อความ dialog
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/NotoSansThai-Regular.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(50.0),
                left: Val::Px(50.0),
                ..default()
            }),
        Name::new("dialogue"),
        TypewriterText::new("", 0.05),
    ));
}

/// อัพเดทข้อความ dialog ตาม state ปัจจุบัน
pub fn update_dialog(
    state: Res<VNState>,
    dialog_resource: Res<DialogResource>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut character_query: Query<&mut Text, (With<Name>, Without<TypewriterText>)>,
    mut dialog_query: Query<(&mut Text, &mut TypewriterText), With<Name>>,
) {
    // ตรวจสอบว่ามี dialog scene ปัจจุบันหรือไม่
    if let Some(scene_handle) = &dialog_resource.current_scene {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            // ตรวจสอบว่า stage ถูกต้องหรือไม่
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];

                // หาชื่อตัวละครที่จะแสดง
                let character_display_name = scene.characters.iter()
                    .find(|c| c.name == entry.character)
                    .and_then(|c| c.display_name.get(&state.language))
                    .cloned()
                    .unwrap_or_else(|| entry.character.clone());

                // หาข้อความที่จะแสดง
                let dialog_text = entry.text.get(&state.language)
                    .cloned()
                    .unwrap_or_else(|| format!("[No text in {}]", state.language));

                // อัพเดทชื่อตัวละคร
                for mut text in &mut character_query.iter_mut() {
                    if text.sections[0].value.is_empty() {
                        text.sections[0].value = character_display_name.clone();
                    }
                }

                // อัพเดทข้อความ dialog ถ้ายังไม่ได้เริ่มพิมพ์
                if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
                    if typewriter.full_text.is_empty() {
                        *typewriter = TypewriterText::new(&dialog_text, 0.05);
                        text.sections[0].value = "".to_string();
                    }
                }
            }
        }
    }
}

/// ตรวจจับการคลิกเพื่อเปลี่ยนหรือเร่งข้อความ
pub fn text_click(
    mut state: ResMut<VNState>,
    mut history: ResMut<DialogHistory>,
    dialog_resource: Res<DialogResource>,
    dialog_scenes: Res<Assets<DialogScene>>,
    choice_state: Res<ChoiceState>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut character_query: Query<&mut Text, (With<Name>, Without<TypewriterText>)>,
    mut dialog_query: Query<(&mut Text, &mut TypewriterText), With<Name>>,
) {
    // ถ้ากำลังแสดงตัวเลือกอยู่ ไม่ให้คลิกเปลี่ยน dialog
    if choice_state.active {
        return;
    }

    if mouse.just_pressed(MouseButton::Left) {
        if let Some(scene_handle) = &dialog_resource.current_scene {
            if let Some(scene) = dialog_scenes.get(scene_handle) {
                // ตรวจสอบว่าข้อความปัจจุบันพิมพ์จบแล้วหรือไม่
                if let Ok((_, typewriter)) = dialog_query.get_single() {
                    let is_finished = typewriter.char_index >= typewriter.full_text.chars().count();

                    if is_finished {
                        // เช็คว่ามี auto_proceed หรือไม่
                        let auto_target = if state.stage < scene.entries.len() {
                            scene.entries[state.stage].auto_proceed
                        } else {
                            None
                        };

                        // บันทึกประวัติ stage ปัจจุบัน
                        history.add_stage(state.stage);

                        // ไปยัง stage ตาม auto_proceed หรือถัดไปตามปกติ
                        if let Some(target) = auto_target {
                            state.stage = target;
                        } else {
                            state.stage = (state.stage + 1) % scene.entries.len();
                        }

                        // รีเซ็ตข้อความ
                        for mut text in &mut character_query.iter_mut() {
                            text.sections[0].value = "".to_string();
                        }

                        if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
                            *typewriter = TypewriterText::new("", 0.05);
                            text.sections[0].value = "".to_string();
                        }
                    } else {
                        // ถ้ายังพิมพ์ไม่จบ ให้แสดงข้อความทั้งหมดทันที
                        if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
                            text.sections[0].value = typewriter.full_text.clone();
                            typewriter.current_text = typewriter.full_text.clone();
                            typewriter.char_index = typewriter.full_text.chars().count();
                        }
                    }
                }
            }
        }
    }
}


/// สลับภาษาระหว่าง Thai และ English
pub fn toggle_language(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<VNState>,
    mut character_query: Query<&mut Text, (With<Name>, Without<TypewriterText>)>,
    mut dialog_query: Query<(&mut Text, &mut TypewriterText), With<Name>>,
) {
    // กด L เพื่อสลับภาษา
    if keyboard.just_pressed(KeyCode::KeyL) {
        state.language = if state.language == "thai" {
            "english".to_string()
        } else {
            "thai".to_string()
        };

        // รีเซ็ตข้อความเพื่อให้มีการอัพเดทในภาษาใหม่
        for mut text in &mut character_query.iter_mut() {
            text.sections[0].value = "".to_string();
        }

        if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
            *typewriter = TypewriterText::new("", 0.05);
            text.sections[0].value = "".to_string();
        }
    }
}