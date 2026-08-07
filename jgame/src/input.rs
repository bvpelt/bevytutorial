use bevy::prelude::*;

use crate::background::GameSpeed;
use crate::player::{Player, PlayerState, TEXTURE_COLS};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_input, update_game_speed));
    }
}

fn update_game_speed(keyboard_input: Res<ButtonInput<KeyCode>>, mut game_speed: ResMut<GameSpeed>) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        game_speed.0 += 5.0;
        info!("Game speed increased to: {}", game_speed.0);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS)
    {
        game_speed.0 = (game_speed.0 - 5.0).max(0.0);
        info!("Game speed decreased to: {}", game_speed.0);
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerState, &mut Player, &mut Sprite), With<Player>>,
) {
    let mut new_state = None;

    if keyboard_input.just_pressed(KeyCode::Numpad5) {
        new_state = Some(PlayerState::Idle);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad8) {
        new_state = Some(PlayerState::Jump);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad2) {
        new_state = Some(PlayerState::Fall);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad6) {
        new_state = Some(PlayerState::Run);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad3) {
        new_state = Some(PlayerState::Dizzy);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad1) {
        new_state = Some(PlayerState::Sit);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad7) {
        new_state = Some(PlayerState::Roll);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad4) {
        new_state = Some(PlayerState::Bite);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad9) {
        new_state = Some(PlayerState::Ko);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad0) {
        new_state = Some(PlayerState::GetHit);
    }

    if let Some(state) = new_state {
        for (mut current_state, mut player, mut sprite) in &mut query {
            if *current_state != state {
                *current_state = state;
                player.current_frame = 0;

                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = (state.row_index() * TEXTURE_COLS) as usize;
                }

                info!("Switched animation to: {:?}", state);
            }
        }
    }
}
