use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerAnimations {
    pub walk: Handle<Image>,
    pub attack: Handle<Image>,
    pub jump: Handle<Image>,
    pub heal: Handle<Image>,
    pub front: Handle<Image>,
}