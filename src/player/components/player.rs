use bevy::prelude::*;

pub const PLAYER_FRAME_WIDTH: f32 = 153.0;
pub const PLAYER_FRAME_HEIGHT: f32 = 204.0;
pub const PLAYER_SCALE: f32 = 0.35;
pub const JUMP_FRAME_WIDTH: f32 = 288.0;
pub const JUMP_FRAME_HEIGHT: f32 = 288.0;
pub const GROUND_Y: f32 = 0.0;
pub const JUMP_VELOCITY: f32 = 520.0;
pub const GRAVITY: f32 = -1200.0;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerAnimationState {
	Walk,
	Jump,
}

#[derive(Component, Clone)]
pub struct PlayerAnimationSet {
	pub walk_texture: Handle<Image>,
	pub walk_layout: Handle<TextureAtlasLayout>,
	pub jump_texture: Handle<Image>,
	pub jump_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Debug)]
pub struct JumpState {
	pub vertical_velocity: f32,
	pub is_jumping: bool,
	pub animation_frame: usize,
	pub animation_timer: Timer,
}

impl Default for JumpState {
	fn default() -> Self {
		Self {
			vertical_velocity: 0.0,
			is_jumping: false,
			animation_frame: 0,
			animation_timer: Timer::from_seconds(0.10, TimerMode::Repeating),
		}
	}
}

#[derive(Component, Debug, Default)]
pub struct Player;

