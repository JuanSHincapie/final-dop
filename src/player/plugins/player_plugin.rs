use bevy::prelude::*;
use crate::player::systems::{
    animate_player_system::animate_player_system,
    player_movement_system::player_movement_system,
    spawn_player_system::spawn_player_system,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system)
            .add_systems(Update, (player_movement_system, animate_player_system));
    }
}