use bevy::prelude::*;
use crate::core::resources::{DialogHistory, DialogResource, VNState};
use crate::core::dialog::choice::ChoiceState;
use crate::core::dialog::typewriter::TypewriterText;
use crate::util::input;
use crate::types::DialogScene;

#[derive(Component)]
pub struct DialogBox;

#[derive(Component)]
pub struct CharacterName;

#[derive(Component)]
pub struct DialogText;

#[derive(Component)]
pub struct DialogControls;

#[derive(Component)]
pub struct LanguageIndicator;

const DIALOG_Z_LAYER: f32 = 10.0;
const TEXT_COLOR: Color = Color::WHITE;
const NAME_COLOR: Color = Color::srgb(1.0, 0.8, 0.2);
const DIALOG_BG_COLOR: Color = Color::srgba(0.05, 0.05, 0.1, 0.85);
const DIALOG_BORDER_COLOR: Color = Color::srgba(0.3, 0.3, 0.5, 0.5);

pub fn setup_dialog_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let regular_font = asset_server.load("fonts/NotoSansThai-Regular.ttf");
    let bold_font = asset_server.load("fonts/NotoSansThai-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    min_height: Val::Percent(30.0),
                    max_height: Val::Percent(40.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                background_color: DIALOG_BG_COLOR.into(),
                border_color: DIALOG_BORDER_COLOR.into(),
                border_radius: BorderRadius::all(Val::Px(15.0)),
                z_index: ZIndex::Global(DIALOG_Z_LAYER as i32),
                ..default()
            },
            DialogBox,
            Name::new("dialog_box"),
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        margin: UiRect::bottom(Val::Px(10.0)),
                        padding: UiRect::new(
                            Val::Px(15.0),
                            Val::Px(15.0),
                            Val::Px(5.0),
                            Val::Px(5.0),
                        ),
                        ..default()
                    },
                    background_color: Color::srgba(0.1, 0.1, 0.2, 0.9).into(),
                    border_color: NAME_COLOR.with_alpha(0.7).into(),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },))
                .with_children(|name_box| {
                    name_box.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: bold_font.clone(),
                                font_size: 32.0,
                                color: NAME_COLOR,
                            },
                        ),
                        CharacterName,
                        Name::new("character_name"),
                    ));
                });

            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: regular_font.clone(),
                        font_size: 30.0,
                        color: TEXT_COLOR,
                    },
                )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    }),
                DialogText,
                Name::new("dialogue"),
                TypewriterText::new("", 0.05),
            ));

            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        margin: UiRect::top(Val::Px(10.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                DialogControls,
            ));
        });
}

pub fn handle_text_interaction(
    mut state: ResMut<VNState>,
    mut history: ResMut<DialogHistory>,
    mut dialog_resource: ResMut<DialogResource>,
    dialog_scenes: Res<Assets<DialogScene>>,
    choice_state: Res<ChoiceState>,
    mouse: Res<ButtonInput<MouseButton>>,
    touch: Res<Touches>,
    dialog_box_query: Query<&Interaction, (With<DialogBox>, Changed<Interaction>)>,
    mut dialog_query: Query<(&mut Text, &mut TypewriterText), With<DialogText>>,
) {
    if choice_state.active {
        return;
    }

    if let Some(_) = input::detect_interaction(&mouse, &touch, &dialog_box_query) {
        if let Some(scene_handle) = &dialog_resource.current_scene {
            if let Some(scene) = dialog_scenes.get(scene_handle) {
                let is_finished = {
                    if let Ok((_, typewriter)) = dialog_query.get_single() {
                        input::is_dialog_text_finished(typewriter)
                    } else {
                        false
                    }
                };

                if is_finished {
                    process_stage_progression(&mut state, &mut history, &mut dialog_resource, scene);
                } else {
                    if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
                        text.sections[0].value = typewriter.full_text.clone();
                        typewriter.current_text = typewriter.full_text.clone();
                        typewriter.char_index = typewriter.full_text.chars().count();
                    }
                }
            }
        }
    }
}

fn process_stage_progression(
    state: &mut VNState,
    history: &mut DialogHistory,
    dialog_resource: &mut DialogResource,
    scene: &DialogScene,
) {
    if state.stage < scene.entries.len() {
        let entry = &scene.entries[state.stage];

        if !entry.actions.is_empty() {
            for action in &entry.actions {
                if let Some(scene_name) = action.strip_prefix("change_scene:") {
                    if dialog_resource.change_scene(scene_name, state) {
                        return;
                    }
                }
            }
        }
    }

    let auto_target = if state.stage < scene.entries.len() {
        scene.entries[state.stage].auto_proceed
    } else {
        None
    };

    history.add_stage(state.stage);

    if let Some(target) = auto_target {
        state.change_stage(target);
    } else {
        let next_stage = (state.stage + 1) % scene.entries.len();
        state.change_stage(next_stage);
    }
}

pub fn handle_language_toggle(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<VNState>,
    mut language_query: Query<&mut Text, With<LanguageIndicator>>,
) {
    if let Some(_) = input::detect_key_press(&keyboard, &[KeyCode::KeyL]) {
        let new_language = if state.language == "thai" {
            "english".to_string()
        } else {
            "thai".to_string()
        };

        state.change_language(new_language);

        if let Ok(mut lang_text) = language_query.get_single_mut() {
            lang_text.sections[0].value = if state.language == "thai" {
                "TH".to_string()
            } else {
                "EN".to_string()
            };
        }
    }
}