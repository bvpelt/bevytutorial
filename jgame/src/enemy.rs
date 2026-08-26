use bevy::prelude::*;
use rand::RngExt;

use crate::components::{
    AnimationIndices, AnimationTimer, Enemy, EnemySpawnTimer, GameAssets, GameState, Score,
};

pub fn enemy_spawner_system(
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    assets: Res<GameAssets>,
    mut commands: Commands,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() != GameState::Playing {
        return;
    }

    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut rng = rand::rng();

        commands.spawn((
            Sprite {
                image: assets.enemy_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.enemy_atlas_layout.clone(),
                    index: 0,
                }),
                custom_size: Some(Vec2::new(160.0, 119.0)),
                ..default()
            },
            Transform::from_xyz(450.0, -260.5, 2.0),
            Enemy { speed: 300.0 },
            AnimationIndices { first: 0, last: 5 },
            AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        ));

        let next_interval = rng.random_range(0.8..1.8);
        timer
            .0
            .set_duration(std::time::Duration::from_secs_f32(next_interval));
    }
}

pub fn enemy_movement_system(
    time: Res<Time>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut query: Query<(
        Entity,
        &mut Transform,
        &Enemy,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut Sprite,
    )>,
    game_state: Res<State<GameState>>,
) {
    let delta = time.delta_secs();

    for (entity, mut transform, enemy, indices, mut timer, mut sprite) in query.iter_mut() {
        if *game_state.get() == GameState::Playing {
            transform.translation.x -= enemy.speed * delta;

            timer.tick(time.delta());
            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index >= indices.last {
                        indices.first
                    } else {
                        atlas.index + 1
                    };
                }
            }

            if transform.translation.x < -450.0 {
                commands.entity(entity).despawn();
                score.0 += 1;
            }
        }
    }
}
