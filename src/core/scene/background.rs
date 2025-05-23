use bevy::prelude::*;
use crate::util::identifier::texture;
use crate::core::resources::VNState;
use crate::types::DialogScene;

#[derive(Component)]
pub struct Background {
    pub current_path: String,
}

pub fn setup_scene_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
) {
    let default_bg = "backgrounds/school.png";
    let mut bg_path = default_bg.to_string();

    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if !scene.default_background.is_empty() {
                bg_path = scene.default_background.clone();
            }
        }
    }

    let bg_texture = texture(&bg_path).load::<Image>(&asset_server);

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

                if let Some(bg_path) = &entry.background {
                    if !bg_path.is_empty() {
                        if let Ok((_, mut bg_handle, mut background)) = background_query.get_single_mut() {
                            if background.current_path != *bg_path {
                                let new_bg = texture(bg_path).load::<Image>(&asset_server);
                                *bg_handle = new_bg;
                                background.current_path = bg_path.clone();
                            }
                        }
                    }
                }
            }
        }
    }
}