// src/common/plugin.rs
use bevy::prelude::*;
use crate::common::helium::{VNState, DialogResource, DialogHistory, DialogManager, StageChangeEvent, DialogResetEvent};
use crate::common::game_state::{GameState, ChangeStateEvent, PreviousState, handle_state_changes, handle_pause_input};
use crate::common::dialog::types::DialogScene;
use crate::common::dialog::typewriter::typewriter_system;
use crate::common::dialog::choice::{ChoiceState, handle_choice_selection, debug_choice_system};
use crate::client::render::ui::dialog::{setup_ui, manage_dialog_state, handle_text_interaction, handle_language_toggle};
use crate::client::render::ui::choice::{manage_choice_display, highlight_choice_button};
use crate::client::render::ui::main_menu::{
    setup_main_menu, handle_menu_button_hover, handle_menu_buttons, cleanup_main_menu,
    setup_loading_screen, handle_loading_transition
};
use crate::client::render::setup::{setup_scene, update_background};
use crate::client::render::character::{setup_characters, update_characters, check_character_assets, debug_characters};
use crate::common::dialog::init::load_dialogs;

pub struct VNPlugin;

impl Plugin for VNPlugin {
    fn build(&self, app: &mut App) {
        app
            // State Management
            .init_state::<GameState>()

            // Asset และ Loader registration
            .init_asset::<DialogScene>()
            .init_asset_loader::<crate::common::dialog::types::DialogLoader>()

            // Resource initialization
            .init_resource::<VNState>()
            .init_resource::<DialogResource>()
            .init_resource::<DialogHistory>()
            .init_resource::<ChoiceState>()
            .init_resource::<DialogManager>()
            .init_resource::<PreviousState>()

            // Event registration
            .add_event::<StageChangeEvent>()
            .add_event::<DialogResetEvent>()
            .add_event::<ChangeStateEvent>()

            // Global systems (ทำงานทุก state)
            .add_systems(Update, (
                handle_state_changes,
                handle_pause_input,
            ))

            // === MAIN MENU STATE ===
            .add_systems(OnEnter(GameState::MainMenu), (
                setup_main_menu,
            ))
            .add_systems(Update, (
                handle_menu_button_hover,
                handle_menu_buttons,
            ).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), (
                cleanup_main_menu,
            ))

            // === LOADING STATE ===
            .add_systems(OnEnter(GameState::Loading), (
                setup_loading_screen,
            ))
            .add_systems(Update, (
                handle_loading_transition,
            ).run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), (
                cleanup_loading_screen,
            ))

            // === IN-GAME STATE ===
            .add_systems(OnEnter(GameState::InGame), (
                setup_game_scene,
                setup_ui,
                load_dialogs,
            ))
            .add_systems(Update, (
                // === Phase 1: Asset Loading & Setup ===
                setup_characters,
                check_character_assets,

                // === Phase 2: Core Dialog Management ===
                manage_dialog_state
                    .after(setup_characters)
                    .before(handle_text_interaction)
                    .before(manage_choice_display),

                // Typewriter effect
                typewriter_system
                    .after(manage_dialog_state),

                // === Phase 3: User Input Handling ===
                handle_text_interaction
                    .after(manage_dialog_state)
                    .before(handle_choice_selection),

                handle_language_toggle
                    .after(manage_dialog_state),

                // === Phase 4: Choice Management ===
                manage_choice_display
                    .after(manage_dialog_state)
                    .after(typewriter_system),

                highlight_choice_button
                    .after(manage_choice_display),

                handle_choice_selection
                    .after(manage_choice_display)
                    .after(highlight_choice_button),

                // === Phase 5: Visual Updates ===
                update_characters
                    .after(manage_dialog_state)
                    .after(setup_characters),

                update_background
                    .after(manage_dialog_state),

                // === Phase 6: Debug Systems ===
                debug_choice_system,
                debug_characters,
            ).run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), (
                cleanup_game_scene,
            ));
    }
}

/// ระบบสำหรับตั้งค่าฉากเกม (แยกจาก setup_scene เดิม)
fn setup_game_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dialog_scenes: Res<Assets<DialogScene>>,
    state: Res<VNState>,
) {
    info!("ตั้งค่าฉากเกม");

    // กล้อง 2D (ถ้ายังไม่มี)
    if commands.get_entity(Entity::PLACEHOLDER).is_none() {
        commands.spawn(Camera2dBundle::default());
    }

    // เรียกใช้ระบบ setup_scene เดิม
    setup_scene(commands, asset_server, dialog_scenes, state);
}

/// ลบ loading screen
fn cleanup_loading_screen(
    mut commands: Commands,
    loading_query: Query<Entity, (With<Node>, Without<Camera>)>,
) {
    for entity in loading_query.iter() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
    info!("ลบ loading screen แล้ว");
}

/// ลบฉากเกม
fn cleanup_game_scene(
    mut commands: Commands,
    game_entities: Query<Entity, (Without<Camera>, Without<Window>)>,
) {
    for entity in game_entities.iter() {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
    info!("ลบฉากเกมแล้ว");
}

/// ระบบเสริมสำหรับจัดการ events
pub fn handle_stage_change_events(
    mut events: EventReader<StageChangeEvent>,
    mut state: ResMut<VNState>,
    mut dialog_resource: ResMut<DialogResource>,
) {
    for event in events.read() {
        if let Some(scene_name) = &event.scene_name {
            dialog_resource.change_scene(scene_name, &mut state);
        } else {
            state.change_stage(event.new_stage);
        }
    }
}

/// ระบบเสริมสำหรับจัดการ dialog reset events
pub fn handle_dialog_reset_events(
    mut events: EventReader<DialogResetEvent>,
    mut state: ResMut<VNState>,
) {
    for _event in events.read() {
        state.dialog_needs_reset = true;
    }
}