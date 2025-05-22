// src/common/plugin.rs
use bevy::prelude::*;
use crate::common::helium::{VNState, DialogResource, DialogHistory};
use crate::common::dialog::types::DialogScene;
use crate::common::dialog::typewriter::typewriter_system;
use crate::common::dialog::choice::{ChoiceState, handle_choice_click, debug_choice_system};
use crate::client::render::ui::dialog::{setup_ui, update_dialog, text_click, toggle_language};
use crate::client::render::ui::choice::{display_choices, highlight_choice_button, cleanup_overlay_on_choice_change};
use crate::client::render::setup::{setup_scene, update_background};
use crate::client::render::character::{setup_characters, update_characters, check_character_assets, debug_characters};
use crate::client::render::transition::{ActiveTransition, start_transition, update_transition};
use crate::common::dialog::init::load_dialogs;

pub struct VNPlugin;

impl Plugin for VNPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<DialogScene>()
            .init_asset_loader::<crate::common::dialog::types::DialogLoader>()
            .init_resource::<VNState>()
            .init_resource::<DialogResource>()
            .init_resource::<DialogHistory>()
            .init_resource::<ChoiceState>()
            .init_resource::<ActiveTransition>()
            .add_systems(Startup, (
                setup_scene,
                setup_ui,
                load_dialogs,
            ))
            // จัดกลุ่มและลำดับการทำงานของระบบให้เหมาะสม
            .add_systems(Update, (
                // ระบบโหลด assets และ setup ตัวละคร - ต้องทำก่อน
                setup_characters,
                check_character_assets,

                // ระบบ dialog
                update_dialog,
                typewriter_system,
                text_click.after(display_choices),

                // ระบบตัวละคร - ต้องทำหลังจาก setup_characters
                update_characters.after(setup_characters),

                // ระบบพื้นหลัง
                update_background.after(update_dialog),

                // ระบบ transition
                start_transition.after(update_dialog),
                update_transition,

                // ระบบ choice
                display_choices.after(update_dialog),
                highlight_choice_button.after(display_choices),
                handle_choice_click.after(display_choices),
                cleanup_overlay_on_choice_change,

                // ระบบอื่นๆ
                toggle_language,
                debug_choice_system,
                debug_characters, // เพิ่มระบบ debug
            ));
    }
}