use bevy::prelude::*;
use crate::common::util::identifier::texture;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bg_texture = texture("backgrounds/school.png").load::<Image>(&asset_server);

    // กล้อง
    commands.spawn(Camera2dBundle::default());

    // พื้นหลัง
    commands.spawn(SpriteBundle {
        texture: bg_texture,
        ..default()
    });
}