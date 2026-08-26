use crate::input::LastInput;
use crate::state::PlayerState;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub speed: f32,
    pub max_speed: f32,
    pub vy: f32,
    pub jump_force: f32,
    pub weight: f32,
    pub ground_y: f32,
    pub bounds_x: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            state: PlayerState::StandingRight,
            speed: 0.0,
            max_speed: 400.0,
            vy: 0.0,
            jump_force: 700.0,
            weight: 1500.0,
            ground_y: -360.0 + 90.9,
            bounds_x: 600.0,
        }
    }
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

pub fn player_state_system(
    last_input: Res<LastInput>,
    mut query: Query<(&Transform, &mut Player, &mut Sprite)>,
) {
    let Ok((transform, mut player, mut sprite)) = query.single_mut() else {
        return;
    };

    let on_ground = transform.translation.y <= player.ground_y;
    let current_state = player.state;

    if let Some(new_state) = current_state.next_state(last_input.0, on_ground, player.vy) {
        player.state = new_state;

        match new_state {
            PlayerState::JumpingLeft | PlayerState::JumpingRight => {
                if on_ground {
                    player.vy = player.jump_force;
                }
            }
            _ => {}
        }

        if let Some(atlas) = &mut sprite.texture_atlas {
            let (frame_y, _) = new_state.get_sprite_info();
            atlas.index = frame_y * 9;
        }
    }

    player.speed = player.state.get_target_speed(player.max_speed);
}

pub fn player_physics_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Player)>) {
    let delta = time.delta_secs();
    let Ok((mut transform, mut player)) = query.single_mut() else {
        return;
    };

    // Horizontal movement
    transform.translation.x += player.speed * delta;
    transform.translation.x = transform
        .translation
        .x
        .clamp(-player.bounds_x, player.bounds_x);

    // Vertical movement & gravity
    transform.translation.y += player.vy * delta;

    if transform.translation.y > player.ground_y {
        player.vy -= player.weight * delta;
    } else {
        player.vy = 0.0;
        transform.translation.y = player.ground_y;
    }
}

pub fn player_animation_system(
    time: Res<Time>,
    mut query: Query<(&Player, &mut Sprite, &mut AnimationTimer)>,
) {
    let Ok((player, mut sprite, mut timer)) = query.single_mut() else {
        return;
    };
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            let (frame_y, max_frame) = player.state.get_sprite_info();
            let start_idx = frame_y * 9;
            let current_offset = atlas.index.saturating_sub(start_idx);

            if current_offset < max_frame {
                atlas.index = start_idx + current_offset + 1;
            } else {
                atlas.index = start_idx;
            }
        }
    }
}
