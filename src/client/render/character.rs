use crate::common::dialog::types::DialogScene;
use crate::common::helium::VNState;
use crate::common::util::identifier::texture;
use bevy::prelude::*;

/// คอมโพเนนต์สำหรับตัวละคร
#[derive(Component)]
pub struct CharacterSprite {
    pub name: String,
}

/// คอมโพเนนต์สำหรับการเปลี่ยน expression
#[derive(Component)]
pub struct ExpressionState {
    pub current: String,
}

/// ตำแหน่งที่กำหนดไว้ล่วงหน้า
const POSITION_LEFT: Vec3 = Vec3::new(-400.0, -50.0, 5.0);
const POSITION_CENTER: Vec3 = Vec3::new(0.0, -50.0, 5.0);
const POSITION_RIGHT: Vec3 = Vec3::new(400.0, -50.0, 5.0);
const POSITION_OFFSCREEN: Vec3 = Vec3::new(1000.0, -50.0, 5.0);

const HIGHLIGHT_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const DIMMED_SCALE: Vec3 = Vec3::new(0.8, 0.8, 0.8);
const HIGHLIGHT_COLOR: Color = Color::WHITE;
const DIMMED_COLOR: Color = Color::srgba(0.6, 0.6, 0.6, 0.8);

/// ระบบสำหรับการแสดงตัวละครเริ่มต้น
pub fn setup_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
) {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            // เพิ่มสไปรต์สำหรับแต่ละตัวละครในฉาก
            for character in &scene.characters {
                if !character.sprite.is_empty() {
                    let sprite_handle = texture(&character.sprite).load::<Image>(&asset_server);

                    // สร้างเอนทิตี้ตัวละคร แต่ให้อยู่นอกจอก่อน
                    commands.spawn((
                        SpriteBundle {
                            texture: sprite_handle,
                            transform: Transform {
                                translation: POSITION_OFFSCREEN,
                                scale: DIMMED_SCALE,
                                ..default()
                            },
                            sprite: Sprite {
                                color: DIMMED_COLOR,
                                ..default()
                            },
                            visibility: Visibility::Hidden,
                            ..default()
                        },
                        CharacterSprite {
                            name: character.name.clone(),
                        },
                        ExpressionState {
                            current: "default".to_string(),
                        },
                        Name::new(format!("character_{}", character.name)),
                    ));
                }
            }
        }
    }
}

/// แปลงตำแหน่งจากสตริงเป็นเวกเตอร์
fn position_from_string(position: &str) -> Vec3 {
    match position.to_lowercase().as_str() {
        "left" => POSITION_LEFT,
        "center" => POSITION_CENTER,
        "right" => POSITION_RIGHT,
        "offscreen" => POSITION_OFFSCREEN,
        _ => POSITION_CENTER,
    }
}

/// ระบบสำหรับอัพเดทตำแหน่งและสถานะของตัวละคร
pub fn update_characters(
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut character_query: Query<(
        &mut Transform,
        &mut Sprite,
        &mut Visibility,
        &CharacterSprite,
    )>,
    time: Res<Time>,
) {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];
                let speaking_character = &entry.character;

                // อัพเดทสถานะตัวละครตามที่กำหนดในเอนทรี่
                for (mut transform, mut sprite, mut visibility, character) in
                    character_query.iter_mut()
                {
                    // เริ่มต้นให้ซ่อนตัวละครทั้งหมด
                    *visibility = Visibility::Hidden;

                    // ตรวจสอบว่าตัวละครนี้มีอยู่ในเอนทรี่ปัจจุบันหรือไม่
                    for char_state in &entry.character_states {
                        if char_state.name == character.name {
                            // แสดงตัวละคร
                            *visibility = Visibility::Visible;

                            // ตั้งค่าตำแหน่ง
                            let target_pos = position_from_string(&char_state.position);
                            // ใช้ lerp manual สำหรับ transform
                            let t = time.delta_seconds() * 5.0;
                            transform.translation.x += (target_pos.x - transform.translation.x) * t;
                            transform.translation.z += (target_pos.z - transform.translation.z) * t;

                            // ตั้งค่าการเน้น (highlight)
                            let is_speaking = character.name == *speaking_character;
                            let should_highlight = char_state.highlight || is_speaking;

                            // อีกแนวทางหนึ่ง: สร้าง Color ใหม่สำหรับการเปลี่ยนแปลงทีละน้อย
                            if should_highlight {
                                // ใช้ lerp manual สำหรับ scale
                                let t = time.delta_seconds() * 8.0;
                                transform.scale.x += (HIGHLIGHT_SCALE.x - transform.scale.x) * t;
                                transform.scale.y += (HIGHLIGHT_SCALE.y - transform.scale.y) * t;
                                transform.scale.z += (HIGHLIGHT_SCALE.z - transform.scale.z) * t;

                                // สร้าง Color ใหม่จากค่าปัจจุบันและเป้าหมาย
                                let current = sprite.color.to_srgba();
                                let target = HIGHLIGHT_COLOR.to_srgba();

                                let new_r = current.red + (target.red - current.red) * t;
                                let new_g = current.green + (target.green - current.green) * t;
                                let new_b = current.blue + (target.blue - current.blue) * t;
                                let new_a = current.alpha + (target.alpha - current.alpha) * t;

                                sprite.color = Color::srgba(new_r, new_g, new_b, new_a);

                                // ยกตัวละครขึ้นเล็กน้อยเมื่อพูด
                                let target_y = target_pos.y + 20.0;
                                transform.translation.y += (target_y - transform.translation.y) * t;
                            } else {
                                // ทำแบบเดียวกันสำหรับสถานะที่ไม่ได้ highlight
                                let t = time.delta_seconds() * 8.0;
                                transform.scale.x += (DIMMED_SCALE.x - transform.scale.x) * t;
                                transform.scale.y += (DIMMED_SCALE.y - transform.scale.y) * t;
                                transform.scale.z += (DIMMED_SCALE.z - transform.scale.z) * t;

                                let current = sprite.color.to_srgba();
                                let target = DIMMED_COLOR.to_srgba();

                                let new_r = current.red + (target.red - current.red) * t;
                                let new_g = current.green + (target.green - current.green) * t;
                                let new_b = current.blue + (target.blue - current.blue) * t;
                                let new_a = current.alpha + (target.alpha - current.alpha) * t;

                                sprite.color = Color::srgba(new_r, new_g, new_b, new_a);

                                transform.translation.y +=
                                    (target_pos.y - transform.translation.y) * t;
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}
