use bevy::prelude::*;
use crate::player::components::animation::{AnimationIndices, AnimationTimer};
use crate::player::components::player::{JumpState, PlayerAnimationSet, PlayerAnimationState};

pub fn animate_player_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(
        &mut JumpState,
        &PlayerAnimationSet,
        &mut PlayerAnimationState,
        &AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
    )>,
) {
    let is_walking = keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::KeyD);

    for (mut jump_state, animation_set, mut state, indices, mut timer, mut sprite) in &mut query {
        let target_state = if jump_state.is_jumping {
            PlayerAnimationState::Jump
        } else {
            PlayerAnimationState::Walk
        };

        if *state != target_state {
            *state = target_state;
            match target_state {
                PlayerAnimationState::Walk => {
                    sprite.image = animation_set.walk_texture.clone();
                    sprite.texture_atlas = Some(TextureAtlas {
                        layout: animation_set.walk_layout.clone(),
                        index: indices.first,
                    });
                }
                PlayerAnimationState::Jump => {
                    sprite.image = animation_set.jump_texture.clone();
                    sprite.texture_atlas = Some(TextureAtlas {
                        layout: animation_set.jump_layout.clone(),
                        index: jump_state.animation_frame,
                    });
                }
            }
        }

        match *state {
            PlayerAnimationState::Walk => {
                if !is_walking {
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = indices.first;
                    }
                    continue;
                }

                timer.0.tick(time.delta());
                if timer.0.just_finished() {
                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = if atlas.index >= indices.last {
                            indices.first
                        } else {
                            atlas.index + 1
                        };
                    }
                }
            }
            PlayerAnimationState::Jump => {
                jump_state.animation_timer.tick(time.delta());
                if jump_state.animation_timer.just_finished() && jump_state.animation_frame < 2 {
                    jump_state.animation_frame += 1;
                }

                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = jump_state.animation_frame;
                }
            }
        }
    }
}