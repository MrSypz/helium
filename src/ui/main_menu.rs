use bevy::prelude::*;
use crate::core::game_state::{GameState, ChangeStateEvent};
use crate::core::language::manager::{LanguageResource, LanguageChangeEvent, get_text};
use crate::core::language::types::LanguagePack;
use crate::core::language::fonts::FontResource;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct StartGameButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct ExitGameButton;

#[derive(Component)]
pub struct GameTitle;

#[derive(Component)]
pub struct GameSubtitle;

#[derive(Component)]
pub struct ControlsHelp;

// Component สำหรับระบุ button text
#[derive(Component)]
pub struct StartGameButtonText;

#[derive(Component)]
pub struct SettingsButtonText;

#[derive(Component)]
pub struct ExitGameButtonText;

const MENU_BUTTON_COLOR: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);
const MENU_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.9);
const MENU_BUTTON_PRESSED: Color = Color::srgba(0.4, 0.4, 0.5, 0.9);
const MENU_TEXT_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::srgb(1.0, 0.8, 0.2);

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    font_resource: Option<Res<FontResource>>,
) {
    // ใช้ fallback fonts ถ้า FontResource ยังไม่พร้อม
    let (regular_font, bold_font) = if let Some(font_res) = font_resource {
        get_current_fonts(&language_resource, &font_res)
    } else {
        // Fallback fonts - ใช้ default Thai fonts
        let regular = asset_server.load("fonts/NotoSansThai-Regular.ttf");
        let bold = asset_server.load("fonts/NotoSansThai-Bold.ttf");
        (regular, bold)
    };

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
        parent.spawn((
            TextBundle::from_section(
                get_text(&language_resource, &language_packs, "ui.game_title"),
                TextStyle {
                    font: bold_font.clone(),
                    font_size: 64.0,
                    color: TITLE_COLOR,
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(50.0)),
                ..default()
            }),
            GameTitle,
            Name::new("game_title"),
        ));

        // Game Subtitle
        parent.spawn((
            TextBundle::from_section(
                get_text(&language_resource, &language_packs, "ui.game_subtitle"),
                TextStyle {
                    font: regular_font.clone(),
                    font_size: 28.0,
                    color: Color::srgba(0.8, 0.8, 0.9, 0.8),
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            }),
            GameSubtitle,
            Name::new("game_subtitle"),
        ));

        // Start Game Button
        create_menu_button(
            parent,
            &regular_font,
            &get_text(&language_resource, &language_packs, "ui.start_game"),
            StartGameButton,
            StartGameButtonText,
        );

        // Settings Button
        create_menu_button(
            parent,
            &regular_font,
            &get_text(&language_resource, &language_packs, "ui.settings"),
            SettingsButton,
            SettingsButtonText,
        );

        // Exit Game Button
        create_menu_button(
            parent,
            &regular_font,
            &get_text(&language_resource, &language_packs, "ui.exit_game"),
            ExitGameButton,
            ExitGameButtonText,
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
            instructions.spawn((
                TextBundle::from_section(
                    get_text(&language_resource, &language_packs, "ui.controls_help"),
                    TextStyle {
                        font: regular_font.clone(),
                        font_size: 18.0,
                        color: Color::srgba(0.7, 0.7, 0.8, 0.7),
                    },
                ),
                ControlsHelp,
            ));
        });
    });
}

/// ดึง fonts ตามภาษาปัจจุบัน
fn get_current_fonts(
    language_resource: &LanguageResource,
    font_resource: &FontResource,
) -> (Handle<Font>, Handle<Font>) {
    let regular_font = font_resource.get_regular_font(&language_resource.current_language);
    let bold_font = font_resource.get_bold_font(&language_resource.current_language);
    (regular_font, bold_font)
}

/// Safe version ที่รองรับ fallback fonts
fn get_safe_fonts(
    language_resource: &LanguageResource,
    font_resource: Option<&FontResource>,
    asset_server: &AssetServer,
) -> (Handle<Font>, Handle<Font>) {
    if let Some(font_res) = font_resource {
        get_current_fonts(language_resource, font_res)
    } else {
        // Fallback fonts
        let regular = asset_server.load("fonts/NotoSansThai-Regular.ttf");
        let bold = asset_server.load("fonts/NotoSansThai-Bold.ttf");
        (regular, bold)
    }
}

fn create_menu_button<T: Component, U: Component>(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    text: &str,
    button_component: T,
    text_component: U,
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
        button.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 28.0,
                    color: MENU_TEXT_COLOR,
                },
            ),
            text_component,
        ));
    });
}

/// Update menu text และ fonts เมื่อเปลี่ยนภาษา
pub fn update_main_menu_language(
    mut language_events: EventReader<LanguageChangeEvent>,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    font_resource: Option<Res<FontResource>>,
    asset_server: Res<AssetServer>,
    mut text_query: Query<&mut Text>,
    title_query: Query<Entity, With<GameTitle>>,
    subtitle_query: Query<Entity, With<GameSubtitle>>,
    controls_query: Query<Entity, With<ControlsHelp>>,
    start_button_text_query: Query<Entity, With<StartGameButtonText>>,
    settings_button_text_query: Query<Entity, With<SettingsButtonText>>,
    exit_button_text_query: Query<Entity, With<ExitGameButtonText>>,
) {
    for _event in language_events.read() {
        // ดึง fonts ใหม่ตามภาษาปัจจุบัน (safe version)
        let (regular_font, bold_font) = get_safe_fonts(
            &language_resource,
            font_resource.as_deref(),
            &asset_server
        );

        // Update title
        if let Ok(entity) = title_query.get_single() {
            if let Ok(mut text) = text_query.get_mut(entity) {
                text.sections[0].value = get_text(&language_resource, &language_packs, "ui.game_title");
                text.sections[0].style.font = bold_font.clone();
            }
        }

        // Update subtitle
        if let Ok(entity) = subtitle_query.get_single() {
            if let Ok(mut text) = text_query.get_mut(entity) {
                text.sections[0].value = get_text(&language_resource, &language_packs, "ui.game_subtitle");
                text.sections[0].style.font = regular_font.clone();
            }
        }

        // Update controls help
        if let Ok(entity) = controls_query.get_single() {
            if let Ok(mut text) = text_query.get_mut(entity) {
                text.sections[0].value = get_text(&language_resource, &language_packs, "ui.controls_help");
                text.sections[0].style.font = regular_font.clone();
            }
        }

        // Update start button text
        if let Ok(entity) = start_button_text_query.get_single() {
            if let Ok(mut text) = text_query.get_mut(entity) {
                text.sections[0].value = get_text(&language_resource, &language_packs, "ui.start_game");
                text.sections[0].style.font = regular_font.clone();
            }
        }

        // Update settings button text
        if let Ok(entity) = settings_button_text_query.get_single() {
            if let Ok(mut text) = text_query.get_mut(entity) {
                text.sections[0].value = get_text(&language_resource, &language_packs, "ui.settings");
                text.sections[0].style.font = regular_font.clone();
            }
        }

        // Update exit button text
        if let Ok(entity) = exit_button_text_query.get_single() {
            if let Ok(mut text) = text_query.get_mut(entity) {
                text.sections[0].value = get_text(&language_resource, &language_packs, "ui.exit_game");
                text.sections[0].style.font = regular_font.clone();
            }
        }
    }
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
    asset_server: Res<AssetServer>,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    font_resource: Option<Res<FontResource>>,
) {
    let (regular_font, bold_font) = get_safe_fonts(
        &language_resource,
        font_resource.as_deref(),
        &asset_server
    );

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
        parent.spawn(TextBundle::from_section(
            get_text(&language_resource, &language_packs, "ui.loading"),
            TextStyle {
                font: bold_font.clone(),
                font_size: 48.0,
                color: Color::WHITE,
            },
        ));

        parent.spawn((
            TextBundle::from_section(
                get_text(&language_resource, &language_packs, "ui.loading_subtitle"),
                TextStyle {
                    font: regular_font,
                    font_size: 24.0,
                    color: Color::srgba(0.8, 0.8, 0.8, 0.8),
                },
            ).with_style(Style {
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            }),
        ));
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