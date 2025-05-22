// src/client/render/transition.rs
use bevy::prelude::*;
use crate::common::dialog::types::{DialogScene};
use crate::common::helium::VNState;

/// คอมโพเนนต์สำหรับการเปลี่ยนฉาก
#[derive(Component)]
pub struct TransitionOverlay {
    pub effect_type: String,
    pub timer: Timer,
}

/// เอนทิตี้เปลี่ยนฉาก
#[derive(Resource)]
pub struct ActiveTransition {
    pub entity: Option<Entity>,
    pub active: bool,
    pub current_stage: usize, // เพิ่มการติดตาม stage ที่เริ่ม transition
}

impl Default for ActiveTransition {
    fn default() -> Self {
        Self {
            entity: None,
            active: false,
            current_stage: 0,
        }
    }
}

/// ระบบสำหรับเริ่มการเปลี่ยนฉาก
pub fn start_transition(
    mut commands: Commands,
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut active_transition: ResMut<ActiveTransition>,
) {
    // ถ้ามี transition ที่กำลังทำงานอยู่ หรือ stage นี้ได้เริ่ม transition ไปแล้ว ให้ข้าม
    if active_transition.active || active_transition.current_stage == state.stage {
        return;
    }

    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];

                if let Some(transition) = &entry.transition {
                    // สร้าง overlay สำหรับการเปลี่ยนฉาก (ปกคลุมทั้งหน้าจอ)
                    let color = match transition.type_name.as_str() {
                        "fade_in" => Color::srgba(0.0, 0.0, 0.0, 1.0),
                        "fade_out" => Color::srgba(0.0, 0.0, 0.0, 0.0),
                        _ => Color::srgba(0.0, 0.0, 0.0, 0.0),
                    };

                    let entity = commands.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            background_color: color.into(),
                            z_index: ZIndex::Global(100),
                            ..default()
                        },
                        TransitionOverlay {
                            effect_type: transition.type_name.clone(),
                            timer: Timer::from_seconds(transition.duration, TimerMode::Once),
                        },
                        Name::new("transition_overlay"),
                    )).id();

                    active_transition.entity = Some(entity);
                    active_transition.active = true;
                    active_transition.current_stage = state.stage; // บันทึก stage ที่เริ่ม transition

                    info!("เริ่ม transition: {} สำหรับ stage {}", transition.type_name, state.stage);
                }
            }
        }
    }
}

/// ระบบสำหรับอัพเดทการเปลี่ยนฉาก
pub fn update_transition(
    mut commands: Commands,
    time: Res<Time>,
    mut active_transition: ResMut<ActiveTransition>,
    mut transition_query: Query<(Entity, &mut TransitionOverlay, &mut BackgroundColor)>,
) {
    if !active_transition.active {
        return;
    }

    if let Some(entity) = active_transition.entity {
        if let Ok((entity, mut overlay, mut bg_color)) = transition_query.get_mut(entity) {
            overlay.timer.tick(time.delta());

            // คำนวณความคืบหน้าด้วยตัวเอง
            let progress = overlay.timer.elapsed_secs() / overlay.timer.duration().as_secs_f32();
            let progress = progress.clamp(0.0, 1.0); // ป้องกันไม่ให้ค่าเกิน 0-1

            match overlay.effect_type.as_str() {
                "fade_in" => {
                    bg_color.0.set_alpha(1.0 - progress);
                },
                "fade_out" => {
                    bg_color.0.set_alpha(progress);
                },
                "crossfade" => {
                    if progress < 0.5 {
                        bg_color.0.set_alpha(progress * 2.0);
                    } else {
                        bg_color.0.set_alpha((1.0 - progress) * 2.0);
                    }
                },
                _ => {}
            }

            if overlay.timer.finished() {
                commands.entity(entity).despawn_recursive();
                active_transition.active = false;
                active_transition.entity = None;
                info!("จบ transition สำหรับ stage {}", active_transition.current_stage);
            }
        }
    }
}