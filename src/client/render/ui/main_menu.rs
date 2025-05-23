// src/client/render/ui/main_menu.rs
use bevy::prelude::*;
use crate::common::game_state::{GameState, ChangeStateEvent};

/// Component สำหรับเมนูหลัก
#[derive(Component)]
pub struct MainMenuUI;

/// Component สำหรับปุ่มเริ่มเกม
#[derive(Component)]
pub struct StartGameButton;

/// Component สำหรับปุ่มตั้งค่า
#[derive(Component)]
pub struct SettingsButton;

/// Component สำหรับปุ่มออกจากเกม
#[derive(Component)]
pub struct ExitGameButton;

/// Component สำหรับชื่อเกม
#[derive(Component)]
pub struct GameTitle;

// Constants สำหรับ UI
const MENU_BUTTON_COLOR: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);
const MENU_BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.9);
const MENU_BUTTON_PRESSED: Color = Color::srgba(0.4, 0.4, 0.5, 0.9);
const MENU_TEXT_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::srgb(1.0, 0.8, 0.2);

/// ระบบสำหรับสร้างเมนูหลัก
pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    info!("สร้างเมนูหลัก");
    commands.spawn(Camera2dBundle::default());

    // โหลดฟอนต์
    let title_font = asset_server.load("fonts/NotoSansThai-Bold.ttf");
    let button_font = asset_server.load("fonts/NotoSansThai-Regular.ttf");

    // สร้าง background gradient
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

    // Container หลักของเมนู
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
        // ชื่อเกม
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

        // คำบรรยายใต้ชื่อ
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

        // ปุ่มเริ่มเกม
        create_menu_button(parent, &button_font, "เริ่มเกม", "Start Game", StartGameButton);

        // ปุ่มตั้งค่า
        create_menu_button(parent, &button_font, "ตั้งค่า", "Settings", SettingsButton);

        // ปุ่มออกจากเกม
        create_menu_button(parent, &button_font, "ออกจากเกม", "Exit Game", ExitGameButton);

        // คำแนะนำการใช้งาน
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

/// สร้างปุ่มเมนู
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

/// ระบบจัดการ hover effect ของปุ่ม
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

/// ระบบจัดการการคลิกปุ่มเมนู
pub fn handle_menu_buttons(
    start_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    settings_query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
    exit_query: Query<&Interaction, (Changed<Interaction>, With<ExitGameButton>)>,
    mut change_events: EventWriter<ChangeStateEvent>,
    mut exit: EventWriter<AppExit>,
) {
    // ปุ่มเริ่มเกม
    for interaction in start_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("กดปุ่มเริ่มเกม");
            change_events.send(ChangeStateEvent {
                new_state: GameState::Loading,
            });
        }
    }

    // ปุ่มตั้งค่า
    for interaction in settings_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("กดปุ่มตั้งค่า");
            change_events.send(ChangeStateEvent {
                new_state: GameState::Settings,
            });
        }
    }

    // ปุ่มออกจากเกม
    for interaction in exit_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("กดปุ่มออกจากเกม");
            exit.send(AppExit::Success);
        }
    }
}

/// ระบบลบเมนูหลักเมื่อออกจากสถานะ MainMenu
pub fn cleanup_main_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    info!("ลบเมนูหลักแล้ว");
}

/// ระบบแสดง loading screen
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

/// ระบบจำลองการโหลดและเปลี่ยนไปเกม
pub fn handle_loading_transition(
    mut timer: Local<Option<Timer>>,
    time: Res<Time>,
    mut change_events: EventWriter<ChangeStateEvent>,
) {
    // สร้าง timer ครั้งแรกที่เข้า loading state
    if timer.is_none() {
        *timer = Some(Timer::from_seconds(2.0, TimerMode::Once));
        info!("เริ่มการโหลด");
    }

    if let Some(ref mut loading_timer) = timer.as_mut() {
        loading_timer.tick(time.delta());

        if loading_timer.finished() {
            info!("โหลดเสร็จแล้ว เริ่มเกม");
            change_events.send(ChangeStateEvent {
                new_state: GameState::InGame,
            });
            *timer = None; // รีเซ็ต timer สำหรับครั้งถัดไป
        }
    }
}