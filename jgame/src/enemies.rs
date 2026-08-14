use bevy::prelude::*;
use rand::Rng;
use rand::RngExt;

use crate::background::GameSpeed;
use crate::player::AnimationTimer;

// Constants matching your JS setup
const GROUND_MARGIN: f32 = 40.0;
const SCREEN_WIDTH: f32 = 900.0;
const SCREEN_HEIGHT: f32 = 500.0;

// ----------------------------------------------------------------------------
// Types & Components
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Flying,
    Ground,
    Climbing,
}

#[derive(Component)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub speed_x: f32,
    pub speed_y: f32,
    pub current_frame: u32,
    pub max_frame: u32,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
    pub va: f32, // Velocity for sine wave trajectory
}

/// Timer resource for spawning enemies periodically
#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

/// Resource storing handles to enemy sprite sheets and layouts
#[derive(Resource)]
pub struct EnemyAssets {
    pub fly_texture: Handle<Image>,
    pub fly_layout: Handle<TextureAtlasLayout>,
    pub raven_texture: Handle<Image>,
    pub raven_layout: Handle<TextureAtlasLayout>,
    pub plant_texture: Handle<Image>,
    pub plant_layout: Handle<TextureAtlasLayout>,
    pub spider_texture: Handle<Image>,
    pub spider_layout: Handle<TextureAtlasLayout>,
}

// ----------------------------------------------------------------------------
// Plugin Implementation
// ----------------------------------------------------------------------------

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup_enemy_assets)
        .add_systems(
            Update,
            (
                spawn_enemies,
                update_enemies,
                animate_enemies,
                draw_spider_webs,
                cleanup_offscreen_enemies,
            ),
        );
    }
}

// ----------------------------------------------------------------------------
// Systems
// ----------------------------------------------------------------------------

fn setup_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Flying Enemy: 60x44 (6 frames)
    let fly_texture = asset_server.load("enemy_fly.png");
    let fly_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(60, 44),
        6,
        1,
        None,
        None,
    ));

    // 271x194 (6 frames)
    let raven_texture = asset_server.load("raven.png");
    let raven_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(271, 194),
        6,
        1,
        None,
        None,
    ));

    // Ground Enemy (Plant): 60x87 (2 frames)
    let plant_texture = asset_server.load("enemy_plant.png");
    let plant_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(60, 87),
        2,
        1,
        None,
        None,
    ));

    // Climbing Enemy (Spider): 120x144 (6 frames)
    let spider_texture = asset_server.load("enemy_spider_big.png");
    let spider_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(120, 144),
        6,
        1,
        None,
        None,
    ));

    commands.insert_resource(EnemyAssets {
        fly_texture,
        fly_layout,
        raven_texture,
        raven_layout,
        plant_texture,
        plant_layout,
        spider_texture,
        spider_layout,
    });
}

fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    game_speed: Res<GameSpeed>,
    assets: Res<EnemyAssets>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut rng = rand::rng();

        // Spawn Flying Enemy
        spawn_flying_enemy(&mut commands, &assets, &mut rng);

        // Conditional ground/climbing spawn if game speed > 0
        if game_speed.0 > 0.0 {
            if rng.random_bool(0.5) {
                spawn_ground_enemy(&mut commands, &assets);
            } else {
                spawn_climbing_enemy(&mut commands, &assets, &mut rng);
            }
        }
    }
}

fn spawn_flying_enemy(commands: &mut Commands, assets: &EnemyAssets, rng: &mut impl Rng) {
    let x = (SCREEN_WIDTH / 2.0) + rng.random_range(0.0..(SCREEN_WIDTH * 0.5));
    let y = rng.random_range(0.0..(SCREEN_HEIGHT * 0.5)) - (SCREEN_HEIGHT / 2.0);

    commands.spawn((
        Sprite {
            image: assets.fly_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.fly_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        Enemy {
            enemy_type: EnemyType::Flying,
            speed_x: rng.random_range(1.0..2.0),
            speed_y: 0.0,
            current_frame: 0,
            max_frame: 5,
            width: 60.0,
            height: 44.0,
            angle: 0.0,
            va: rng.random_range(0.1..0.2),
        },
        AnimationTimer(Timer::from_seconds(1.0 / 20.0, TimerMode::Repeating)),
    ));

    commands.spawn((
        Sprite {
            image: assets.raven_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.raven_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        Enemy {
            enemy_type: EnemyType::Flying,
            speed_x: rng.random_range(1.0..2.0),
            speed_y: 0.0,
            current_frame: 0,
            max_frame: 5,
            width: 271.0,
            height: 194.0,
            angle: 0.0,
            va: rng.random_range(0.1..0.2),
        },
        AnimationTimer(Timer::from_seconds(1.0 / 20.0, TimerMode::Repeating)),
    ));
}

fn spawn_ground_enemy(commands: &mut Commands, assets: &EnemyAssets) {
    let x = SCREEN_WIDTH / 2.0;
    let y = -(SCREEN_HEIGHT / 2.0) + GROUND_MARGIN + (87.0 / 2.0);

    commands.spawn((
        Sprite {
            image: assets.plant_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.plant_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        Enemy {
            enemy_type: EnemyType::Ground,
            speed_x: 0.0,
            speed_y: 0.0,
            current_frame: 0,
            max_frame: 1,
            width: 60.0,
            height: 87.0,
            angle: 0.0,
            va: 0.0,
        },
        AnimationTimer(Timer::from_seconds(1.0 / 20.0, TimerMode::Repeating)),
    ));
}

fn spawn_climbing_enemy(commands: &mut Commands, assets: &EnemyAssets, rng: &mut impl Rng) {
    let x = SCREEN_WIDTH / 2.0;
    let y = rng.random_range(0.0..(SCREEN_HEIGHT * 0.5)) - (SCREEN_HEIGHT / 2.0);
    let speed_y = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

    commands.spawn((
        Sprite {
            image: assets.spider_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.spider_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0),
        Enemy {
            enemy_type: EnemyType::Climbing,
            speed_x: 0.0,
            speed_y,
            current_frame: 0,
            max_frame: 5,
            width: 120.0,
            height: 144.0,
            angle: 0.0,
            va: 0.0,
        },
        AnimationTimer(Timer::from_seconds(1.0 / 20.0, TimerMode::Repeating)),
    ));
}

fn update_enemies(game_speed: Res<GameSpeed>, mut query: Query<(&mut Enemy, &mut Transform)>) {
    for (mut enemy, mut transform) in &mut query {
        // Horizontal movement: self speedX + world speed
        transform.translation.x -= enemy.speed_x + game_speed.0;

        // Type-specific behaviors
        match enemy.enemy_type {
            EnemyType::Flying => {
                enemy.angle += enemy.va;
                transform.translation.y += enemy.angle.sin();
            }
            EnemyType::Climbing => {
                transform.translation.y += enemy.speed_y;
                let bottom_limit = -(SCREEN_HEIGHT / 2.0) + GROUND_MARGIN + (enemy.height / 2.0);

                if transform.translation.y < bottom_limit {
                    enemy.speed_y *= -1.0;
                }
            }
            _ => {}
        }
    }
}

fn animate_enemies(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Enemy, &mut Sprite)>,
) {
    for (mut timer, mut enemy, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            enemy.current_frame = (enemy.current_frame + 1) % (enemy.max_frame + 1);
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = enemy.current_frame as usize;
            }
        }
    }
}

/// Renders spider web line from top of screen down to spider entity
fn draw_spider_webs(query: Query<(&Enemy, &Transform)>, mut gizmos: Gizmos) {
    for (enemy, transform) in &query {
        if enemy.enemy_type == EnemyType::Climbing {
            let top_pos = Vec2::new(transform.translation.x, SCREEN_HEIGHT / 2.0);
            let spider_pos = Vec2::new(transform.translation.x, transform.translation.y + 50.0);
            gizmos.line_2d(top_pos, spider_pos, Color::WHITE);
        }
    }
}

fn cleanup_offscreen_enemies(mut commands: Commands, query: Query<(Entity, &Transform, &Enemy)>) {
    for (entity, transform, enemy) in &query {
        let off_left = transform.translation.x + (enemy.width / 2.0) < -(SCREEN_WIDTH / 2.0);
        let off_top = transform.translation.y > (SCREEN_HEIGHT / 2.0) + enemy.height;

        if off_left || off_top {
            commands.entity(entity).despawn();
        }
    }
}
