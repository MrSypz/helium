use bevy::prelude::*;

#[derive(Component)]
pub struct TypewriterText {
    pub full_text: String,
    pub current_text: String,
    pub timer: Timer,
    pub char_index: usize,
}

impl TypewriterText {
    pub fn new(text: &str, speed: f32) -> Self {
        TypewriterText {
            full_text: text.to_string(),
            current_text: String::new(),
            timer: Timer::from_seconds(speed, TimerMode::Repeating),
            char_index: 0,
        }
    }
}