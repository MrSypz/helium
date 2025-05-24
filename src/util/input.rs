use bevy::prelude::*;
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

pub fn is_dialog_text_finished(typewriter: &TypewriterText) -> bool {
    typewriter.char_index >= typewriter.full_text.chars().count()
}
