use bevy::prelude::*;
use rand::Rng;
use rand::RngExt;
use uuid::Uuid;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

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

/// Resource storing handles to enemy sprite sheets and layouts
#[derive(Resource)]
pub struct EnemyAssets {
    pub worm_texture: Handle<Image>,
    pub worm_layout: Handle<TextureAtlasLayout>,
    pub ghost_texture: Handle<Image>,
    pub ghost_layout: Handle<TextureAtlasLayout>,
    pub spider_texture: Handle<Image>,
    pub spider_layout: Handle<TextureAtlasLayout>,
}

// Resource to manage enemy spawn interval (equivalent to enemyInterval = 500ms)
#[derive(Resource)]
pub struct EnemySpawnTimer(pub Timer);

// Base marker and frame animation data
#[derive(Debug, Component)]
pub struct Enemy {
    pub animation_timer: Timer,
    pub frame_index: usize,
    pub max_frames: usize,
    pub name: String,
}

// Enemy type components
#[derive(Component)]
pub struct Worm {
    pub vx: f32,
    pub width: f32,
    //    pub height: f32,
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
    //    pub width: f32,
    pub height: f32,
}

fn spawn_enemy_worm(commands: &mut Commands, assets: &EnemyAssets, rng: &mut impl Rng) {
    // Worm: Spawns at bottom right, moves left
    let sprite_width = 229.0;
    let sprite_height = 171.0;
    let width = sprite_width / 2.0;
    let height = sprite_height / 2.0;

    let vx = (1.0 + rng.random_range(0.0..1.0)) * 0.1;
    let x = WINDOW_WIDTH; // / 2.0 + width / 2.0;
    let y = height; //-WINDOW_HEIGHT / 2.0 + height / 2.0;

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
            animation_timer: Timer::from_seconds(0.05, TimerMode::Repeating),
            frame_index: 0,
            max_frames: 5,
            name: format!("Worm_{}", Uuid::new_v4().urn()),
        },
        Worm {
            vx,
            width, /*, height  */
        },
    ));
}

fn spawn_enemy_ghost(commands: &mut Commands, assets: &EnemyAssets, rng: &mut impl Rng) {
    let sprite_width = 261.0;
    //    let sprite_height = 209.0;
    let width = sprite_width / 2.0;

    let vx = rng.random_range(0.0..1.0) * 0.2 + 0.1; //rng.random_range(0.1..0.3) * 100.0;
    let x = WINDOW_WIDTH / 2.0 + width / 2.0;
    let y_range = WINDOW_HEIGHT * 0.6;
    let y = (WINDOW_HEIGHT / 2.0) - rng.random_range(0.0..y_range);

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

fn spawn_enemy_spider(commands: &mut Commands, assets: &EnemyAssets, rng: &mut impl Rng) {
    let sprite_width = 310.0;
    let sprite_height = 175.0;
    //    let width = sprite_width / 2.0;
    let height = sprite_height / 2.0;

    let vy = rng.random_range(0.1..0.2) * 100.0;
    let x = rng.random_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
    let y = WINDOW_HEIGHT / 2.0 + height;
    let max_length = (WINDOW_HEIGHT / 2.0) - rng.random_range(50.0..WINDOW_HEIGHT);

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
            // width,
            height,
        },
    ));
}

fn spawn_enemies(
    time: Res<Time>,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    //enemy_assets: Option<Res<EnemyAssets>>,
    assets: Res<EnemyAssets>,
    mut commands: Commands,
) {
    /*
    let Some(assets) = enemy_assets else {
        error!("no resources loaded");
        return;
    };
     */

    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        //info!("Spawn timer just finished");
        return;
    }

    let mut rng = rand::rng();
    let enemy_type = rng.random_range(0..3);

    //info!("Using enemy_type: {:?}", enemy_type);

    match enemy_type {
        0 => {
            // Spawn Worm Enemy
            spawn_enemy_worm(&mut commands, &assets, &mut rng);
        }
        1 => {
            // Spawn Ghost Enemy
            spawn_enemy_ghost(&mut commands, &assets, &mut rng);
        }
        2 => {
            // Spawn Spider Enemy
            spawn_enemy_spider(&mut commands, &assets, &mut rng);
        }
        _ => {}
    }
}

fn setup_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Worm Enemy: 70x60 (6 frames)
    let worm_texture = asset_server.load("enemy_worm.png");
    let worm_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(70, 60),
        6,
        1,
        None,
        None,
    ));

    // Ghost Enemy (Plant): 1566/6= 261x209  60x87 (2 frames)
    let ghost_texture = asset_server.load("enemy_ghost.png");
    let ghost_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(261, 209),
        6,
        1,
        None,
        None,
    ));

    // Climbing Enemy (Spider): 1860/6 = 310x175 (6 frames)
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
            spider.vy *= -1.0;
        }
    }
}

fn draw_spider_webs(mut gizmos: Gizmos, query: Query<(&Transform, &Spider)>) {
    for (transform, _spider) in &query {
        let top_pos = Vec2::new(transform.translation.x, WINDOW_HEIGHT / 2.0);
        let spider_pos = Vec2::new(transform.translation.x, transform.translation.y + 10.0);
        gizmos.line_2d(top_pos, spider_pos, Color::WHITE);
    }
}

fn cleanup_offscreen_enemies(
    mut commands: Commands,
    worm_query: Query<(Entity, &Transform, &Worm)>,
    ghost_query: Query<(Entity, &Transform, &Ghost)>,
    spider_query: Query<(Entity, &Transform, &Spider)>,
) {
    for (entity, transform, worm) in &worm_query {
        if transform.translation.x < -WINDOW_WIDTH / 2.0 - worm.width {
            info!("Worm removed {:?}", entity);
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform, ghost) in &ghost_query {
        if transform.translation.x < -WINDOW_WIDTH / 2.0 - ghost.width {
            info!("Ghost removed {:?}", entity);
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform, spider) in &spider_query {
        if transform.translation.y > WINDOW_HEIGHT / 2.0 + spider.height * 2.0 {
            info!("Spider removed {:?}", entity);
            commands.entity(entity).despawn();
        }
    }
}
