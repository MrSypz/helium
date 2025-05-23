// src/client/render/character.rs
use crate::common::dialog::types::DialogScene;
use crate::common::helium::VNState;
use crate::util::identifier::texture;
use bevy::prelude::*;

/// คอมโพเนนต์สำหรับตัวละคร
#[derive(Component)]
pub struct CharacterSprite {
    pub name: String,
    pub loaded: bool,
    pub last_stage: Option<usize>, // เพิ่ม field นี้เพื่อติดตาม stage ล่าสุดที่อัพเดท
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

/// ระบบสำหรับการตรวจสอบและโหลดตัวละคร
pub fn setup_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
    character_query: Query<&CharacterSprite>,
) {
    // ตรวจสอบว่ามีตัวละครถูกสร้างแล้วหรือไม่
    if !character_query.is_empty() {
        return;
    }

    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            info!("กำลังสร้างตัวละคร {} ตัว", scene.characters.len());

            // เพิ่มสไปรต์สำหรับแต่ละตัวละครในฉาก
            for character in &scene.characters {
                if !character.sprite.is_empty() {
                    info!("สร้างตัวละคร: {} จากไฟล์: {}", character.name, character.sprite);

                    let sprite_handle = texture(&character.sprite).load::<Image>(&asset_server);

                    // สร้างเอนทิตี้ตัวละคร
                    let entity = commands.spawn((
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
                            loaded: false,
                            last_stage: None, // เริ่มต้นไม่มี stage
                        },
                        ExpressionState {
                            current: "default".to_string(),
                        },
                        Name::new(format!("character_{}", character.name)),
                    )).id();

                    info!("สร้างตัวละคร entity: {:?} สำหรับ {}", entity, character.name);
                }
            }
        }
    }
}

/// ระบบตรวจสอบว่า assets โหลดเสร็จแล้วหรือไม่
pub fn check_character_assets(
    asset_server: Res<AssetServer>,
    mut character_query: Query<(&mut CharacterSprite, &Handle<Image>)>,
) {
    for (mut character, texture_handle) in character_query.iter_mut() {
        if !character.loaded {
            match asset_server.load_state(texture_handle.id()) {
                bevy::asset::LoadState::Loaded => {
                    character.loaded = true;
                    info!("ตัวละคร {} โหลดเสร็จแล้ว", character.name);
                },
                bevy::asset::LoadState::Failed(e) => {
                    warn!("ไม่สามารถโหลดตัวละคร {} ได้ เพราะ {}", character.name, e);
                    character.loaded = true; // ทำเครื่องหมายว่าพยายามโหลดแล้ว
                },
                _ => {
                    // ยังโหลดไม่เสร็จ - ไม่ต้อง log
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
        _ => {
            warn!("ตำแหน่งไม่รู้จัก: {}, ใช้ center แทน", position);
            POSITION_CENTER
        }
    }
}

/// ระบบสำหรับอัพเดทตำแหน่งและสถานะของตัวละคร - ลด logging
pub fn update_characters(
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut character_query: Query<(
        &mut Transform,
        &mut Sprite,
        &mut Visibility,
        &mut CharacterSprite,
    )>,
    time: Res<Time>,
) {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];
                let speaking_character = &entry.character;

                // อัพเดทตัวละครทั้งหมด
                for (mut transform, mut sprite, mut visibility, mut character) in
                    character_query.iter_mut()
                {
                    // อัพเดท last_stage
                    character.last_stage = Some(state.stage);

                    // เริ่มต้นให้ซ่อนตัวละครทั้งหมด
                    let mut should_show = false;

                    // ตรวจสอบว่าตัวละครนี้มีอยู่ในเอนทรี่ปัจจุบันหรือไม่
                    for char_state in &entry.character_states {
                        if char_state.name == character.name {
                            should_show = true;

                            // แสดงตัวละคร
                            *visibility = Visibility::Visible;

                            // ตั้งค่าตำแหน่ง
                            let target_pos = position_from_string(&char_state.position);
                            let t = time.delta_seconds() * 5.0;
                            transform.translation.x += (target_pos.x - transform.translation.x) * t;
                            transform.translation.z += (target_pos.z - transform.translation.z) * t;

                            // ตั้งค่าการเน้น (highlight)
                            let is_speaking = character.name == *speaking_character;
                            let should_highlight = char_state.highlight || is_speaking;

                            if should_highlight {
                                let t = time.delta_seconds() * 8.0;
                                transform.scale.x += (HIGHLIGHT_SCALE.x - transform.scale.x) * t;
                                transform.scale.y += (HIGHLIGHT_SCALE.y - transform.scale.y) * t;
                                transform.scale.z += (HIGHLIGHT_SCALE.z - transform.scale.z) * t;

                                let current = sprite.color.to_srgba();
                                let target = HIGHLIGHT_COLOR.to_srgba();

                                let new_r = current.red + (target.red - current.red) * t;
                                let new_g = current.green + (target.green - current.green) * t;
                                let new_b = current.blue + (target.blue - current.blue) * t;
                                let new_a = current.alpha + (target.alpha - current.alpha) * t;

                                sprite.color = Color::srgba(new_r, new_g, new_b, new_a);

                                let target_y = target_pos.y + 20.0;
                                transform.translation.y += (target_y - transform.translation.y) * t;
                            } else {
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

                                transform.translation.y += (target_pos.y - transform.translation.y) * t;
                            }
                            break;
                        }
                    }

                    // ถ้าไม่ควรแสดง ให้ซ่อน
                    if !should_show {
                        *visibility = Visibility::Hidden;
                    }
                }
            } else {
                // Log เฉพาะครั้งแรกที่เกิด error
                static mut LOGGED_STAGE_ERROR: Option<usize> = None;
                unsafe {
                    if LOGGED_STAGE_ERROR != Some(state.stage) {
                        warn!("Stage {} เกินจำนวน entries ({}) ที่มี", state.stage, scene.entries.len());
                        LOGGED_STAGE_ERROR = Some(state.stage);
                    }
                }
            }
        }
    }
}

/// ระบบสำหรับ debug ตัวละคร
pub fn debug_characters(
    keyboard: Res<ButtonInput<KeyCode>>,
    character_query: Query<&CharacterSprite>,
    state: Res<VNState>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        info!("=== Character Debug ===");
        info!("จำนวนตัวละครที่สร้างแล้ว: {}", character_query.iter().count());
        info!("Stage ปัจจุบัน: {}", state.stage);
        info!("Scene ปัจจุบัน: {}", state.current_scene);

        for character in character_query.iter() {
            info!("ตัวละคร: {} (โหลดแล้ว: {}, last_stage: {:?})",
                  character.name, character.loaded, character.last_stage);
        }
    }
}