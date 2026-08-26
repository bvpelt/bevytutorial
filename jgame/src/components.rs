use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Resource)]
pub struct GameAssets {
    pub player_atlas_layout: Handle<TextureAtlasLayout>,
    pub enemy_atlas_layout: Handle<TextureAtlasLayout>,
    pub player_texture: Handle<Image>,
    pub bg_texture: Handle<Image>,
    pub enemy_texture: Handle<Image>,
}

#[derive(Component)]
pub struct Player {
    pub vy: f32,
    pub weight: f32,
    pub ground_y: f32,
    pub is_grounded: bool,
    pub speed: f32,
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
}

#[derive(Component)]
pub struct Background {
    pub speed: f32,
    pub width: f32,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct GameOverText;
