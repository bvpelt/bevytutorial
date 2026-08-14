use bevy::prelude::*;
use rand::RngExt;

use crate::GameBounds;
use crate::GameState;

// ----------------------------------------------------------------------------
// Components & Resources
// ----------------------------------------------------------------------------

#[derive(Component)]
pub struct Raven {
    pub width: f32,
    pub height: f32,
    pub direction_x: f32,
    pub direction_y: f32,
    pub color: Color,
    pub has_trail: bool,
}

#[derive(Component)]
pub struct Particle {
    pub radius: f32,
    pub max_radius: f32,
    pub speed_x: f32,
    pub color: Color,
}

#[derive(Component)]
pub struct FlapTimer(pub Timer);

#[derive(Resource)]
pub struct RavenSpawnTimer(pub Timer);

#[derive(Resource)]
pub struct RavenAssets {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

// ----------------------------------------------------------------------------
// Plugin Implementation
// ----------------------------------------------------------------------------

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RavenSpawnTimer(Timer::from_seconds(
            0.5,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup_raven_assets)
        .add_systems(
            Update,
            (
                spawn_ravens,
                update_ravens,
                animate_ravens_and_spawn_particles,
                update_particles,
                check_escaped_ravens,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

// ----------------------------------------------------------------------------
// Systems
// ----------------------------------------------------------------------------

fn setup_raven_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("raven.png");
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(271, 194),
        6,
        1,
        None,
        None,
    ));

    commands.insert_resource(RavenAssets { texture, layout });
}

fn spawn_ravens(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<RavenSpawnTimer>,
    assets: Res<RavenAssets>,
    bounds: Res<GameBounds>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let mut rng = rand::rng();

        let size_modifier: f32 = rng.random_range(0.4..1.0);
        let sprite_width = 271.0;
        let sprite_height = 194.0;
        let width = sprite_width * size_modifier;
        let height = sprite_height * size_modifier;

        // Spawn on the right edge, random Y inside bounds
        let x = bounds.half_width() + (width / 2.0);
        let y_min = -bounds.half_height() + (height / 2.0);
        let y_max = bounds.half_height() - (height / 2.0);
        let y = rng.random_range(y_min..y_max);

        let direction_x = rng.random_range(3.0..8.0) * 60.0; // Scaled to pixels/sec
        let direction_y = rng.random_range(-2.5..2.5) * 60.0;

        let flap_interval = rng.random_range(0.05..0.1);
        let color = Color::srgb(
            rng.random_range(0.2..1.0),
            rng.random_range(0.2..1.0),
            rng.random_range(0.2..1.0),
        );

        // Sorting / Z-indexing: smaller ravens in the back
        let z_index = 1.0 + size_modifier;

        commands.spawn((
            Sprite {
                image: assets.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.layout.clone(),
                    index: 0,
                }),
                color,
                ..default()
            },
            Transform::from_xyz(x, y, z_index).with_scale(Vec3::splat(size_modifier)),
            Raven {
                width,
                height,
                direction_x,
                direction_y,
                color,
                has_trail: rng.random_bool(0.5),
            },
            FlapTimer(Timer::from_seconds(flap_interval, TimerMode::Repeating)),
        ));
    }
}

fn update_ravens(
    time: Res<Time>,
    bounds: Res<GameBounds>,
    mut query: Query<(&mut Raven, &mut Transform)>,
) {
    let dt = time.delta_secs();

    for (mut raven, mut transform) in &mut query {
        // Bounce off top/bottom bounds
        let top_bound = bounds.half_height() - (raven.height / 2.0);
        let bottom_bound = -bounds.half_height() + (raven.height / 2.0);

        if transform.translation.y > top_bound || transform.translation.y < bottom_bound {
            raven.direction_y *= -1.0;
        }

        transform.translation.x -= raven.direction_x * dt;
        transform.translation.y += raven.direction_y * dt;
    }
}

fn animate_ravens_and_spawn_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut FlapTimer, &Raven, &Transform, &mut Sprite)>,
) {
    let mut rng = rand::rng();

    for (mut timer, raven, transform, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (atlas.index + 1) % 6;
            }

            // Spawn trail particles on flap
            if raven.has_trail {
                for _ in 0..5 {
                    let offset_x = rng.random_range(-25.0..25.0);
                    let offset_y = rng.random_range(-25.0..25.0);
                    let radius = rng.random_range(1.0..(raven.width / 10.0).max(2.0));
                    let max_radius = rng.random_range(25.0..45.0);

                    commands.spawn((
                        Sprite {
                            color: raven.color,
                            ..default()
                        },
                        Transform::from_xyz(
                            transform.translation.x + offset_x,
                            transform.translation.y + offset_y,
                            transform.translation.z - 0.1,
                        ),
                        Particle {
                            radius,
                            max_radius,
                            speed_x: rng.random_range(30.0..90.0),
                            color: raven.color,
                        },
                    ));
                }
            }
        }
    }
}

fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Particle, &mut Transform, &mut Sprite)>,
) {
    let dt = time.delta_secs();

    for (entity, mut particle, mut transform, mut sprite) in &mut query {
        transform.translation.x += particle.speed_x * dt;
        particle.radius += 18.0 * dt;

        if particle.radius > particle.max_radius - 5.0 {
            commands.entity(entity).despawn();
        } else {
            // Scale particle transform & fade alpha based on radius ratio
            transform.scale = Vec3::splat(particle.radius);
            let alpha = (1.0 - (particle.radius / particle.max_radius)).clamp(0.0, 1.0);
            sprite.color = particle.color.with_alpha(alpha);
        }
    }
}

fn check_escaped_ravens(
    mut commands: Commands,
    bounds: Res<GameBounds>,
    query: Query<(Entity, &Transform, &Raven)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, transform, raven) in &query {
        if transform.translation.x < -bounds.half_width() - (raven.width / 2.0) {
            commands.entity(entity).despawn();
            next_state.set(GameState::GameOver);
        }
    }
}
