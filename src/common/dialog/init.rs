use bevy::prelude::*;
use crate::common::helium::{DialogResource, VNState};
use crate::util::identifier::dialog;
use crate::common::dialog::types::DialogScene;

/// โหลด dialog scenes ทั้งหมดในเกม
pub fn load_dialogs(
    _commands: Commands,
    asset_server: Res<AssetServer>,
    mut dialog_resource: ResMut<DialogResource>,
    mut vn_state: ResMut<VNState>,
) {
    let intro_scene = dialog("intro.dialog.json").load::<DialogScene>(&asset_server);
    let school_scene = dialog("school.dialog.json").load::<DialogScene>(&asset_server);
    let choices_scene = dialog("choices.dialog.json").load::<DialogScene>(&asset_server);

    dialog_resource.scenes.insert("intro".to_string(), intro_scene.clone());
    dialog_resource.scenes.insert("school".to_string(), school_scene);
    dialog_resource.scenes.insert("choices".to_string(), choices_scene.clone());

    dialog_resource.current_scene = Some(intro_scene.clone());
    vn_state.current_scene_handle = Some(intro_scene);
}