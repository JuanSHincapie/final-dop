use bevy::prelude::*;
use player::plugins::player_plugin::PlayerPlugin;

mod player;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "src/assets".to_string(),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .run();
}