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

pub fn typewriter_system(
    time: Res<Time>,
    mut query: Query<(&mut Text, &mut TypewriterText)>,
) {
    for (mut text, mut typewriter) in query.iter_mut() {
        if typewriter.char_index < typewriter.full_text.chars().count() {
            typewriter.timer.tick(time.delta());

            if typewriter.timer.just_finished() {
                if let Some(next_char) = typewriter.full_text.chars().nth(typewriter.char_index) {
                    typewriter.current_text.push(next_char);
                    typewriter.char_index += 1;
                    text.sections[0].value = typewriter.current_text.clone();
                }
            }
        }
    }
}