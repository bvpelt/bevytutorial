use crate::GameBounds;
use bevy::prelude::*;
use rand::RngExt;
use uuid::Uuid;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup_enemy_assets)
        .add_systems(
            Update,
            (
                spawn_enemies,
                animate_enemies,
                update_worms,
                update_ghosts,
                update_spiders,
                draw_spider_webs,
                cleanup_offscreen_enemies,
            ),
        );
    }
}

#[derive(Resource)]
pub struct EnemyAssets {
    pub worm_texture: Handle<Image>,
    pub worm_layout: Handle<TextureAtlasLayout>,
    pub ghost_texture: Handle<Image>,
    pub ghost_layout: Handle<TextureAtlasLayout>,
    pub spider_texture: Handle<Image>,
    pub spider_layout: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

#[derive(Debug, Component)]
pub struct Enemy {
    pub animation_timer: Timer,
    pub frame_index: usize,
    pub max_frames: usize,
    pub name: String,
}

#[derive(Component)]
pub struct Worm {
    pub vx: f32,
    pub width: f32,
}

#[derive(Component)]
pub struct Ghost {
    pub vx: f32,
    pub angle: f32,
    pub curve: f32,
    pub width: f32,
}

#[derive(Component)]
pub struct Spider {
    pub vy: f32,
    pub max_length: f32,
    pub height: f32,
}

fn spawn_enemy_worm(
    commands: &mut Commands,
    assets: &EnemyAssets,
    rng: &mut impl rand::Rng,
    bounds: &GameBounds,
) {
    let sprite_width = 80.0;
    let sprite_height = 60.0;
    let width = sprite_width * 0.5;
    let height = sprite_height * 0.5;

    let vx = rng.random_range(100.0..200.0);
    let x = bounds.half_width() + width;
    let y = -bounds.half_height() + height / 2.0;

    commands.spawn((
        Sprite {
            image: assets.worm_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.worm_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x, y, 1.0).with_scale(Vec3::splat(0.5)),
        Enemy {
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_index: 0,
            max_frames: 5,
            name: format!("Worm_{}", Uuid::new_v4().urn()),
        },
        Worm { vx, width },
    ));
}

fn spawn_enemy_ghost(
    commands: &mut Commands,
    assets: &EnemyAssets,
    rng: &mut impl rand::Rng,
    bounds: &GameBounds,
) {
    let sprite_width = 261.0;
    let width = sprite_width * 0.5;

    let vx = rng.random_range(100.0..300.0);
    let x = bounds.half_width() + width;
    let y_range = bounds.height * 0.6;
    let y = bounds.half_height() - rng.random_range(0.0..y_range);

    commands.spawn((
        Sprite {
            image: assets.ghost_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.ghost_layout.clone(),
                index: 0,
            }),
            color: Color::srgba(1.0, 1.0, 1.0, 0.7),
            ..default()
        },
        Transform::from_xyz(x, y, 2.0).with_scale(Vec3::splat(0.5)),
        Enemy {
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_index: 0,
            max_frames: 5,
            name: format!("Ghost_{}", Uuid::new_v4().urn()),
        },
        Ghost {
            vx,
            angle: 0.0,
            curve: rng.random_range(0.5..3.0),
            width,
        },
    ));
}

fn spawn_enemy_spider(
    commands: &mut Commands,
    assets: &EnemyAssets,
    rng: &mut impl rand::Rng,
    bounds: &GameBounds,
) {
    let sprite_width = 310.0;
    let sprite_height = 175.0;
    let width = sprite_width * 0.5;
    let height = sprite_height * 0.5;

    let vy = rng.random_range(100.0..200.0);
    let x = rng.random_range(-bounds.half_width() + width..bounds.half_width() - width);
    let y = bounds.half_height() + height;
    let max_length = bounds.half_height() - rng.random_range(50.0..bounds.height);

    commands.spawn((
        Sprite {
            image: assets.spider_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.spider_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(x, y, 3.0).with_scale(Vec3::splat(0.5)),
        Enemy {
            animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            frame_index: 0,
            max_frames: 5,
            name: format!("Spider_{}", Uuid::new_v4().urn()),
        },
        Spider {
            vy,
            max_length,
            height,
        },
    ));
}

fn spawn_enemies(
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    assets: Res<EnemyAssets>,
    bounds: Res<GameBounds>,
    mut commands: Commands,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let mut rng = rand::rng();
    let enemy_type = rng.random_range(0..3);

    match enemy_type {
        0 => spawn_enemy_worm(&mut commands, &assets, &mut rng, &bounds),
        1 => spawn_enemy_ghost(&mut commands, &assets, &mut rng, &bounds),
        2 => spawn_enemy_spider(&mut commands, &assets, &mut rng, &bounds),
        _ => {}
    }
}

fn setup_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let worm_texture = asset_server.load("enemy_worm.png");
    let worm_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(80, 60),
        6,
        1,
        None,
        None,
    ));

    let ghost_texture = asset_server.load("enemy_ghost.png");
    let ghost_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(261, 209),
        6,
        1,
        None,
        None,
    ));

    let spider_texture = asset_server.load("enemy_spider.png");
    let spider_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(310, 175),
        6,
        1,
        None,
        None,
    ));

    commands.insert_resource(EnemyAssets {
        worm_texture,
        worm_layout,
        ghost_texture,
        ghost_layout,
        spider_texture,
        spider_layout,
    });
}

fn animate_enemies(time: Res<Time>, mut query: Query<(&mut Enemy, &mut Sprite)>) {
    for (mut enemy, mut sprite) in &mut query {
        enemy.animation_timer.tick(time.delta());
        if enemy.animation_timer.just_finished() {
            enemy.frame_index = (enemy.frame_index + 1) % (enemy.max_frames + 1);
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = enemy.frame_index;
            }
        }
    }
}

fn update_worms(time: Res<Time>, mut query: Query<(&Worm, &mut Transform)>) {
    for (worm, mut transform) in &mut query {
        transform.translation.x -= worm.vx * time.delta_secs();
    }
}

fn update_ghosts(time: Res<Time>, mut query: Query<(&mut Ghost, &mut Transform)>) {
    for (mut ghost, mut transform) in &mut query {
        transform.translation.x -= ghost.vx * time.delta_secs();
        transform.translation.y += ghost.angle.sin() * ghost.curve;
        ghost.angle += 0.04;
    }
}

fn update_spiders(time: Res<Time>, mut query: Query<(&mut Spider, &mut Transform)>) {
    for (mut spider, mut transform) in &mut query {
        transform.translation.y -= spider.vy * time.delta_secs();

        if transform.translation.y < spider.max_length {
            spider.vy = -spider.vy.abs();
        }
    }
}

fn draw_spider_webs(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Spider)>,
    bounds: Res<GameBounds>,
) {
    for (transform, _spider) in &query {
        let top_pos = Vec2::new(transform.translation.x, bounds.half_height());
        let spider_pos = Vec2::new(transform.translation.x, transform.translation.y + 10.0);
        gizmos.line_2d(top_pos, spider_pos, Color::WHITE);
    }
}

fn cleanup_offscreen_enemies(
    mut commands: Commands,
    bounds: Res<GameBounds>,
    worm_query: Query<(Entity, &Transform, &Worm)>,
    ghost_query: Query<(Entity, &Transform, &Ghost)>,
    spider_query: Query<(Entity, &Transform, &Spider)>,
) {
    for (entity, transform, worm) in &worm_query {
        if transform.translation.x < -bounds.half_width() - worm.width {
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform, ghost) in &ghost_query {
        if transform.translation.x < -bounds.half_width() - ghost.width {
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform, spider) in &spider_query {
        if transform.translation.y > bounds.half_height() + spider.height * 2.0 {
            commands.entity(entity).despawn();
        }
    }
}
