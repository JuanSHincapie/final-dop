// src/player/systems/spawn_player_system.rs
use bevy::prelude::*;
use crate::player::components::animation::{AnimationIndices, AnimationTimer};
use crate::player::components::player::{
    JumpState,
    JUMP_FRAME_HEIGHT,
    JUMP_FRAME_WIDTH,
    Player,
    PlayerAnimationSet,
    PlayerAnimationState,
    PLAYER_FRAME_HEIGHT,
    PLAYER_FRAME_WIDTH,
    PLAYER_SCALE,
};

pub fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let walk_texture = asset_server.load("sprites/player/walk.png");
    let jump_texture = asset_server.load("sprites/player/jump.png");

    let walk_layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLAYER_FRAME_WIDTH as u32, PLAYER_FRAME_HEIGHT as u32),
        4,
        2,
        None,
        None,
    );

    let jump_layout = TextureAtlasLayout::from_grid(
        UVec2::new(JUMP_FRAME_WIDTH as u32, JUMP_FRAME_HEIGHT as u32),
        3,
        1,
        Some(UVec2::new(1, 0)),
        None,
    );

    let walk_texture_atlas_layout = texture_atlas_layouts.add(walk_layout);
    let jump_texture_atlas_layout = texture_atlas_layouts.add(jump_layout);

    let animation_set = PlayerAnimationSet {
        walk_texture: walk_texture.clone(),
        walk_layout: walk_texture_atlas_layout.clone(),
        jump_texture,
        jump_layout: jump_texture_atlas_layout,
    };

    commands.spawn((
        Sprite::from_atlas_image(
            walk_texture,
            TextureAtlas {
                layout: walk_texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        Player,
        JumpState::default(),
        animation_set,
        PlayerAnimationState::Walk,
        AnimationIndices {
            first: 0,
            last: 7,
        },
        AnimationTimer(Timer::from_seconds(
            0.12,
            TimerMode::Repeating,
        )),
    ));
}