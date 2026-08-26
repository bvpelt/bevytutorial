use crate::components::Background;
use bevy::prelude::*;

pub fn background_scroll_system(time: Res<Time>, mut query: Query<(&mut Transform, &Background)>) {
    for (mut transform, background) in query.iter_mut() {
        // Move background to the left
        transform.translation.x -= background.speed * time.delta_secs();

        // Reposition tile to the right once it moves off-screen to the left
        if transform.translation.x <= -background.width {
            transform.translation.x += background.width * 2.0;
        }
    }
}
