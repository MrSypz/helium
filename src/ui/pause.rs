// src/ui/pause.rs
use bevy::prelude::*;
use crate::core::game_state::{GameState, ChangeStateEvent};
use crate::core::language::manager::LanguageResource;
use crate::core::language::types::LanguagePack;
use crate::core::text::styles::TextStyleResource;
use crate::core::text::components::TextStylePreset;
use crate::core::text::builder::TextBuilder;

#[derive(Component)]
pub struct PauseUI;

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct MainMenuButton;

const PAUSE_Z_LAYER: f32 = 20.0;
const PAUSE_OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
const PAUSE_PANEL_COLOR: Color = Color::srgba(0.1, 0.1, 0.15, 0.95);
const PAUSE_BUTTON_COLOR: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);
const PAUSE_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.9);
const PAUSE_BUTTON_PRESSED: Color = Color::srgba(0.4, 0.4, 0.5, 0.9);

pub fn setup_pause_ui(
    mut commands: Commands,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    text_styles: Res<TextStyleResource>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: PAUSE_OVERLAY_COLOR.into(),
            z_index: ZIndex::Global(PAUSE_Z_LAYER as i32),
            ..default()
        },
        PauseUI,
        Name::new("pause_overlay"),
    )).with_children(|overlay| {
        // Main pause panel
        overlay.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Auto,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    padding: UiRect::all(Val::Px(40.0)),
                    row_gap: Val::Px(25.0),
                    ..default()
                },
                background_color: PAUSE_PANEL_COLOR.into(),
                border_color: Color::srgba(0.4, 0.4, 0.5, 0.6).into(),
                border_radius: BorderRadius::all(Val::Px(15.0)),
                ..default()
            },
            Name::new("pause_panel"),
        )).with_children(|panel| {
            // Pause Title
            TextBuilder::localized_child(
                panel,
                "ui.paused_title",
                TextStylePreset::Custom(48.0, true, Color::srgb(1.0, 0.8, 0.2)),
                &language_resource,
                &language_packs,
                &text_styles,
            );

            // Resume Button
            create_pause_button(
                panel,
                "ui.resume_game",
                ResumeButton,
                &language_resource,
                &language_packs,
                &text_styles,
            );

            // Main Menu Button
            create_pause_button(
                panel,
                "ui.main_menu",
                MainMenuButton,
                &language_resource,
                &language_packs,
                &text_styles,
            );

            // Controls hint
            panel.spawn((
                NodeBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
            )).with_children(|hint_container| {
                TextBuilder::localized_child(
                    hint_container,
                    "ui.pause_controls_hint",
                    TextStylePreset::Hint,
                    &language_resource,
                    &language_packs,
                    &text_styles,
                );
            });
        });
    });
}

fn create_pause_button<T: Component>(
    parent: &mut ChildBuilder,
    text_key: &str,
    button_component: T,
    language_resource: &LanguageResource,
    language_packs: &Assets<LanguagePack>,
    text_styles: &TextStyleResource,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(280.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(8.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: PAUSE_BUTTON_COLOR.into(),
            border_color: Color::srgba(0.4, 0.4, 0.5, 0.5).into(),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        button_component,
        Name::new("pause_button"),
    )).with_children(|button| {
        TextBuilder::localized_child(
            button,
            text_key,
            TextStylePreset::Button,
            language_resource,
            language_packs,
            text_styles,
        );
    });
}

pub fn handle_pause_button_hover(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, Or<(With<ResumeButton>, With<MainMenuButton>)>)
    >,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PAUSE_BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *bg_color = PAUSE_BUTTON_HOVER.into();
            }
            Interaction::None => {
                *bg_color = PAUSE_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn handle_pause_buttons(
    resume_query: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    main_menu_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut change_events: EventWriter<ChangeStateEvent>,
) {
    // Resume Game
    for interaction in resume_query.iter() {
        if *interaction == Interaction::Pressed {
            change_events.send(ChangeStateEvent {
                new_state: GameState::InGame,
            });
        }
    }
    // Main Menu
    for interaction in main_menu_query.iter() {
        if *interaction == Interaction::Pressed {
            change_events.send(ChangeStateEvent {
                new_state: GameState::MainMenu,
            });
        }
    }
}

pub fn cleanup_pause_ui(
    mut commands: Commands,
    pause_query: Query<Entity, With<PauseUI>>,
) {
    for entity in pause_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}