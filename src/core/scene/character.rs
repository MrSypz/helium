use bevy::prelude::*;
use crate::types::DialogScene;
use crate::core::resources::VNState;
use crate::util::identifier::texture;

#[derive(Component)]
pub struct CharacterSprite {
    pub name: String,
    pub loaded: bool,
}

#[derive(Component)]
pub struct ExpressionState {
    pub current: String,
}

const POSITION_LEFT: Vec3 = Vec3::new(-400.0, -50.0, 5.0);
const POSITION_CENTER: Vec3 = Vec3::new(0.0, -50.0, 5.0);
const POSITION_RIGHT: Vec3 = Vec3::new(400.0, -50.0, 5.0);
const POSITION_OFFSCREEN: Vec3 = Vec3::new(1000.0, -50.0, 5.0);

const HIGHLIGHT_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const DIMMED_SCALE: Vec3 = Vec3::new(0.8, 0.8, 0.8);
const HIGHLIGHT_COLOR: Color = Color::WHITE;
const DIMMED_COLOR: Color = Color::srgba(0.6, 0.6, 0.6, 0.8);

pub fn setup_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
    character_query: Query<&CharacterSprite>,
) {
    if !character_query.is_empty() {
        return;
    }

    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            for character in &scene.characters {
                if !character.sprite.is_empty() {
                    let sprite_handle = texture(&character.sprite).load::<Image>(&asset_server);

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
                            loaded: false,
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

pub fn check_character_assets(
    asset_server: Res<AssetServer>,
    mut character_query: Query<(&mut CharacterSprite, &Handle<Image>)>,
) {
    for (mut character, texture_handle) in character_query.iter_mut() {
        if !character.loaded {
            match asset_server.load_state(texture_handle.id()) {
                bevy::asset::LoadState::Loaded => {
                    character.loaded = true;
                },
                bevy::asset::LoadState::Failed(_) => {
                    character.loaded = true;
                },
                _ => {}
            }
        }
    }
}

fn position_from_string(position: &str) -> Vec3 {
    match position.to_lowercase().as_str() {
        "left" => POSITION_LEFT,
        "center" => POSITION_CENTER,
        "right" => POSITION_RIGHT,
        "offscreen" => POSITION_OFFSCREEN,
        _ => POSITION_CENTER,
    }
}

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

                for (mut transform, mut sprite, mut visibility, character) in
                    character_query.iter_mut()
                {
                    let mut should_show = false;

                    for char_state in &entry.character_states {
                        if char_state.name == character.name {
                            should_show = true;
                            *visibility = Visibility::Visible;

                            let target_pos = position_from_string(&char_state.position);
                            let t = time.delta_seconds() * 5.0;
                            transform.translation.x += (target_pos.x - transform.translation.x) * t;
                            transform.translation.z += (target_pos.z - transform.translation.z) * t;

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

                    if !should_show {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}