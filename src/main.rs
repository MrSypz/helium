use bevy::prelude::*;
use helium::common::plugin::VNPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Helium Visual Novel".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(VNPlugin)
        .run();
}