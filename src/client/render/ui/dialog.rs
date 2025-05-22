use crate::common::dialog::choice::ChoiceState;
use crate::common::dialog::types::DialogScene;
use crate::common::dialog::typewriter::TypewriterText;
use crate::common::helium::{DialogHistory, DialogResource, VNState};
use crate::util::input_handler;
use bevy::prelude::*;

// UI component tags - make them public
#[derive(Component)]
pub struct DialogBox;

#[derive(Component)]
pub struct CharacterName;

#[derive(Component)]
pub struct DialogText;

#[derive(Component)]
pub struct DialogControls;

#[derive(Component)]
pub struct LanguageIndicator;

// Constants for UI
const DIALOG_Z_LAYER: f32 = 10.0;
const TEXT_COLOR: Color = Color::WHITE;
const NAME_COLOR: Color = Color::srgb(1.0, 0.8, 0.2);
const DIALOG_BG_COLOR: Color = Color::srgba(0.05, 0.05, 0.1, 0.85);
const DIALOG_BORDER_COLOR: Color = Color::srgba(0.3, 0.3, 0.5, 0.5);
/// ระบบสำหรับ setup UI แบบ modern
#[derive(Component)]
pub struct DialogVisibility {
    pub should_show: bool,
    pub changed: bool,
}

impl Default for DialogVisibility {
    fn default() -> Self {
        DialogVisibility {
            should_show: false,
            changed: false,
        }
    }
}

/// ระบบสำหรับ setup UI แบบ modern
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, _state: Res<VNState>) {
    // โหลด font
    let regular_font = asset_server.load("fonts/NotoSansThai-Regular.ttf");
    let bold_font = asset_server.load("fonts/NotoSansThai-Bold.ttf");

    // กล่องข้อความหลัก - ใช้ flexbox layout สำหรับความ responsive
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    min_height: Val::Percent(30.0),
                    max_height: Val::Percent(40.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                background_color: DIALOG_BG_COLOR.into(),
                border_color: DIALOG_BORDER_COLOR.into(),
                border_radius: BorderRadius::all(Val::Px(15.0)),
                z_index: ZIndex::Global(DIALOG_Z_LAYER as i32),
                ..default()
            },
            DialogBox,
            Name::new("dialog_box"),
            DialogVisibility::default(),
        ))
        .with_children(|parent| {
            // ชื่อตัวละคร - อยู่ในกล่องเล็กๆ ด้านบน
            parent
                .spawn((NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        padding: UiRect::new(
                            Val::Px(15.0),
                            Val::Px(15.0),
                            Val::Px(5.0),
                            Val::Px(5.0),
                        ),
                        ..default()
                    },
                    background_color: Color::srgba(0.1, 0.1, 0.2, 0.9).into(),
                    border_color: NAME_COLOR.with_alpha(0.7).into(),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },))
                .with_children(|name_box| {
                    name_box.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: bold_font.clone(),
                                font_size: 32.0,
                                color: NAME_COLOR,
                            },
                        ),
                        CharacterName,
                        Name::new("character_name"),
                    ));
                });

            // ข้อความบทสนทนา
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: regular_font.clone(),
                        font_size: 30.0,
                        color: TEXT_COLOR,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
                DialogText,
                Name::new("dialogue"),
                TypewriterText::new("", 0.05),
            ));

            // ตัวควบคุมด้านล่าง - แสดงแถบสถานะและปุ่ม
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                DialogControls,
            ));
        });
}

pub fn update_dialog(
    state: Res<VNState>,
    dialog_resource: Res<DialogResource>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut query_set: ParamSet<(
        Query<&mut Text, (With<CharacterName>, Without<TypewriterText>)>,
        Query<(&mut Text, &mut TypewriterText), With<DialogText>>,
        Query<&mut Text, With<LanguageIndicator>>,
    )>,
) {
    // อัพเดทตัวแสดงภาษา
    {
        let mut language_query = query_set.p2();
        if let Ok(mut lang_text) = language_query.get_single_mut() {
            lang_text.sections[0].value = if state.language == "thai" {
                "TH".parse().unwrap()
            } else {
                "EN".parse().unwrap()
            };
        }
    }

    // ตรวจสอบว่ามี dialog scene ปัจจุบันหรือไม่
    if let Some(scene_handle) = &dialog_resource.current_scene {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            // ตรวจสอบว่า stage ถูกต้องหรือไม่
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];

                // หาชื่อตัวละครที่จะแสดง
                let character_display_name = scene
                    .characters
                    .iter()
                    .find(|c| c.name == entry.character)
                    .and_then(|c| c.display_name.get(&state.language))
                    .cloned()
                    .unwrap_or_else(|| entry.character.clone());

                // หาข้อความที่จะแสดง
                let dialog_text = entry
                    .text
                    .get(&state.language)
                    .cloned()
                    .unwrap_or_else(|| format!("[No text in {}]", state.language));

                // อัพเดทชื่อตัวละคร
                {
                    let mut character_query = query_set.p0();
                    for mut text in character_query.iter_mut() {
                        if text.sections[0].value.is_empty() {
                            text.sections[0].value = character_display_name.clone();
                        }
                    }
                }

                // อัพเดทข้อความ dialog ถ้ายังไม่ได้เริ่มพิมพ์
                {
                    let mut dialog_query = query_set.p1();
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
}

pub fn text_click(
    mut state: ResMut<VNState>,
    mut history: ResMut<DialogHistory>,
    mut dialog_resource: ResMut<DialogResource>,
    dialog_scenes: Res<Assets<DialogScene>>,
    choice_state: Res<ChoiceState>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
    mut query_set: ParamSet<(
        Query<&mut Text, (With<CharacterName>, Without<TypewriterText>)>,
        Query<(&mut Text, &mut TypewriterText), With<DialogText>>,
    )>,
    dialog_box_query: Query<&Interaction, (With<DialogBox>, Changed<Interaction>)>,
) {
    // ถ้ากำลังแสดงตัวเลือกอยู่ ไม่ให้คลิกเปลี่ยน dialog
    if choice_state.active {
        return;
    }

    // ตรวจสอบการคลิกหรือแตะ
    if let Some(_) = input_handler::detect_interaction(&mouse, &touch, &dialog_box_query) {
        if let Some(scene_handle) = &dialog_resource.current_scene {
            if let Some(scene) = dialog_scenes.get(scene_handle) {
                // ตรวจสอบว่าข้อความปัจจุบันพิมพ์จบแล้วหรือไม่
                let is_finished = {
                    let dialog_query = query_set.p1();
                    if let Ok((_, typewriter)) = dialog_query.get_single() {
                        input_handler::is_dialog_text_finished(typewriter)
                    } else {
                        false
                    }
                };

                if is_finished {
                    // เช็คว่ามี actions หรือไม่ (เช่น เปลี่ยน scene)
                    if state.stage < scene.entries.len() {
                        let entry = &scene.entries[state.stage];

                        // ประมวลผล actions ถ้ามี
                        if !entry.actions.is_empty() {
                            for action in &entry.actions {
                                // ตรวจสอบ action ประเภทเปลี่ยน scene
                                if let Some(scene_name) = action.strip_prefix("change_scene:") {
                                    info!("เปลี่ยน scene ไปที่: {}", scene_name);
                                    if dialog_resource.change_scene(scene_name, &mut state) {
                                        // รีเซ็ตข้อความเมื่อเปลี่ยน scene
                                        reset_dialog_text(&mut query_set);
                                        return;
                                    } else {
                                        warn!("ไม่พบ scene ชื่อ: {}", scene_name);
                                    }
                                }
                            }
                        }
                    }

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
                    reset_dialog_text(&mut query_set);
                } else {
                    // ถ้ายังพิมพ์ไม่จบ ให้แสดงข้อความทั้งหมดทันที
                    let mut dialog_query = query_set.p1();
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

// Helper function เพื่อรีเซ็ตข้อความเมื่อเปลี่ยน stage หรือ scene
fn reset_dialog_text(
    query_set: &mut ParamSet<(
        Query<&mut Text, (With<CharacterName>, Without<TypewriterText>)>,
        Query<(&mut Text, &mut TypewriterText), With<DialogText>>,
    )>,
) {
    {
        let mut character_query = query_set.p0();
        for mut text in &mut character_query.iter_mut() {
            text.sections[0].value = "".to_string();
        }
    }

    {
        let mut dialog_query = query_set.p1();
        if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
            *typewriter = TypewriterText::new("", 0.05);
            text.sections[0].value = "".to_string();
        }
    }
}

pub fn toggle_language(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<VNState>,
    mut query_set: ParamSet<(
        Query<&mut Text, (With<CharacterName>, Without<TypewriterText>)>,
        Query<(&mut Text, &mut TypewriterText), With<DialogText>>,
        Query<&mut Text, With<LanguageIndicator>>,
    )>,
) {
    if let Some(_) = input_handler::detect_key_press(&keyboard, &[KeyCode::KeyL]) {
        let language_changed = input_handler::switch_language(&mut state);

        if language_changed {
            let mut language_query = query_set.p2();
            if let Ok(mut lang_text) = language_query.get_single_mut() {
                lang_text.sections[0].value = if state.language == "thai" {
                    "TH".parse().unwrap()
                } else {
                    "EN".parse().unwrap()
                };
            }
        }

        {
            let mut character_query = query_set.p0();
            for mut text in &mut character_query.iter_mut() {
                text.sections[0].value = "".to_string();
            }
        }

        {
            let mut dialog_query = query_set.p1();
            if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
                *typewriter = TypewriterText::new("", 0.05);
                text.sections[0].value = "".to_string();
            }
        }
    }
}
