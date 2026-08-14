use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
    pub collisions: u32,
    pub max_collisions: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            value: 0,
            collisions: 0,
            max_collisions: 3, // Game over after 3 collisions / escaped enemies
        }
    }
}
