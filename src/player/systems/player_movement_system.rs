use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::components::player::{
    GROUND_Y,
    GRAVITY,
    JumpState,
    JUMP_VELOCITY,
    Player,
    PLAYER_FRAME_WIDTH,
    PLAYER_SCALE,
};

const PLAYER_SPEED: f32 = 220.0;

pub fn player_movement_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut JumpState), With<Player>>,
) {
    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let Ok(window) = window_query.get_single() else {
        return;
    };

    let half_player_width = (PLAYER_FRAME_WIDTH * PLAYER_SCALE) * 0.5;
    let min_x = -window.width() * 0.5 + half_player_width;
    let max_x = window.width() * 0.5 - half_player_width;

    for (mut transform, mut jump_state) in &mut query {
        if keyboard.just_pressed(KeyCode::KeyW) && !jump_state.is_jumping {
            jump_state.animation_frame = 0;
            jump_state.animation_timer.reset();
            jump_state.is_jumping = true;
            jump_state.vertical_velocity = JUMP_VELOCITY;
        }

        if direction != 0.0 {
            transform.translation.x += direction * PLAYER_SPEED * time.delta_secs();
            transform.translation.x = transform.translation.x.clamp(min_x, max_x);
        }

        if jump_state.is_jumping || transform.translation.y > GROUND_Y {
            jump_state.vertical_velocity += GRAVITY * time.delta_secs();
            transform.translation.y += jump_state.vertical_velocity * time.delta_secs();

            if transform.translation.y <= GROUND_Y {
                transform.translation.y = GROUND_Y;
                jump_state.vertical_velocity = 0.0;
                jump_state.is_jumping = false;
                jump_state.animation_frame = 0;
            }
        }
    }
}