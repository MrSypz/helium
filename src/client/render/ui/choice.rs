use bevy::prelude::*;
use crate::common::helium::VNState;
use crate::common::dialog::types::{DialogScene, DialogChoice};
use crate::common::dialog::choice::{ChoiceState, ChoiceButton};

/// Component สำหรับ Choice Container
#[derive(Component)]
pub struct ChoiceContainer;

/// แสดงตัวเลือกเมื่อถึงจุดที่มีตัวเลือก
pub fn display_choices(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut choice_state: ResMut<ChoiceState>,
    query: Query<Entity, With<ChoiceContainer>>,
) {
    // ตรวจสอบหากยังไม่มีการแสดงตัวเลือกและควรแสดง
    if !choice_state.active {
        // เช็คว่า stage ปัจจุบันมีตัวเลือกหรือไม่
        if let Some(scene_handle) = &state.current_scene_handle {
            if let Some(scene) = dialog_scenes.get(scene_handle) {
                if state.stage < scene.entries.len() {
                    let entry = &scene.entries[state.stage];

                    // ถ้ามีตัวเลือก ให้แสดง
                    if !entry.choices.is_empty() {
                        // ลบตัวเลือกเก่าถ้ามี
                        for entity in query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }

                        choice_state.active = true;
                        choice_state.choices = entry.choices.clone();

                        info!("แสดงตัวเลือก {} ทางเลือก", entry.choices.len());

                        // สร้าง node สำหรับแสดงตัวเลือก
                        commands.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(80.0),
                                    height: Val::Auto,
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(10.0),
                                    top: Val::Percent(30.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    row_gap: Val::Px(15.0),
                                    padding: UiRect::all(Val::Px(20.0)),
                                    ..default()
                                },
                                background_color: Color::srgba(0.1, 0.1, 0.1, 0.9).into(),
                                ..default()
                            },
                            Name::new("choice_container"),
                            ChoiceContainer,
                        )).with_children(|parent| {
                            // สร้างปุ่มสำหรับแต่ละตัวเลือก
                            for (i, choice) in entry.choices.iter().enumerate() {
                                let choice_text = choice.text.get(&state.language)
                                    .cloned()
                                    .unwrap_or_else(|| format!("[No choice text in {}]", state.language));

                                parent.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(50.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border: UiRect::all(Val::Px(2.0)),
                                            margin: UiRect::all(Val::Px(5.0)),
                                            padding: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgba(0.2, 0.2, 0.3, 1.0).into(),
                                        border_color: Color::srgba(0.5, 0.5, 0.5, 1.0).into(),
                                        ..default()
                                    },
                                    ChoiceButton {
                                        choice_index: i,
                                        target_stage: choice.target_stage,
                                    },
                                    Name::new(format!("choice_button_{}", i)),
                                )).with_children(|button| {
                                    button.spawn(
                                        TextBundle::from_section(
                                            choice_text,
                                            TextStyle {
                                                font: asset_server.load("fonts/NotoSansThai-Regular.ttf"),
                                                font_size: 24.0,
                                                color: Color::srgb(1.0, 1.0, 1.0),
                                            },
                                        )
                                    );
                                });
                            }
                        });
                    }
                }
            }
        }
    }
}

/// ตรวจสอบการชี้ (hover) ที่ปุ่มตัวเลือก
pub fn highlight_choice_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ChoiceButton>)>,
) {
    for (interaction, mut bg_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgba(0.3, 0.3, 0.5, 1.0).into();
            }
            Interaction::Hovered => {
                *bg_color = Color::srgba(0.3, 0.3, 0.4, 1.0).into();
            }
            Interaction::None => {
                *bg_color = Color::srgba(0.2, 0.2, 0.3, 1.0).into();
            }
        }
    }
}