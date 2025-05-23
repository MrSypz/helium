// src/ui/settings.rs (แบบง่าย)
use bevy::prelude::*;
use bevy::window::{WindowMode, PrimaryWindow};
use crate::core::game_state::{GameState, ChangeStateEvent};
use crate::core::language::manager::{LanguageResource, LanguageChangeEvent, get_text};
use crate::core::language::types::{LanguagePack, LanguageCode};
use crate::core::resources::{SettingsResource, SettingsChangeEvent};

#[derive(Component)]
pub struct SettingsUI;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct LanguageButton;

#[derive(Component)]
pub struct ResolutionButton;

#[derive(Component)]
pub struct FullscreenButton;

#[derive(Component)]
pub struct ApplyButton;

#[derive(Component)]
pub struct SettingsTitle;

#[derive(Component)]
pub struct CurrentLanguageText;

#[derive(Component)]
pub struct CurrentResolutionText;

#[derive(Component)]
pub struct CurrentFullscreenText;

#[derive(Resource, Default)]
pub struct ResolutionDropdownState {
    pub is_open: bool,
}

const SETTINGS_BG_COLOR: Color = Color::srgba(0.08, 0.08, 0.15, 1.0);
const PANEL_BG_COLOR: Color = Color::srgba(0.15, 0.15, 0.25, 0.95);
const BUTTON_COLOR: Color = Color::srgba(0.2, 0.2, 0.3, 0.9);
const BUTTON_HOVER: Color = Color::srgba(0.3, 0.3, 0.4, 0.9);
const BUTTON_PRESSED: Color = Color::srgba(0.4, 0.4, 0.5, 0.9);
const TEXT_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::srgb(1.0, 0.8, 0.2);

const RESOLUTIONS: [(f32, f32, &str); 4] = [
    (1280.0, 720.0, "HD"),
    (1600.0, 900.0, "HD+"),
    (1920.0, 1080.0, "Full HD"),
    (2560.0, 1440.0, "2K")
];

pub fn setup_settings_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    settings: Res<SettingsResource>,
) {
    let font = asset_server.load("fonts/thai/NotoSansThai-Regular.ttf");
    let bold_font = asset_server.load("fonts/thai/NotoSansThai-Bold.ttf");

    // Background
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: SETTINGS_BG_COLOR.into(),
            ..default()
        },
        SettingsUI,
    ));

    // Main panel
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(500.0),
                height: Val::Auto,
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                margin: UiRect::new(Val::Px(-250.0), Val::Auto, Val::Px(-200.0), Val::Auto),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(30.0)),
                row_gap: Val::Px(25.0),
                ..default()
            },
            background_color: PANEL_BG_COLOR.into(),
            border_radius: BorderRadius::all(Val::Px(15.0)),
            ..default()
        },
        SettingsUI,
    )).with_children(|parent| {
        // Title
        parent.spawn((
            TextBundle::from_section(
                get_text(&language_resource, &language_packs, "ui.settings_title"),
                TextStyle {
                    font: bold_font.clone(),
                    font_size: 36.0,
                    color: TITLE_COLOR,
                },
            ),
            SettingsTitle,
        ));

        // Language Setting
        create_setting_row(parent, &font, &bold_font, &language_resource, &language_packs,
                           "ui.language_setting", &format_language(&language_resource.current_language), LanguageButton, CurrentLanguageText);

        // Resolution Setting  
        let res_text = format!("{}x{}", settings.resolution.0 as u32, settings.resolution.1 as u32);
        create_setting_row(parent, &font, &bold_font, &language_resource, &language_packs,
                           "ui.resolution_setting", &res_text, ResolutionButton, CurrentResolutionText);

        // Fullscreen Setting
        let fullscreen_text = if settings.fullscreen {
            get_text(&language_resource, &language_packs, "ui.enabled")
        } else {
            get_text(&language_resource, &language_packs, "ui.disabled")
        };
        create_setting_row(parent, &font, &bold_font, &language_resource, &language_packs,
                           "ui.fullscreen_setting", &fullscreen_text, FullscreenButton, CurrentFullscreenText);

        // Buttons
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            ..default()
        }).with_children(|buttons| {
            create_button(buttons, &font, &get_text(&language_resource, &language_packs, "ui.back"), BackButton);
            create_button(buttons, &font, &get_text(&language_resource, &language_packs, "ui.apply_settings"), ApplyButton);
        });
    });
}

fn create_setting_row<B: Component, T: Component>(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    bold_font: &Handle<Font>,
    language_resource: &LanguageResource,
    language_packs: &Assets<LanguagePack>,
    label_key: &str,
    current_value: &str,
    button_component: B,
    text_component: T,
) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(15.0)),
            ..default()
        },
        background_color: Color::srgba(0.1, 0.1, 0.2, 0.5).into(),
        border_radius: BorderRadius::all(Val::Px(8.0)),
        ..default()
    }).with_children(|row| {
        // Label
        row.spawn(TextBundle::from_section(
            get_text(language_resource, language_packs, label_key),
            TextStyle {
                font: bold_font.clone(),
                font_size: 20.0,
                color: TEXT_COLOR,
            },
        ));

        // Value + Button
        row.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        }).with_children(|value_section| {
            // Current value
            value_section.spawn((
                TextBundle::from_section(
                    current_value,
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: Color::srgba(0.8, 0.8, 0.9, 1.0),
                    },
                ),
                text_component,
            ));

            // Change button
            value_section.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(80.0),
                        height: Val::Px(35.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    ..default()
                },
                button_component,
            )).with_children(|button| {
                button.spawn(TextBundle::from_section(
                    get_text(language_resource, language_packs, "ui.change"),
                    TextStyle {
                        font: font.clone(),
                        font_size: 14.0,
                        color: TEXT_COLOR,
                    },
                ));
            });
        });
    });
}

fn create_button<T: Component>(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    text: &str,
    component: T,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(45.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BUTTON_COLOR.into(),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },
        component,
    )).with_children(|button| {
        button.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: 18.0,
                color: TEXT_COLOR,
            },
        ));
    });
}

fn format_language(language: &LanguageCode) -> String {
    match language {
        LanguageCode::Thai => "ไทย".to_string(),
        LanguageCode::English => "English".to_string(),
        LanguageCode::Japanese => "日本語".to_string(),
    }
}

fn get_next_resolution(current: (f32, f32)) -> (f32, f32) {
    let current_index = RESOLUTIONS.iter()
        .position(|(w, h, _)| *w == current.0 && *h == current.1)
        .unwrap_or(0);
    let next_index = (current_index + 1) % RESOLUTIONS.len();
    (RESOLUTIONS[next_index].0, RESOLUTIONS[next_index].1)
}

pub fn handle_settings_button_hover(
    mut query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut bg_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => *bg_color = BUTTON_PRESSED.into(),
            Interaction::Hovered => *bg_color = BUTTON_HOVER.into(),
            Interaction::None => *bg_color = BUTTON_COLOR.into(),
        }
    }
}

pub fn handle_settings_buttons(
    back_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    apply_query: Query<&Interaction, (Changed<Interaction>, With<ApplyButton>)>,
    language_query: Query<&Interaction, (Changed<Interaction>, With<LanguageButton>)>,
    resolution_query: Query<&Interaction, (Changed<Interaction>, With<ResolutionButton>)>,
    fullscreen_query: Query<&Interaction, (Changed<Interaction>, With<FullscreenButton>)>,
    mut settings: ResMut<SettingsResource>,
    mut language_resource: ResMut<LanguageResource>,
    mut language_events: EventWriter<LanguageChangeEvent>,
    mut settings_events: EventWriter<SettingsChangeEvent>,
    mut change_events: EventWriter<ChangeStateEvent>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Back button
    for interaction in back_query.iter() {
        if *interaction == Interaction::Pressed {
            change_events.send(ChangeStateEvent {
                new_state: GameState::MainMenu,
            });
        }
    }

    // Apply button
    for interaction in apply_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Ok(mut window) = window_query.get_single_mut() {
                window.resolution.set(settings.resolution.0, settings.resolution.1);
                window.mode = if settings.fullscreen {
                    WindowMode::BorderlessFullscreen
                } else {
                    WindowMode::Windowed
                };
            }
        }
    }

    // Language button
    for interaction in language_query.iter() {
        if *interaction == Interaction::Pressed {
            let new_language = language_resource.next_language();
            if language_resource.change_language(new_language.clone()) {
                language_events.send(LanguageChangeEvent { new_language: new_language.clone() });
                settings.language = new_language;
                settings_events.send(SettingsChangeEvent);
            }
        }
    }

    // Resolution button
    for interaction in resolution_query.iter() {
        if *interaction == Interaction::Pressed {
            settings.resolution = get_next_resolution(settings.resolution);
            settings_events.send(SettingsChangeEvent);
        }
    }

    // Fullscreen button
    for interaction in fullscreen_query.iter() {
        if *interaction == Interaction::Pressed {
            settings.fullscreen = !settings.fullscreen;
            settings_events.send(SettingsChangeEvent);
        }
    }
}

pub fn update_settings_ui(
    mut language_events: EventReader<LanguageChangeEvent>,
    mut settings_events: EventReader<SettingsChangeEvent>,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    settings: Res<SettingsResource>,
    mut title_query: Query<&mut Text, (With<SettingsTitle>, Without<CurrentLanguageText>, Without<CurrentResolutionText>, Without<CurrentFullscreenText>)>,
    mut language_text_query: Query<&mut Text, (With<CurrentLanguageText>, Without<SettingsTitle>, Without<CurrentResolutionText>, Without<CurrentFullscreenText>)>,
    mut resolution_text_query: Query<&mut Text, (With<CurrentResolutionText>, Without<SettingsTitle>, Without<CurrentLanguageText>, Without<CurrentFullscreenText>)>,
    mut fullscreen_text_query: Query<&mut Text, (With<CurrentFullscreenText>, Without<SettingsTitle>, Without<CurrentLanguageText>, Without<CurrentResolutionText>)>,
) {
    let mut should_update = false;

    for _event in language_events.read() {
        should_update = true;
    }

    for _event in settings_events.read() {
        should_update = true;
    }

    if should_update {
        // Update title
        if let Ok(mut text) = title_query.get_single_mut() {
            text.sections[0].value = get_text(&language_resource, &language_packs, "ui.settings_title");
        }

        // Update language text
        if let Ok(mut text) = language_text_query.get_single_mut() {
            text.sections[0].value = format_language(&language_resource.current_language);
        }

        // Update resolution text
        if let Ok(mut text) = resolution_text_query.get_single_mut() {
            text.sections[0].value = format!("{}x{}", settings.resolution.0 as u32, settings.resolution.1 as u32);
        }

        // Update fullscreen text
        if let Ok(mut text) = fullscreen_text_query.get_single_mut() {
            text.sections[0].value = if settings.fullscreen {
                get_text(&language_resource, &language_packs, "ui.enabled")
            } else {
                get_text(&language_resource, &language_packs, "ui.disabled")
            };
        }
    }
}

pub fn cleanup_settings(
    mut commands: Commands,
    settings_query: Query<Entity, With<SettingsUI>>,
) {
    for entity in settings_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}