use bevy::prelude::*;
use crate::core::resources::*;
use crate::core::game_state::{GameState, ChangeStateEvent, PreviousState, handle_state_changes, handle_pause_input};
use crate::core::dialog::{manager::*, typewriter::*, choice::*};
use crate::core::scene::{background::*, character::*};
use crate::core::language::{manager::*, types::*, sync::*};
use crate::core::text::{styles::*, components::*, builder::*};
use crate::types::{DialogScene, DialogLoader};
use crate::ui::{dialog::*, choice::*, main_menu::*, settings::*};

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

            // Resources - เพิ่ม TextStyleResource
            .init_resource::<VNState>()
            .init_resource::<DialogResource>()
            .init_resource::<DialogHistory>()
            .init_resource::<ChoiceState>()
            .init_resource::<DialogManager>()
            .init_resource::<PreviousState>()
            .init_resource::<LanguageResource>()
            .init_resource::<SettingsResource>()
            .init_resource::<ResolutionDropdownState>()
            .init_resource::<TextStyleResource>() // เพิ่มบรรทัดนี้

            // Events
            .add_event::<StageChangeEvent>()
            .add_event::<DialogResetEvent>()
            .add_event::<ChangeStateEvent>()
            .add_event::<LanguageChangeEvent>()
            .add_event::<SettingsChangeEvent>()

            // Startup: Setup camera และ language
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
                ensure_text_styles_initialized, // เพิ่มระบบ lazy initialization
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

            // Loading
            .add_systems(OnEnter(GameState::Loading), setup_loading_screen)
            .add_systems(Update, handle_loading_transition.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup_loading_screen)

            // In-Game
            .add_systems(OnEnter(GameState::InGame), (
                setup_dialog_ui,
                load_dialogs,
                setup_scene_background,
            ))
            .add_systems(Update, (
                // Core dialog management
                manage_dialog_state
                    .before(handle_text_interaction)
                    .before(manage_choice_display),

                typewriter_system.after(manage_dialog_state),

                // User interactions
                handle_text_interaction.after(manage_dialog_state),
                update_dialog_fonts.after(manage_dialog_state),

                // Choice system
                manage_choice_display.after(manage_dialog_state),
                highlight_choice_button.after(manage_choice_display),
                handle_choice_selection.after(manage_choice_display),

                // Scene rendering
                setup_characters.after(manage_dialog_state),
                update_characters.after(setup_characters),
                update_background.after(manage_dialog_state),
                check_character_assets,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), cleanup_game_scene);
    }
}

/// สร้าง camera เพียงครั้งเดียวตอนเริ่มต้นเกม
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

fn cleanup_game_scene(
    mut commands: Commands,
    game_entities: Query<Entity, (Without<Camera>, Without<Window>)>,
) {
    for entity in game_entities.iter() {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
}