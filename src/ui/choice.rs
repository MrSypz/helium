use bevy::prelude::*;
use crate::core::resources::VNState;
use crate::core::dialog::choice::{ChoiceState, ChoiceButton};
use crate::core::dialog::typewriter::TypewriterText;
use crate::core::language::manager::{LanguageResource, get_text};
use crate::core::language::types::LanguagePack;
use crate::ui::dialog::DialogText;
use crate::types::{DialogScene, DialogChoice};

#[derive(Component)]
pub struct ChoiceContainer;

#[derive(Component)]
pub struct ChoiceText;

#[derive(Component)]
pub struct ChoiceOverlay;

const CHOICE_Z_LAYER: f32 = 15.0;
const CHOICE_TITLE_COLOR: Color = Color::srgb(1.0, 0.85, 0.3);
const CHOICE_TEXT_COLOR: Color = Color::WHITE;
const CHOICE_PANEL_BG: Color = Color::srgba(0.12, 0.12, 0.18, 0.95);
const CHOICE_BORDER_COLOR: Color = Color::srgba(0.5, 0.5, 0.7, 0.5);
const CHOICE_BUTTON_BG: Color = Color::srgba(0.2, 0.2, 0.25, 0.9);
const CHOICE_BUTTON_BORDER: Color = Color::srgba(0.6, 0.6, 0.7, 0.5);
const CHOICE_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.95);
const CHOICE_BUTTON_ACTIVE: Color = Color::srgba(0.4, 0.4, 0.5, 1.0);

pub fn manage_choice_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<VNState>,
    dialog_scenes: Res<Assets<DialogScene>>,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    mut choice_state: ResMut<ChoiceState>,
    existing_containers: Query<Entity, With<ChoiceContainer>>,
    existing_overlays: Query<Entity, With<ChoiceOverlay>>,
    typewriter_query: Query<&TypewriterText, With<DialogText>>,
) {
    let should_show_choices = should_display_choices(&state, &dialog_scenes, &typewriter_query);

    if should_show_choices && !choice_state.active {
        if let Some(choices) = get_current_choices(&state, &dialog_scenes) {
            cleanup_existing_choices(&mut commands, &existing_containers, &existing_overlays);
            choice_state.activate(choices.clone());
            create_choice_ui(
                &mut commands,
                &asset_server,
                &state,
                &language_resource,
                &language_packs,
                &choices
            );
        }
    } else if !should_show_choices && choice_state.active {
        choice_state.deactivate();
        cleanup_existing_choices(&mut commands, &existing_containers, &existing_overlays);
    }
}

fn should_display_choices(
    state: &VNState,
    dialog_scenes: &Assets<DialogScene>,
    typewriter_query: &Query<&TypewriterText, With<DialogText>>,
) -> bool {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];

                if !entry.choices.is_empty() {
                    if let Ok(typewriter) = typewriter_query.get_single() {
                        return typewriter.char_index >= typewriter.full_text.chars().count();
                    }
                    return true;
                }
            }
        }
    }
    false
}

fn get_current_choices(
    state: &VNState,
    dialog_scenes: &Assets<DialogScene>,
) -> Option<Vec<DialogChoice>> {
    if let Some(scene_handle) = &state.current_scene_handle {
        if let Some(scene) = dialog_scenes.get(scene_handle) {
            if state.stage < scene.entries.len() {
                let entry = &scene.entries[state.stage];
                if !entry.choices.is_empty() {
                    return Some(entry.choices.clone());
                }
            }
        }
    }
    None
}

fn cleanup_existing_choices(
    commands: &mut Commands,
    containers: &Query<Entity, With<ChoiceContainer>>,
    overlays: &Query<Entity, With<ChoiceOverlay>>,
) {
    for entity in containers.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for entity in overlays.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn create_choice_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    state: &VNState,
    language_resource: &LanguageResource,
    language_packs: &Assets<LanguagePack>,
    choices: &[DialogChoice],
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.6).into(),
            z_index: ZIndex::Global((CHOICE_Z_LAYER - 1.0) as i32),
            ..default()
        },
        Name::new("choice_overlay"),
        ChoiceOverlay,
    ));

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(70.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Percent(15.0),
                top: Val::Percent(20.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(25.0)),
                ..default()
            },
            background_color: CHOICE_PANEL_BG.into(),
            border_color: CHOICE_BORDER_COLOR.into(),
            border_radius: BorderRadius::all(Val::Px(20.0)),
            z_index: ZIndex::Global(CHOICE_Z_LAYER as i32),
            ..default()
        },
        Name::new("choice_container"),
        ChoiceContainer,
    )).with_children(|parent| {
        // Title - ใช้ language system
        parent.spawn((
            TextBundle::from_section(
                get_text(language_resource, language_packs, "dialog.choose_action"),
                TextStyle {
                    font: asset_server.load("fonts/NotoSansThai-Bold.ttf"),
                    font_size: 32.0,
                    color: CHOICE_TITLE_COLOR,
                },
            )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                }),
            Name::new("choice_title"),
        ));

        for (i, choice) in choices.iter().enumerate() {
            create_choice_button(parent, asset_server, state, i, choice);
        }
    });
}

fn create_choice_button(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    state: &VNState,
    index: usize,
    choice: &DialogChoice,
) {
    // ใช้ current language จาก state.language แทน hardcode
    let current_lang = match state.language.as_str() {
        "thai" => "thai",
        "english" => "english",
        "japanese" => "japanese",
        _ => "thai", // fallback
    };

    let choice_text = choice.text.get(current_lang)
        .cloned()
        .unwrap_or_else(|| format!("[No choice text in {}]", current_lang));

    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                min_height: Val::Px(60.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.5)),
                margin: UiRect::all(Val::Px(6.0)),
                padding: UiRect::all(Val::Px(12.0)),
                ..default()
            },
            background_color: CHOICE_BUTTON_BG.into(),
            border_color: CHOICE_BUTTON_BORDER.into(),
            border_radius: BorderRadius::all(Val::Px(12.0)),
            ..default()
        },
        ChoiceButton {
            choice_index: index,
            target_stage: choice.target_stage,
        },
        Name::new(format!("choice_button_{}", index)),
    )).with_children(|button| {
        button.spawn(NodeBundle {
            style: Style {
                width: Val::Px(30.0),
                height: Val::Px(30.0),
                margin: UiRect::right(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgba(0.3, 0.3, 0.5, 0.8).into(),
            ..default()
        }).with_children(|number_circle| {
            number_circle.spawn(TextBundle::from_section(
                format!("{}", index + 1),
                TextStyle {
                    font: asset_server.load("fonts/NotoSansThai-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });

        button.spawn((
            TextBundle::from_section(
                choice_text,
                TextStyle {
                    font: asset_server.load("fonts/NotoSansThai-Regular.ttf"),
                    font_size: 26.0,
                    color: CHOICE_TEXT_COLOR,
                },
            )
                .with_style(Style {
                    margin: UiRect::left(Val::Px(5.0)),
                    max_width: Val::Percent(90.0),
                    ..default()
                }),
            ChoiceText,
            Name::new(format!("choice_text_{}", index)),
        ));
    });
}

pub fn highlight_choice_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ChoiceButton>)>,
) {
    for (interaction, mut bg_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = CHOICE_BUTTON_ACTIVE.into();
            }
            Interaction::Hovered => {
                *bg_color = CHOICE_BUTTON_HOVER.into();
            }
            Interaction::None => {
                *bg_color = CHOICE_BUTTON_BG.into();
            }
        }
    }
}