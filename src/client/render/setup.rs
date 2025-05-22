use bevy::prelude::*;
use crate::util::identifier::texture;
use crate::common::helium::VNState;
use crate::common::dialog::types::DialogScene;

/// คอมโพเนนต์สำหรับพื้นหลัง
#[derive(Component)]
pub struct Background;

/// ระบบสำหรับตั้งค่าฉาก
pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
) {
    // กล้อง
    commands.spawn(Camera2dBundle::default());

    // พื้นหลังเริ่มต้น
    let default_bg = "backgrounds/school.png";

    // หาพื้นหลังจากฉากปัจจุบัน
    let mut bg_path = default_bg;

    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if !scene.default_background.is_empty() {
                bg_path = &scene.default_background;
            }
        }
    }

    let bg_texture = texture(bg_path).load::<Image>(&asset_server);

    // พื้นหลัง
    commands.spawn((
        SpriteBundle {
            texture: bg_texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Background,
        Name::new("background"),
    ));
}

/// ระบบสำหรับอัพเดทพื้นหลัง
pub fn update_background(
    asset_server: Res<AssetServer>,
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut background_query: Query<(Entity, &mut Handle<Image>), With<Background>>,
) {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];

                // ตรวจสอบว่ามีการเปลี่ยนพื้นหลังหรือไม่
                if let Some(bg_path) = &entry.background {
                    if !bg_path.is_empty() {
                        let new_bg = texture(bg_path).load::<Image>(&asset_server);

                        // อัพเดทพื้นหลัง
                        if let Ok((_, mut bg_handle)) = background_query.get_single_mut() {
                            *bg_handle = new_bg;
                        }
                    }
                }
            }
        }
    }
}