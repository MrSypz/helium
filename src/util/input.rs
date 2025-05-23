use bevy::prelude::*;
use crate::core::resources::{VNState, DialogHistory};
use crate::core::dialog::typewriter::TypewriterText;
use crate::ui::dialog::DialogBox;

pub struct InputEvent {
    pub pressed: bool,
    pub source: InputSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputSource {
    MouseClick,
    TouchScreen,
    KeyPress(KeyCode),
    UIInteraction,
}

pub fn detect_interaction(
    mouse: &Res<ButtonInput<MouseButton>>,
    touch: &Res<Touches>,
    dialog_box_query: &Query<&Interaction, (With<DialogBox>, Changed<Interaction>)>,
) -> Option<InputEvent> {
    if mouse.just_pressed(MouseButton::Left) {
        return Some(InputEvent {
            pressed: true,
            source: InputSource::MouseClick,
        });
    }

    if touch.iter_just_pressed().next().is_some() {
        return Some(InputEvent {
            pressed: true,
            source: InputSource::TouchScreen,
        });
    }

    if dialog_box_query.iter().any(|&interaction| interaction == Interaction::Pressed) {
        return Some(InputEvent {
            pressed: true,
            source: InputSource::UIInteraction,
        });
    }

    None
}

pub fn detect_key_press(
    keyboard: &Res<ButtonInput<KeyCode>>,
    keys: &[KeyCode],
) -> Option<InputEvent> {
    for &key in keys {
        if keyboard.just_pressed(key) {
            return Some(InputEvent {
                pressed: true,
                source: InputSource::KeyPress(key),
            });
        }
    }
    None
}

pub fn is_dialog_text_finished(typewriter: &TypewriterText) -> bool {
    typewriter.char_index >= typewriter.full_text.chars().count()
}

pub fn switch_language(state: &mut VNState) -> bool {
    let old_language = state.language.clone();
    state.language = if state.language == "thai" {
        "english".to_string()
    } else {
        "thai".to_string()
    };

    old_language != state.language
}

pub fn handle_back_action(history: &mut DialogHistory) -> Option<usize> {
    history.go_back()
}