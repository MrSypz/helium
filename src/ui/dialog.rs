use bevy::prelude::*;
use crate::core::resources::{DialogHistory, DialogResource, VNState};
use crate::core::dialog::choice::ChoiceState;
use crate::core::dialog::typewriter::TypewriterText;
use crate::core::language::manager::{LanguageResource, LanguageChangeEvent};
use crate::core::language::types::LanguagePack;
use crate::core::text::styles::TextStyleResource;
use crate::core::text::components::TextStylePreset;
use crate::core::text::builder::TextBuilder;
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
const DIALOG_BG_COLOR: Color = Color::srgba(0.05, 0.05, 0.1, 0.85);
const DIALOG_BORDER_COLOR: Color = Color::srgba(0.3, 0.3, 0.5, 0.5);

pub fn setup_dialog_ui(
    mut commands: Commands,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    text_styles: Res<TextStyleResource>,
) {
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
            // Character name container
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
                    border_color: Color::srgb(1.0, 0.8, 0.2).with_alpha(0.7).into(),
                    border_radius: BorderRadius::all(Val::Px(10.0)),
                    ..default()
                },))
                .with_children(|name_box| {
                    // Character name - สร้าง text พร้อม components ในคำสั่งเดียว
                    TextBuilder::static_child_with_components(
                        name_box,
                        "",
                        TextStylePreset::DialogName,
                        &language_resource,
                        &text_styles,
                        (CharacterName, Name::new("character_name")),
                    );
                });

            // Dialog text - สร้างแยกเพื่อเพิ่ม TypewriterText
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStylePreset::DialogText.to_style(&text_styles, &language_resource.current_language)
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
                DialogText,
                Name::new("dialogue"),
                TypewriterText::new("", 0.05),
            ));

            // Controls container
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
            )).with_children(|controls| {
                // Language indicator
                TextBuilder::localized_child_with_components(
                    controls,
                    "dialog.language_indicator",
                    TextStylePreset::Custom(24.0, true, Color::srgba(0.8, 0.8, 0.9, 0.8)),
                    &language_resource,
                    &language_packs,
                    &text_styles,
                    (LanguageIndicator, Name::new("language_indicator")),
                );
            });
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

/// Update dialog text fonts เมื่อเปลี่ยนภาษา
pub fn update_dialog_fonts(
    mut language_events: EventReader<LanguageChangeEvent>,
    language_resource: Res<LanguageResource>,
    text_styles: Res<TextStyleResource>,
    mut character_name_query: Query<&mut Text, (With<CharacterName>, Without<DialogText>)>,
    mut dialog_text_query: Query<&mut Text, (With<DialogText>, Without<CharacterName>)>,
) {
    for _event in language_events.read() {
        // Update character name font
        for mut text in character_name_query.iter_mut() {
            if !text.sections.is_empty() {
                text.sections[0].style = TextStylePreset::DialogName.to_style(&text_styles, &language_resource.current_language);
            }
        }

        // Update dialog text font
        for mut text in dialog_text_query.iter_mut() {
            if !text.sections.is_empty() {
                text.sections[0].style = TextStylePreset::DialogText.to_style(&text_styles, &language_resource.current_language);
            }
        }
    }
}
