use bevy::prelude::*;
use crate::common::helium::{VNState, DialogResource, DialogHistory};
use crate::common::dialog::types::DialogScene;
use crate::common::dialog::typewriter::typewriter_system;
use crate::common::dialog::choice::{ChoiceState, handle_choice_click, handle_back_button, debug_choice_system};
use crate::client::render::ui::dialog::{setup_ui, update_dialog, text_click, toggle_language};
use crate::client::render::ui::choice::{display_choices, highlight_choice_button};
use crate::client::render::setup::setup_scene;
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
            .add_systems(Startup, (
                setup_scene,
                setup_ui,
                load_dialogs,
            ))
            // จัดกลุ่มและลำดับการทำงานของระบบให้เหมาะสม
            .add_systems(Update, (
                // ระบบ dialog
                update_dialog,
                typewriter_system,
                text_click.after(display_choices),

                // ระบบ choice
                display_choices.after(update_dialog),
                highlight_choice_button.after(display_choices),
                handle_choice_click.after(display_choices),

                // ระบบอื่นๆ
                toggle_language,
                handle_back_button,
                debug_choice_system,
            ));
    }
}