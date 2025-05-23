use bevy::prelude::*;

/// สถานะหลักของเกม
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    /// หน้าเมนูหลัก
    MainMenu,
    /// กำลังเล่นเกม
    InGame,
    /// หยุดเกมชั่วคราว
    Paused,
    /// หน้าตั้งค่า
    Settings,
    /// กำลังโหลด
    Loading,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::MainMenu
    }
}

/// Events สำหรับการเปลี่ยนสถานะ
#[derive(Event)]
pub struct ChangeStateEvent {
    pub new_state: GameState,
}

/// Resource สำหรับเก็บสถานะก่อนหน้า (สำหรับ pause/resume)
#[derive(Resource, Default)]
pub struct PreviousState {
    pub state: Option<GameState>,
}

/// ระบบจัดการการเปลี่ยนสถานะ
pub fn handle_state_changes(
    mut events: EventReader<ChangeStateEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut previous_state: ResMut<PreviousState>,
    current_state: Res<State<GameState>>,
) {
    for event in events.read() {
        info!("เปลี่ยนสถานะจาก {:?} ไป {:?}", current_state.get(), event.new_state);

        // เก็บสถานะปัจจุบันไว้ก่อน (สำหรับ pause/resume)
        if event.new_state == GameState::Paused {
            previous_state.state = Some(current_state.get().clone());
        }

        next_state.set(event.new_state.clone());
    }
}

/// ระบบจัดการปุ่ม ESC สำหรับ pause/resume
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