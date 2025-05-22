use bevy::prelude::*;
use crate::common::helium::{VNState};
use crate::common::dialog::types::{DialogScene};
use crate::common::dialog::choice::{ChoiceState, ChoiceButton};
use crate::common::dialog::typewriter::TypewriterText;
use crate::client::render::ui::dialog::DialogText;

/// Component สำหรับ Choice Container
#[derive(Component)]
pub struct ChoiceContainer;

/// Component สำหรับ Choice Text
#[derive(Component)]
pub struct ChoiceText;

/// Component สำหรับ Choice Overlay - เพิ่มเพื่อให้มองเห็นและจัดการได้ง่าย
#[derive(Component)]
pub struct ChoiceOverlay;

/// Constants for UI - ปรับปรุงใหม่
const CHOICE_Z_LAYER: f32 = 15.0;
const CHOICE_TITLE_COLOR: Color = Color::srgb(1.0, 0.85, 0.3);
const CHOICE_TEXT_COLOR: Color = Color::WHITE;
const CHOICE_PANEL_BG: Color = Color::srgba(0.12, 0.12, 0.18, 0.95);
const CHOICE_BORDER_COLOR: Color = Color::srgba(0.5, 0.5, 0.7, 0.5);
const CHOICE_BUTTON_BG: Color = Color::srgba(0.2, 0.2, 0.25, 0.9);
const CHOICE_BUTTON_BORDER: Color = Color::srgba(0.6, 0.6, 0.7, 0.5);
const CHOICE_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.95);
const CHOICE_BUTTON_ACTIVE: Color = Color::srgba(0.4, 0.4, 0.5, 1.0);

/// แสดงตัวเลือกเมื่อถึงจุดที่มีตัวเลือก - ตรวจสอบว่าข้อความแสดงจบแล้ว
pub fn display_choices(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut choice_state: ResMut<ChoiceState>,
    query: Query<Entity, With<ChoiceContainer>>,
    overlay_query: Query<Entity, With<ChoiceOverlay>>,
    typewriter_query: Query<&TypewriterText, With<DialogText>>,
) {
    // ตรวจสอบหากยังไม่มีการแสดงตัวเลือกและควรแสดง
    if !choice_state.active {
        // เช็คว่า stage ปัจจุบันมีตัวเลือกหรือไม่
        if let Some(scene_handle) = &state.current_scene_handle {
            if let Some(scene) = dialog_scenes.get(scene_handle) {
                if state.stage < scene.entries.len() {
                    let entry = &scene.entries[state.stage];

                    // ถ้ามีตัวเลือก ให้แสดง แต่ต้องตรวจสอบว่าข้อความแสดงเสร็จแล้ว
                    if !entry.choices.is_empty() {
                        // ตรวจสอบว่าข้อความปัจจุบันพิมพ์จบแล้วหรือไม่
                        let text_finished = if let Ok(typewriter) = typewriter_query.get_single() {
                            typewriter.char_index >= typewriter.full_text.chars().count()
                        } else {
                            true // ถ้าไม่มี typewriter ให้ถือว่าเสร็จแล้ว
                        };

                        // แสดงตัวเลือกเฉพาะเมื่อข้อความแสดงเสร็จแล้ว
                        if text_finished {
                            // ลบตัวเลือกเก่าถ้ามี
                            for entity in query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }

                            // ลบ overlay เก่าถ้ามี
                            for entity in overlay_query.iter() {
                                commands.entity(entity).despawn_recursive();
                            }

                            choice_state.active = true;
                            choice_state.choices = entry.choices.clone();

                            info!("แสดงตัวเลือก {} ทางเลือก", entry.choices.len());

                            // ดีไซน์ใหม่: สร้าง overlay ทั้งหน้าจอเพื่อทำ dim background
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
                                ChoiceOverlay, // เพิ่ม component เพื่อให้ค้นหาได้ง่าย
                            ));

                            // สร้าง node สำหรับแสดงตัวเลือก - ดีไซน์ใหม่
                            commands.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Percent(70.0),
                                        height: Val::Auto,
                                        position_type: PositionType::Absolute,
                                        left: Val::Percent(15.0), // Center horizontally
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
                                // Choice Title
                                parent.spawn((
                                    TextBundle::from_section(
                                        "เลือกการกระทำของคุณ",
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
                                parent.spawn(
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(1.0),
                                            margin: UiRect::vertical(Val::Px(5.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgba(0.6, 0.6, 0.7, 0.3).into(),
                                        ..default()
                                    }
                                );

                                // สร้างปุ่มสำหรับแต่ละตัวเลือก - ดีไซน์ใหม่
                                for (i, choice) in entry.choices.iter().enumerate() {
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
                                            choice_index: i,
                                            target_stage: choice.target_stage,
                                        },
                                        Name::new(format!("choice_button_{}", i)),
                                    )).with_children(|button| {
                                        // Number indicator
                                        button.spawn(
                                            NodeBundle {
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
                                            }
                                        ).with_children(|number_circle| {
                                            number_circle.spawn(
                                                TextBundle::from_section(
                                                    format!("{}", i + 1),
                                                    TextStyle {
                                                        font: asset_server.load("fonts/NotoSansThai-Bold.ttf"),
                                                        font_size: 20.0,
                                                        color: Color::WHITE,
                                                    },
                                                )
                                            );
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
                                            Name::new(format!("choice_text_{}", i)),
                                        ));
                                    });
                                }

                                // Divider
                                parent.spawn(
                                    NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(1.0),
                                            margin: UiRect::vertical(Val::Px(10.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgba(0.6, 0.6, 0.7, 0.3).into(),
                                        ..default()
                                    }
                                );
                            });
                        }
                    }
                }
            }
        }
    }
}

/// ตรวจสอบการชี้ (hover) ที่ปุ่มตัวเลือก - ปรับปรุงใหม่
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

pub fn cleanup_overlay_on_choice_change(
    choice_state: Res<ChoiceState>,
    overlay_query: Query<Entity, With<ChoiceOverlay>>,
    mut commands: Commands,
) {
    // ถ้า choice_state.active เป็น false แต่ยังมี overlay ให้ลบออก
    if !choice_state.active && !overlay_query.is_empty() {
        for entity in overlay_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}