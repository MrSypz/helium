use bevy::prelude::*;
use crate::core::resources::{DialogResource, VNState, DialogManager};
use crate::util::identifier::dialog;
use crate::types::DialogScene;

pub fn load_dialogs(
    asset_server: Res<AssetServer>,
    mut dialog_resource: ResMut<DialogResource>,
    mut vn_state: ResMut<VNState>,
) {
    let intro_scene = dialog("intro.dialog.json").load::<DialogScene>(&asset_server);
    let school_scene = dialog("school.dialog.json").load::<DialogScene>(&asset_server);
    let choices_scene = dialog("choices.dialog.json").load::<DialogScene>(&asset_server);

    dialog_resource.scenes.insert("intro".to_string(), intro_scene.clone());
    dialog_resource.scenes.insert("school".to_string(), school_scene);
    dialog_resource.scenes.insert("choices".to_string(), choices_scene);

    dialog_resource.current_scene = Some(intro_scene.clone());
    vn_state.current_scene_handle = Some(intro_scene);
}

pub fn manage_dialog_state(
    mut state: ResMut<VNState>,
    mut dialog_manager: ResMut<DialogManager>,
    dialog_resource: Res<DialogResource>,
    dialog_scenes: Res<Assets<DialogScene>>,
    mut character_query: Query<&mut Text, (With<crate::ui::dialog::CharacterName>, Without<crate::core::dialog::typewriter::TypewriterText>)>,
    mut dialog_query: Query<(&mut Text, &mut crate::core::dialog::typewriter::TypewriterText), With<crate::ui::dialog::DialogText>>,
) {
    if state.should_reset_dialog() {
        for mut text in character_query.iter_mut() {
            text.sections[0].value = "".to_string();
        }

        if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
            *typewriter = crate::core::dialog::typewriter::TypewriterText::new("", 0.05);
            text.sections[0].value = "".to_string();
        }

        dialog_manager.reset();
        state.mark_dialog_reset();
    }

    if dialog_manager.is_processing() {
        if let Some(scene_handle) = &dialog_resource.current_scene {
            if let Some(scene) = dialog_scenes.get(scene_handle) {
                if state.stage < scene.entries.len() {
                    let entry = &scene.entries[state.stage];

                    let character_display_name = scene
                        .characters
                        .iter()
                        .find(|c| c.name == entry.character)
                        .and_then(|c| c.display_name.get(&state.language))
                        .cloned()
                        .unwrap_or_else(|| entry.character.clone());

                    let dialog_text = entry
                        .text
                        .get(&state.language)
                        .cloned()
                        .unwrap_or_else(|| format!("[No text in {}]", state.language));

                    dialog_manager.set_content(character_display_name.clone(), dialog_text.clone());

                    for mut text in character_query.iter_mut() {
                        text.sections[0].value = character_display_name.clone();
                    }

                    if let Ok((mut text, mut typewriter)) = dialog_query.get_single_mut() {
                        *typewriter = crate::core::dialog::typewriter::TypewriterText::new(&dialog_text, 0.05);
                        text.sections[0].value = "".to_string();
                    }
                }
            }
        }
    }
}