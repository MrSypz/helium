// src/common/plugin.rs
use bevy::prelude::*;
use crate::common::helium::{VNState, DialogResource, DialogHistory, DialogManager, StageChangeEvent, DialogResetEvent};
use crate::common::dialog::types::DialogScene;
use crate::common::dialog::typewriter::typewriter_system;
use crate::common::dialog::choice::{ChoiceState, handle_choice_selection, debug_choice_system};
use crate::client::render::ui::dialog::{setup_ui, manage_dialog_state, handle_text_interaction, handle_language_toggle};
use crate::client::render::ui::choice::{manage_choice_display, highlight_choice_button};
use crate::client::render::setup::{setup_scene, update_background};
use crate::client::render::character::{setup_characters, update_characters, check_character_assets, debug_characters};
use crate::client::render::transition::{ActiveTransition, start_transition, update_transition};
use crate::common::dialog::init::load_dialogs;

pub struct VNPlugin;

impl Plugin for VNPlugin {
    fn build(&self, app: &mut App) {
        app
            // Asset และ Loader registration
            .init_asset::<DialogScene>()
            .init_asset_loader::<crate::common::dialog::types::DialogLoader>()

            // Resource initialization
            .init_resource::<VNState>()
            .init_resource::<DialogResource>()
            .init_resource::<DialogHistory>()
            .init_resource::<ChoiceState>()
            .init_resource::<ActiveTransition>()
            .init_resource::<DialogManager>() // ใหม่

            // Event registration
            .add_event::<StageChangeEvent>() // ใหม่
            .add_event::<DialogResetEvent>() // ใหม่

            // Startup systems
            .add_systems(Startup, (
                setup_scene,
                setup_ui,
                load_dialogs,
            ))

            // Update systems - จัดลำดับใหม่เพื่อความชัดเจน
            .add_systems(Update, (
                // === Phase 1: Asset Loading & Setup ===
                setup_characters,
                check_character_assets,

                // === Phase 2: Core Dialog Management ===
                // ระบบหลักสำหรับจัดการ dialog state - ต้องทำงานก่อนทุกอย่าง
                manage_dialog_state
                    .after(setup_characters)
                    .before(handle_text_interaction)
                    .before(manage_choice_display),

                // Typewriter effect
                typewriter_system
                    .after(manage_dialog_state),

                // === Phase 3: User Input Handling ===
                // การจัดการ input ต่างๆ
                handle_text_interaction
                    .after(manage_dialog_state)
                    .before(handle_choice_selection), // ต้องทำก่อน choice selection

                handle_language_toggle
                    .after(manage_dialog_state),

                // === Phase 4: Choice Management ===
                // การจัดการตัวเลือก
                manage_choice_display
                    .after(manage_dialog_state)
                    .after(typewriter_system), // ต้องรอให้ typewriter ทำงานเสร็จก่อน

                highlight_choice_button
                    .after(manage_choice_display),

                handle_choice_selection
                    .after(manage_choice_display)
                    .after(highlight_choice_button),

                // === Phase 5: Visual Updates ===
                // การอัพเดทส่วน visual ต่างๆ
                update_characters
                    .after(manage_dialog_state)
                    .after(setup_characters),

                update_background
                    .after(manage_dialog_state),

                // === Phase 6: Transition Effects ===
                start_transition
                    .after(manage_dialog_state),

                update_transition,

                // === Phase 7: Debug Systems ===
                debug_choice_system,
                debug_characters,
            ));
    }
}

/// ระบบเสริมสำหรับจัดการ events - อาจจะใช้ในอนาคต
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

/// ระบบเสริมสำหรับจัดการ dialog reset events - อาจจะใช้ในอนาคต
pub fn handle_dialog_reset_events(
    mut events: EventReader<DialogResetEvent>,
    mut state: ResMut<VNState>,
) {
    for _event in events.read() {
        state.dialog_needs_reset = true;
    }
}