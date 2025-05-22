use bevy::prelude::*;
use crate::common::helium::{VNState, DialogHistory};
use crate::common::dialog::typewriter::TypewriterText;
use crate::client::render::ui::dialog::DialogBox;

/// Centralized input handler for dialog interactions
pub struct InputEvent {
    pub pressed: bool,
    pub source: InputSource,
}

/// Enum to identify the source of input
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputSource {
    MouseClick,
    TouchScreen,
    KeyPress(KeyCode),
    UIInteraction,
}

/// Check if any interaction has occurred (mouse, touch, or UI)
pub fn detect_interaction(
    mouse: &Res<ButtonInput<MouseButton>>,
    touch: &Res<Touches>,
    dialog_box_query: &Query<&Interaction, (With<DialogBox>, Changed<Interaction>)>,
) -> Option<InputEvent> {
    // ตรวจสอบการคลิกเมาส์
    if mouse.just_pressed(MouseButton::Left) {
        return Some(InputEvent {
            pressed: true,
            source: InputSource::MouseClick,
        });
    }
    
    // ตรวจสอบการแตะหน้าจอ
    if touch.iter_just_pressed().next().is_some() {
        return Some(InputEvent {
            pressed: true,
            source: InputSource::TouchScreen,
        });
    }
    
    // ตรวจสอบการกดที่ dialog box
    if dialog_box_query.iter().any(|&interaction| interaction == Interaction::Pressed) {
        return Some(InputEvent {
            pressed: true,
            source: InputSource::UIInteraction,
        });
    }
    
    None
}

/// Check for keyboard input based on specified keys
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

/// Check if dialog text is finished typing
pub fn is_dialog_text_finished(
    typewriter: &TypewriterText,
) -> bool {
    typewriter.char_index >= typewriter.full_text.chars().count()
}

/// Handle language switching with centralized logic
pub fn switch_language(
    state: &mut VNState,
) -> bool {
    let old_language = state.language.clone();
    state.language = if state.language == "thai" {
        "english".to_string()
    } else {
        "thai".to_string()
    };
    
    old_language != state.language
}

/// Centralized back button handling
pub fn handle_back_action(
    history: &mut DialogHistory,
) -> Option<usize> {
    history.go_back()
}