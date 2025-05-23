use std::collections::HashSet;
use std::sync::LazyLock;
use bevy::prelude::*;
use crate::util::identifier::texture;
use crate::common::helium::VNState;
use crate::common::dialog::types::DialogScene;

/// คอมโพเนนต์สำหรับพื้นหลัง
#[derive(Component)]
pub struct Background {
    pub current_path: String, // เพิ่มการติดตามพาธปัจจุบัน
}

/// ระบบสำหรับตั้งค่าฉาก
pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
) {
    // พื้นหลังเริ่มต้น
    let default_bg = "backgrounds/school.png";

    // หาพื้นหลังจากฉากปัจจุบัน
    let mut bg_path = default_bg.to_string();

    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if !scene.default_background.is_empty() {
                bg_path = scene.default_background.clone();
            }
        }
    }

    let bg_texture = texture(&bg_path).load::<Image>(&asset_server);

    info!("สร้างพื้นหลังด้วยไฟล์: {}", bg_path);

    // พื้นหลัง
    commands.spawn((
        SpriteBundle {
            texture: bg_texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Background {
            current_path: bg_path,
        },
        Name::new("background"),
    ));
}

/// ระบบสำหรับอัพเดทพื้นหลัง - ปรับปรุงการ logging
pub fn update_background(
    asset_server: Res<AssetServer>,
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut background_query: Query<(Entity, &mut Handle<Image>, &mut Background), With<Background>>,
) {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];

                // ตรวจสอบว่ามีการเปลี่ยนพื้นหลังหรือไม่
                if let Some(bg_path) = &entry.background {
                    if !bg_path.is_empty() {
                        // อัพเดทเฉพาะเมื่อพื้นหลังเปลี่ยน
                        if let Ok((_, mut bg_handle, mut background)) = background_query.get_single_mut() {
                            if background.current_path != *bg_path {
                                let new_bg = texture(bg_path).load::<Image>(&asset_server);
                                *bg_handle = new_bg;
                                background.current_path = bg_path.clone();
                                info!("เปลี่ยนพื้นหลังเป็น: {}", bg_path);
                            }
                        }
                    }
                }
            }
        }
    }
}
static LOGGED_ASSETS: LazyLock<std::sync::Mutex<HashSet<String>>> =
    LazyLock::new(|| std::sync::Mutex::new(HashSet::new()));
/// ระบบตรวจสอบ asset loading status - ใหม่
pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    background_query: Query<&Handle<Image>, With<Background>>,
) {
    for bg_handle in background_query.iter() {
        match asset_server.load_state(bg_handle.id()) {
            bevy::asset::LoadState::Failed(e) => {
                let asset_path = format!("{:?}", bg_handle.id());

                let mut logged = LOGGED_ASSETS.lock().unwrap();
                if !logged.contains(&asset_path) {
                    warn!("ไม่สามารถโหลดพื้นหลังได้: {}", e);
                    logged.insert(asset_path);
                }
            },
            bevy::asset::LoadState::Loaded => {
                // Asset โหลดสำเร็จ - ไม่ต้อง log
            },
            _ => {
                // กำลังโหลด - ไม่ต้อง log
            }
        }
    }
}
