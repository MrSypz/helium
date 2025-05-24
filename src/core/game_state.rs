use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
    Paused,
    Settings,
    Loading,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::MainMenu
    }
}

#[derive(Event)]
pub struct ChangeStateEvent {
    pub new_state: GameState,
}

#[derive(Resource, Default)]
pub struct PreviousState {
    pub state: Option<GameState>,
}

pub fn handle_state_changes(
    mut events: EventReader<ChangeStateEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut previous_state: ResMut<PreviousState>,
    current_state: Res<State<GameState>>,
) {
    for event in events.read() {
        // เก็บ previous state เฉพาะเมื่อไป Paused หรือ Settings
        if event.new_state == GameState::Paused || event.new_state == GameState::Settings {
            previous_state.state = Some(current_state.get().clone());
        }
        next_state.set(event.new_state.clone());
    }
}

pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut change_events: EventWriter<ChangeStateEvent>,
    previous_state: Res<PreviousState>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::InGame => {
                change_events.send(ChangeStateEvent {
                    new_state: GameState::Paused,
                });
            }
            GameState::Paused => {
                // กลับไป InGame เสมอ (ไม่ใช้ previous_state)
                change_events.send(ChangeStateEvent {
                    new_state: GameState::InGame,
                });
            }
            GameState::Settings => {
                // กลับไป state ก่อนหน้า หรือ InGame ถ้าไม่มี
                if let Some(prev_state) = &previous_state.state {
                    change_events.send(ChangeStateEvent {
                        new_state: prev_state.clone(),
                    });
                } else {
                    change_events.send(ChangeStateEvent {
                        new_state: GameState::InGame,
                    });
                }
            }
            _ => {}
        }
    }
}