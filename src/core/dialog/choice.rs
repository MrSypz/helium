use bevy::prelude::*;
use crate::core::resources::{VNState, DialogHistory};
use crate::types::DialogChoice;

#[derive(Component)]
pub struct ChoiceButton {
    pub choice_index: usize,
    pub target_stage: usize,
}

#[derive(Resource, Default)]
pub struct ChoiceState {
    pub active: bool,
    pub choices: Vec<DialogChoice>,
    pub history: Vec<usize>,
}

impl ChoiceState {
    pub fn add_choice(&mut self, choice_index: usize) {
        self.history.push(choice_index);
    }

    pub fn activate(&mut self, choices: Vec<DialogChoice>) {
        self.active = true;
        self.choices = choices;
    }

    pub fn deactivate(&mut self) {
        if self.active {
            self.active = false;
            self.choices.clear();
        }
    }
}

pub fn handle_choice_selection(
    mut commands: Commands,
    mut state: ResMut<VNState>,
    mut choice_state: ResMut<ChoiceState>,
    mut history: ResMut<DialogHistory>,
    choice_query: Query<(&ChoiceButton, &Interaction), Changed<Interaction>>,
    container_query: Query<Entity, With<crate::ui::choice::ChoiceContainer>>,
    overlay_query: Query<Entity, With<crate::ui::choice::ChoiceOverlay>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !choice_state.active {
        return;
    }

    let mut selected_choice: Option<(usize, usize)> = None;

    let number_keys = [
        KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3,
        KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6,
        KeyCode::Digit7, KeyCode::Digit8, KeyCode::Digit9,
    ];

    for (i, key) in number_keys.iter().enumerate() {
        if keyboard.just_pressed(*key) && i < choice_state.choices.len() {
            let target_stage = choice_state.choices[i].target_stage;
            selected_choice = Some((i, target_stage));
            break;
        }
    }

    if selected_choice.is_none() {
        for (choice, interaction) in choice_query.iter() {
            if *interaction == Interaction::Pressed {
                selected_choice = Some((choice.choice_index, choice.target_stage));
                break;
            }
        }
    }

    if let Some((choice_index, target_stage)) = selected_choice {
        choice_state.add_choice(choice_index);
        history.add_choice(state.stage, choice_index, target_stage);
        choice_state.deactivate();
        state.change_stage(target_stage);

        for entity in container_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in overlay_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}