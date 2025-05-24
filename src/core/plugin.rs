use bevy::prelude::*;
use crate::core::resources::*;
use crate::core::game_state::{GameState, ChangeStateEvent, PreviousState, handle_state_changes, handle_pause_input};
use crate::core::dialog::{
    manager::{load_dialogs, manage_dialog_state},
    choice::{ChoiceState, handle_choice_selection as core_handle_choice_selection},
};
use crate::core::scene::{
    background::{setup_scene_background, update_background},
    character::{setup_characters, update_characters, check_character_assets},
};
use crate::core::language::{
    manager::{load_language_packs, check_language_loading},
    types::{LanguagePack, LanguageLoader},
    sync::{sync_language_with_vn_state, sync_vn_state_with_language},
};
use crate::core::language::manager::{LanguageChangeEvent, LanguageResource};
use crate::core::text::{
    styles::{TextStyleResource, ensure_text_styles_initialized},
    builder::update_localized_text,
};
use crate::types::{DialogScene, DialogLoader};
use crate::ui::{
    dialog::{setup_dialog_ui, handle_text_interaction, update_dialog_fonts, paused_typewriter_system},
    choice::{manage_choice_display, highlight_choice_button},
    main_menu::{setup_main_menu, handle_menu_button_hover, handle_menu_buttons, cleanup_main_menu, setup_loading_screen, handle_loading_transition},
    settings::{setup_settings_ui, handle_settings_button_hover, handle_settings_buttons, update_settings_values, cleanup_settings, ResolutionDropdownState},
    pause::{setup_pause_ui, handle_pause_button_hover, handle_pause_buttons, cleanup_pause_ui},
};

pub struct VNPlugin;

impl Plugin for VNPlugin {
    fn build(&self, app: &mut App) {
        app
            // Core systems
            .init_state::<GameState>()
            .init_asset::<DialogScene>()
            .init_asset::<LanguagePack>()
            .init_asset_loader::<DialogLoader>()
            .init_asset_loader::<LanguageLoader>()

            // Resources
            .init_resource::<VNState>()
            .init_resource::<DialogResource>()
            .init_resource::<DialogHistory>()
            .init_resource::<ChoiceState>()
            .init_resource::<DialogManager>()
            .init_resource::<PreviousState>()
            .init_resource::<LanguageResource>()
            .init_resource::<SettingsResource>()
            .init_resource::<ResolutionDropdownState>()
            .init_resource::<TextStyleResource>()

            // Events
            .add_event::<StageChangeEvent>()
            .add_event::<DialogResetEvent>()
            .add_event::<ChangeStateEvent>()
            .add_event::<LanguageChangeEvent>()
            .add_event::<SettingsChangeEvent>()

            // Startup systems
            .add_systems(Startup, (
                setup_global_camera,
                load_language_packs,
            ))

            // Global systems
            .add_systems(Update, (
                handle_state_changes,
                handle_pause_input,
                check_language_loading,
                sync_language_with_vn_state,
                sync_vn_state_with_language,
                update_localized_text,
                ensure_text_styles_initialized,
                // เพิ่ม conditional cleanup
                conditional_cleanup_game_scene,
            ))

            // Main Menu
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(Update, (
                handle_menu_button_hover,
                handle_menu_buttons,
            ).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)

            // Settings
            .add_systems(OnEnter(GameState::Settings), setup_settings_ui)
            .add_systems(Update, (
                handle_settings_button_hover,
                handle_settings_buttons,
                update_settings_values,
            ).run_if(in_state(GameState::Settings)))
            .add_systems(OnExit(GameState::Settings), cleanup_settings)

            // Pause - render ทับ dialog โดยไม่ cleanup
            .add_systems(OnEnter(GameState::Paused), setup_pause_ui)
            .add_systems(Update, (
                handle_pause_button_hover,
                handle_pause_buttons,
            ).run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), cleanup_pause_ui)

            // Loading
            .add_systems(OnEnter(GameState::Loading), setup_loading_screen)
            .add_systems(Update, handle_loading_transition.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup_loading_screen)

            // In-Game - ลบ OnExit cleanup ออก
            .add_systems(OnEnter(GameState::InGame), (
                setup_dialog_ui_if_needed,
                load_dialogs_if_needed,
                setup_scene_background_if_needed,
            ))
            .add_systems(Update, (
                // Core dialog management
                manage_dialog_state
                    .before(handle_text_interaction)
                    .before(manage_choice_display),

                paused_typewriter_system.after(manage_dialog_state),

                // User interactions
                handle_text_interaction.after(manage_dialog_state),
                update_dialog_fonts.after(manage_dialog_state),

                // Choice system
                manage_choice_display.after(manage_dialog_state),
                highlight_choice_button.after(manage_choice_display),
                core_handle_choice_selection.after(manage_choice_display),

                // Scene rendering
                setup_characters.after(manage_dialog_state),
                update_characters.after(setup_characters),
                update_background.after(manage_dialog_state),
                check_character_assets,
            ).run_if(in_state(GameState::InGame)));
        // ไม่มี OnExit(GameState::InGame) cleanup
    }
}

fn setup_global_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        Name::new("main_camera"),
    ));
}

fn cleanup_loading_screen(
    mut commands: Commands,
    loading_query: Query<Entity, (With<Node>, Without<Camera>)>,
) {
    for entity in loading_query.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
}

// ระบบ cleanup ที่ปลอดภัยและตรวจสอบว่า entity มีอยู่จริง
fn conditional_cleanup_game_scene(
    mut commands: Commands,
    mut change_events: EventReader<ChangeStateEvent>,
    current_state: Res<State<GameState>>,
    game_entities: Query<Entity, (
        Without<Camera>,
        Without<Window>,
        Or<(
            With<crate::ui::dialog::DialogBox>,
            With<crate::core::scene::background::Background>,
            With<crate::core::scene::character::CharacterSprite>,
            With<crate::ui::choice::ChoiceContainer>,
            With<crate::ui::choice::ChoiceOverlay>,
        )>
    )>,
    mut cleanup_happened: Local<bool>,
) {
    for event in change_events.read() {
        let should_cleanup = match (current_state.get(), &event.new_state) {
            (GameState::InGame, GameState::MainMenu) => true,
            (GameState::InGame, GameState::Loading) => true,
            (GameState::Paused, GameState::MainMenu) => true,
            _ => false,
        };

        if should_cleanup && !*cleanup_happened {
            *cleanup_happened = true;

            let mut cleaned_count = 0;

            for entity in game_entities.iter() {
                if let Some(entity_commands) = commands.get_entity(entity) {
                    entity_commands.despawn_recursive();
                    cleaned_count += 1;
                }
            }

            info!("Safely cleaned up {} game entities when transitioning from {:?} to {:?}",
                  cleaned_count, current_state.get(), event.new_state);
        }

        if event.new_state == GameState::InGame {
            *cleanup_happened = false;
        }
    }
}

fn setup_dialog_ui_if_needed(
    commands: Commands,
    language_resource: Res<LanguageResource>,
    language_packs: Res<Assets<LanguagePack>>,
    text_styles: Res<TextStyleResource>,
    existing_dialog: Query<Entity, With<crate::ui::dialog::DialogBox>>,
) {
    // Setup เฉพาะเมื่อยังไม่มี dialog UI
    if existing_dialog.is_empty() {
        setup_dialog_ui(commands, language_resource, language_packs, text_styles);
    }
}

fn load_dialogs_if_needed(
    asset_server: Res<AssetServer>,
    dialog_resource: ResMut<DialogResource>,
    vn_state: ResMut<VNState>,
) {
    if dialog_resource.scenes.is_empty() {
        load_dialogs(asset_server, dialog_resource, vn_state);
    }
}

fn setup_scene_background_if_needed(
    commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
    existing_bg: Query<Entity, With<crate::core::scene::background::Background>>,
) {
    if existing_bg.is_empty() {
        setup_scene_background(commands, asset_server, dialog_scenes, state);
    }
}