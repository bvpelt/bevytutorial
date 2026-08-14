use bevy::prelude::*;
use rand::RngExt;

use crate::background::GameSpeed;
use crate::enemies::Enemy;
use crate::player::Player;
use crate::score::Score; // <-- Import Score from the crate root

// ----------------------------------------------------------------------------
// Components & Resources
// ----------------------------------------------------------------------------

#[derive(Component)]
pub struct CollisionEffect {
    pub current_frame: u32,
    pub max_frame: u32,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Resource)]
pub struct CollisionAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub boom_sound: Handle<AudioSource>,
}

// ----------------------------------------------------------------------------
// Plugin Implementation
// ----------------------------------------------------------------------------

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_collision_assets)
            .add_systems(
                Update,
                (
                    check_player_enemy_collisions,
                    update_collision_effects,
                    animate_collision_effects,
                ),
            );
    }
}

// ----------------------------------------------------------------------------
// Systems
// ----------------------------------------------------------------------------

fn setup_collision_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("boom.png");
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(100, 90),
        5,
        1,
        None,
        None,
    ));

    let boom_sound = asset_server.load("boom.wav");

    commands.insert_resource(CollisionAssets {
        texture,
        layout,
        boom_sound,
    });
}

/// AABB Collision Detection System
fn check_player_enemy_collisions(
    mut commands: Commands,
    player_query: Query<(&Transform, &Sprite), With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Enemy)>,
    assets: Res<CollisionAssets>,
    mut score: ResMut<Score>,
) {
    let Ok((player_transform, _player_sprite)) = player_query.single() else {
        return;
    };

    let use_sound = false;
    let player_width = 100.0;
    let player_height = 91.0;
    let player_pos = player_transform.translation;

    for (enemy_entity, enemy_transform, enemy) in &enemy_query {
        let enemy_pos = enemy_transform.translation;

        let collision_x = (player_pos.x - enemy_pos.x).abs() * 2.0 < (player_width + enemy.width);
        let collision_y = (player_pos.y - enemy_pos.y).abs() * 2.0 < (player_height + enemy.height);

        if collision_x && collision_y {
            // 1. Despawn enemy on hit
            commands.entity(enemy_entity).despawn();

            // 2. Spawn collision boom visual effect
            spawn_collision_effect(&mut commands, &assets, enemy_pos);

            // 3. Play boom sound
            if use_sound {
                commands.spawn(AudioPlayer(assets.boom_sound.clone()));
            }
            // 4. Update Score & Collisions
            score.value += 1;
            score.collisions += 1;
        }
    }
}

fn spawn_collision_effect(commands: &mut Commands, assets: &CollisionAssets, position: Vec3) {
    let mut rng = rand::rng();
    let size_modifier: f32 = rng.random_range(0.5..1.5);
    let fps: f64 = rng.random_range(5.0..15.0);

    commands.spawn((
        Sprite {
            image: assets.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(position).with_scale(Vec3::splat(size_modifier)),
        CollisionEffect {
            current_frame: 0,
            max_frame: 4,
        },
        AnimationTimer(Timer::from_seconds(
            (1.0 / fps) as f32,
            TimerMode::Repeating,
        )),
    ));
}

fn update_collision_effects(
    game_speed: Res<GameSpeed>,
    mut query: Query<&mut Transform, With<CollisionEffect>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= game_speed.0;
    }
}

fn animate_collision_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut AnimationTimer,
        &mut CollisionEffect,
        &mut Sprite,
    )>,
) {
    for (entity, mut timer, mut effect, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            effect.current_frame += 1;

            if effect.current_frame > effect.max_frame {
                commands.entity(entity).despawn();
            } else if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = effect.current_frame as usize;
            }
        }
    }
}
