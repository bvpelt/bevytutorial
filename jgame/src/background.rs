use crate::components::{Background, GameState};
use bevy::prelude::*;

pub fn background_scroll_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Background)>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() != GameState::Playing {
        return;
    }

    let delta = time.delta_secs();

    for (mut transform, bg) in query.iter_mut() {
        transform.translation.x -= bg.speed * delta;

        if transform.translation.x <= -bg.width {
            transform.translation.x += bg.width;
        }
    }
}
