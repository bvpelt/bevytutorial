use crate::components::{AnimationIndices, AnimationTimer, Player};
use bevy::prelude::*;

pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let delta = time.delta_secs();

    for (mut transform, mut player) in query.iter_mut() {
        let mut x_direction = 0.0;

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            x_direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            x_direction -= 1.0;
        }

        // Jump impulse
        if keyboard_input.just_pressed(KeyCode::ArrowUp) && player.is_grounded {
            player.vy = 650.0;
            player.is_grounded = false;
        }

        // Horizontal Movement
        transform.translation.x += x_direction * player.speed * delta;
        transform.translation.x = transform.translation.x.clamp(-300.0, 300.0);

        // Vertical Gravity Physics
        if !player.is_grounded {
            player.vy -= player.weight * delta;
            transform.translation.y += player.vy * delta;

            if transform.translation.y <= player.ground_y {
                transform.translation.y = player.ground_y;
                player.vy = 0.0;
                player.is_grounded = true;
            }
        }
    }
}

pub fn player_animation_system(
    time: Res<Time>,
    mut query: Query<(
        &Player,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
    )>,
) {
    let delta = time.delta();

    for (player, mut indices, mut timer, mut sprite) in query.iter_mut() {
        // Switch frames depending on jump state
        if !player.is_grounded {
            indices.first = 0;
            indices.last = 5;
        } else {
            indices.first = 0;
            indices.last = 8;
        }

        timer.tick(delta);
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index >= indices.last {
                    atlas.index = indices.first;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
