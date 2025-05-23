use bevy::prelude::*;
use crate::common::helium::VNState;
use crate::common::dialog::types::DialogScene;
use crate::common::dialog::choice::{ChoiceState, ChoiceButton};
use crate::common::dialog::typewriter::TypewriterText;
use crate::client::render::ui::dialog::DialogText;

/// Component สำหรับ Choice Container
#[derive(Component)]
pub struct ChoiceContainer;

/// Component สำหรับ Choice Text
#[derive(Component)]
pub struct ChoiceText;

/// Component สำหรับ Choice Overlay
#[derive(Component)]
pub struct ChoiceOverlay;

// Constants for UI - เหมือนเดิม
const CHOICE_Z_LAYER: f32 = 15.0;
const CHOICE_TITLE_COLOR: Color = Color::srgb(1.0, 0.85, 0.3);
const CHOICE_TEXT_COLOR: Color = Color::WHITE;
const CHOICE_PANEL_BG: Color = Color::srgba(0.12, 0.12, 0.18, 0.95);
const CHOICE_BORDER_COLOR: Color = Color::srgba(0.5, 0.5, 0.7, 0.5);
const CHOICE_BUTTON_BG: Color = Color::srgba(0.2, 0.2, 0.25, 0.9);
const CHOICE_BUTTON_BORDER: Color = Color::srgba(0.6, 0.6, 0.7, 0.5);
const CHOICE_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.95);
const CHOICE_BUTTON_ACTIVE: Color = Color::srgba(0.4, 0.4, 0.5, 1.0);

/// แสดงตัวเลือกเมื่อถึงจุดที่มีตัวเลือก - ปรับปรุงแล้ว
pub fn manage_choice_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut choice_state: ResMut<ChoiceState>,
    existing_containers: Query<Entity, With<ChoiceContainer>>,
    existing_overlays: Query<Entity, With<ChoiceOverlay>>,
    typewriter_query: Query<&TypewriterText, With<DialogText>>,
) {
    // ตรวจสอบว่าควรแสดงตัวเลือกหรือไม่
    let should_show_choices = should_display_choices(&state, &dialog_scenes, &typewriter_query);

    if should_show_choices && !choice_state.active {
        // ได้เวลาแสดงตัวเลือก
        if let Some(choices) = get_current_choices(&state, &dialog_scenes) {
            // ลบตัวเลือกเก่าถ้ามี
            cleanup_existing_choices(&mut commands, &existing_containers, &existing_overlays);

            // เปิดใช้งานตัวเลือก
            choice_state.activate(choices.clone());

            // สร้าง UI ใหม่
            create_choice_ui(&mut commands, &asset_server, &state, &choices);
        }
    } else if !should_show_choices && choice_state.active {
        // ไม่ควรแสดงตัวเลือกแล้ว แต่ยังแสดงอยู่ -> ปิด
        choice_state.deactivate();
        cleanup_existing_choices(&mut commands, &existing_containers, &existing_overlays);
    }
}

/// ตรวจสอบว่าควรแสดงตัวเลือกหรือไม่
fn should_display_choices(
    state: &VNState,
    dialog_scenes: &Assets<DialogScene>,
    typewriter_query: &Query<&TypewriterText, With<DialogText>>,
) -> bool {
    // ตรวจสอบว่า stage ปัจจุบันมีตัวเลือกหรือไม่
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];

                // ถ้ามีตัวเลือก และข้อความแสดงเสร็จแล้ว
                if !entry.choices.is_empty() {
                    // ตรวจสอบว่าข้อความปัจจุบันพิมพ์จบแล้วหรือไม่
                    if let Ok(typewriter) = typewriter_query.get_single() {
                        return typewriter.char_index >= typewriter.full_text.chars().count();
                    }
                    return true; // ถ้าไม่มี typewriter ให้ถือว่าเสร็จแล้ว
                }
            }
        }
    }
    false
}

/// ดึงตัวเลือกสำหรับ stage ปัจจุบัน
fn get_current_choices(
    state: &VNState,
    dialog_scenes: &Assets<DialogScene>,
) -> Option<Vec<crate::common::dialog::types::DialogChoice>> {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];
                if !entry.choices.is_empty() {
                    return Some(entry.choices.clone());
                }
            }
        }
    }
    None
}

/// ลบตัวเลือกที่มีอยู่
fn cleanup_existing_choices(
    commands: &mut Commands,
    containers: &Query<Entity, With<ChoiceContainer>>,
    overlays: &Query<Entity, With<ChoiceOverlay>>,
) {
    for entity in containers.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in overlays.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// สร้าง Choice UI
fn create_choice_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    state: &VNState,
    choices: &[crate::common::dialog::types::DialogChoice],
) {
    info!("สร้าง UI ตัวเลือก {} ทางเลือก", choices.len());

    // สร้าง overlay ทั้งหน้าจอ
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.6).into(),
            z_index: ZIndex::Global((CHOICE_Z_LAYER - 1.0) as i32),
            ..default()
        },
        Name::new("choice_overlay"),
        ChoiceOverlay,
    ));

    // สร้าง panel หลักสำหรับตัวเลือก
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Percent(15.0),
                top: Val::Percent(20.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(25.0)),
                ..default()
            },
            background_color: CHOICE_PANEL_BG.into(),
            border_color: CHOICE_BORDER_COLOR.into(),
            border_radius: BorderRadius::all(Val::Px(20.0)),
            z_index: ZIndex::Global(CHOICE_Z_LAYER as i32),
            ..default()
        },
        Name::new("choice_container"),
        ChoiceContainer,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            TextBundle::from_section(
                if state.language == "thai" { "เลือกการกระทำของคุณ" } else { "Choose your action" },
                TextStyle {
                    font: asset_server.load("fonts/NotoSansThai-Bold.ttf"),
                    font_size: 32.0,
                    color: CHOICE_TITLE_COLOR,
                },
            )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            Name::new("choice_title"),
        ));

        // Divider
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                margin: UiRect::vertical(Val::Px(5.0)),
                ..default()
            },
            background_color: Color::srgba(0.6, 0.6, 0.7, 0.3).into(),
            ..default()
        });

        // สร้างปุ่มสำหรับแต่ละตัวเลือก
        for (i, choice) in choices.iter().enumerate() {
            create_choice_button(parent, asset_server, state, i, choice);
        }

        // Bottom divider
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(1.0),
                margin: UiRect::vertical(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgba(0.6, 0.6, 0.7, 0.3).into(),
            ..default()
        });
    });
}

/// สร้างปุ่มตัวเลือก
fn create_choice_button(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    state: &VNState,
    index: usize,
    choice: &crate::common::dialog::types::DialogChoice,
) {
    let choice_text = choice.text.get(&state.language)
        .cloned()
        .unwrap_or_else(|| format!("[No choice text in {}]", state.language));

    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                min_height: Val::Px(60.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.5)),
                margin: UiRect::all(Val::Px(6.0)),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            background_color: CHOICE_BUTTON_BG.into(),
            border_color: CHOICE_BUTTON_BORDER.into(),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        },
        ChoiceButton {
            choice_index: index,
            target_stage: choice.target_stage,
        },
        Name::new(format!("choice_button_{}", index)),
    )).with_children(|button| {
        // Number indicator
        button.spawn(NodeBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                margin: UiRect::right(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgba(0.3, 0.3, 0.5, 0.8).into(),
            ..default()
        }).with_children(|number_circle| {
            number_circle.spawn(TextBundle::from_section(
                format!("{}", index + 1),
                TextStyle {
                    font: asset_server.load("fonts/NotoSansThai-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

        // Choice text
        button.spawn((
            TextBundle::from_section(
                choice_text,
                TextStyle {
                    font: asset_server.load("fonts/NotoSansThai-Regular.ttf"),
                    font_size: 26.0,
                    color: CHOICE_TEXT_COLOR,
                },
            )
                .with_style(Style {
                    margin: UiRect::left(Val::Px(5.0)),
                    max_width: Val::Percent(90.0),
                    ..default()
                }),
            ChoiceText,
            Name::new(format!("choice_text_{}", index)),
        ));
    });
}

/// ตรวจสอบการชี้ (hover) ที่ปุ่มตัวเลือก
pub fn highlight_choice_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ChoiceButton>)>,
) {
    for (interaction, mut bg_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = CHOICE_BUTTON_ACTIVE.into();
            }
            Interaction::Hovered => {
                *bg_color = CHOICE_BUTTON_HOVER.into();
            }
            Interaction::None => {
                *bg_color = CHOICE_BUTTON_BG.into();
            }
        }
    }
}