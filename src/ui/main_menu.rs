use bevy::prelude::*;
use crate::core::game_state::{GameState, ChangeStateEvent};
use crate::core::language::manager::LanguageResource;
use crate::core::language::types::LanguagePack;
use crate::core::text::styles::TextStyleResource;
use crate::core::text::components::TextStylePreset;
use crate::core::text::builder::TextBuilder;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct ExitGameButton;

const MENU_BUTTON_COLOR: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);
const MENU_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.9);
const MENU_BUTTON_PRESSED: Color = Color::srgba(0.4, 0.4, 0.5, 0.9);

pub fn setup_main_menu(
    mut commands: Commands,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    text_styles: Res<TextStyleResource>,
) {
    // Background
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.2, 1.0).into(),
            ..default()
        },
        MainMenuUI,
        Name::new("menu_background"),
    ));

    // Main container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        MainMenuUI,
        Name::new("menu_container"),
    )).with_children(|parent| {
        // Game Title
        TextBuilder::localized_child(
            parent,
            "ui.game_title",
            TextStylePreset::Custom(64.0, true, Color::srgb(1.0, 0.8, 0.2)),
            &language_resource,
            &language_packs,
            &text_styles,
        );

        // Game Subtitle
        parent.spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
                ..default()
            },
        )).with_children(|subtitle_container| {
            TextBuilder::localized_child(
                subtitle_container,
                "ui.game_subtitle",
                TextStylePreset::Subtitle,
                &language_resource,
                &language_packs,
                &text_styles,
            );
        });

        // Start Game Button
        create_menu_button(
            parent,
            "ui.start_game",
            StartGameButton,
            &language_resource,
            &language_packs,
            &text_styles,
        );

        // Settings Button
        create_menu_button(
            parent,
            "ui.settings",
            SettingsButton,
            &language_resource,
            &language_packs,
            &text_styles,
        );

        // Exit Game Button
        create_menu_button(
            parent,
            "ui.exit_game",
            ExitGameButton,
            &language_resource,
            &language_packs,
            &text_styles,
        );

        // Controls Help
        parent.spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(60.0)),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            Name::new("instructions_container"),
        )).with_children(|instructions| {
            TextBuilder::localized_child(
                instructions,
                "ui.controls_help",
                TextStylePreset::Hint,
                &language_resource,
                &language_packs,
                &text_styles,
            );
        });
    });
}

fn create_menu_button<T: Component>(
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
                width: Val::Px(300.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: MENU_BUTTON_COLOR.into(),
            border_color: Color::srgba(0.4, 0.4, 0.5, 0.5).into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            ..default()
        },
        button_component,
        Name::new("menu_button"),
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

pub fn handle_menu_button_hover(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, Or<(With<StartGameButton>, With<SettingsButton>, With<ExitGameButton>)>)
    >,
) {
    for (interaction, mut bg_color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = MENU_BUTTON_PRESSED.into();
            }
            Interaction::Hovered => {
                *bg_color = MENU_BUTTON_HOVER.into();
            }
            Interaction::None => {
                *bg_color = MENU_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn handle_menu_buttons(
    start_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    settings_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    exit_query: Query<&Interaction, (Changed<Interaction>, With<ExitGameButton>)>,
    mut change_events: EventWriter<ChangeStateEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in start_query.iter() {
        if *interaction == Interaction::Pressed {
            change_events.send(ChangeStateEvent {
                new_state: GameState::Loading,
            });
        }
    }

    for interaction in settings_query.iter() {
        if *interaction == Interaction::Pressed {
            change_events.send(ChangeStateEvent {
                new_state: GameState::Settings,
            });
        }
    }

    for interaction in exit_query.iter() {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit::Success);
        }
    }
}

pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_loading_screen(
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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.05, 0.05, 0.1, 1.0).into(),
            ..default()
        },
        Name::new("loading_screen"),
    )).with_children(|parent| {
        TextBuilder::localized_child(
            parent,
            "ui.loading",
            TextStylePreset::Custom(48.0, true, Color::WHITE),
            &language_resource,
            &language_packs,
            &text_styles,
        );

        parent.spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
        )).with_children(|subtitle_container| {
            TextBuilder::localized_child(
                subtitle_container,
                "ui.loading_subtitle",
                TextStylePreset::Subtitle,
                &language_resource,
                &language_packs,
                &text_styles,
            );
        });
    });
}

pub fn handle_loading_transition(
    mut timer: Local<Option<Timer>>,
    time: Res<Time>,
    mut change_events: EventWriter<ChangeStateEvent>,
) {
    if timer.is_none() {
        *timer = Some(Timer::from_seconds(2.0, TimerMode::Once));
    }

    if let Some(ref mut loading_timer) = timer.as_mut() {
        loading_timer.tick(time.delta());

        if loading_timer.finished() {
            change_events.send(ChangeStateEvent {
                new_state: GameState::InGame,
            });
            *timer = None;
        }
    }
}
