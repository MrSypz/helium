use bevy::prelude::*;
use crate::core::game_state::{GameState, ChangeStateEvent};

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

const MENU_BUTTON_COLOR: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);
const MENU_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.9);
const MENU_BUTTON_PRESSED: Color = Color::srgba(0.4, 0.4, 0.5, 0.9);
const MENU_TEXT_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::srgb(1.0, 0.8, 0.2);

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    let title_font = asset_server.load("fonts/NotoSansThai-Bold.ttf");
    let button_font = asset_server.load("fonts/NotoSansThai-Regular.ttf");

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
        parent.spawn((
            TextBundle::from_section(
                "Helium Visual Novel",
                TextStyle {
                    font: title_font.clone(),
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

        parent.spawn((
            TextBundle::from_section(
                "เริ่มต้นการผจญภัยครั้งใหม่ของคุณ",
                TextStyle {
                    font: button_font.clone(),
                    font_size: 28.0,
                    color: Color::srgba(0.8, 0.8, 0.9, 0.8),
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            }),
            Name::new("game_subtitle"),
        ));

        create_menu_button(parent, &button_font, "เริ่มเกม", "Start Game", StartGameButton);
        create_menu_button(parent, &button_font, "ตั้งค่า", "Settings", SettingsButton);
        create_menu_button(parent, &button_font, "ออกจากเกม", "Exit Game", ExitGameButton);

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
            instructions.spawn(TextBundle::from_section(
                "การควบคุม: คลิกเพื่อดำเนินเนื้อเรื่อง | L - เปลี่ยนภาษา | ESC - หยุดเกมชั่วคราว",
                TextStyle {
                    font: button_font.clone(),
                    font_size: 18.0,
                    color: Color::srgba(0.7, 0.7, 0.8, 0.7),
                },
            ));
        });
    });
}

fn create_menu_button<T: Component>(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    thai_text: &str,
    english_text: &str,
    component: T,
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
        component,
        Name::new(format!("button_{}", english_text.to_lowercase().replace(" ", "_"))),
    )).with_children(|button| {
        button.spawn(TextBundle::from_section(
            thai_text,
            TextStyle {
                font: font.clone(),
                font_size: 28.0,
                color: MENU_TEXT_COLOR,
            },
        ));
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
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/NotoSansThai-Regular.ttf");

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
            "กำลังโหลด...",
            TextStyle {
                font: font.clone(),
                font_size: 48.0,
                color: Color::WHITE,
            },
        ));

        parent.spawn((
            TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: font,
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