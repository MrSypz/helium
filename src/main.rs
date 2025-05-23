use bevy::prelude::*;
use helium::core::plugin::VNPlugin;
use helium::core::game_state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Helium Visual Novel".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VNPlugin)
        .insert_state(GameState::MainMenu)
        .run();
}