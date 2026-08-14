use bevy::prelude::*;
use rand::RngExt;

use crate::GameState;
use crate::enemies::Raven;
use crate::score::Score;

// ----------------------------------------------------------------------------
// Components & Resources
// ----------------------------------------------------------------------------

#[derive(Component)]
pub struct Explosion {
    pub current_frame: u32,
    pub max_frame: u32,
}

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

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
                (handle_mouse_click_shooting, animate_explosions)
                    .run_if(in_state(GameState::InGame)),
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
        UVec2::new(200, 179),
        6,
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

fn handle_mouse_click_shooting(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    raven_query: Query<(Entity, &Transform, &Raven)>,
    assets: Res<CollisionAssets>,
    mut score: ResMut<Score>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    // Convert screen cursor position to 2D world coordinates
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };
    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    for (entity, transform, raven) in &raven_query {
        let raven_pos = transform.translation.truncate();
        let half_width = raven.width / 2.0;
        let half_height = raven.height / 2.0;

        // Check AABB click collision
        if world_position.x >= raven_pos.x - half_width
            && world_position.x <= raven_pos.x + half_width
            && world_position.y >= raven_pos.y - half_height
            && world_position.y <= raven_pos.y + half_height
        {
            // 1. Despawn clicked Raven
            commands.entity(entity).despawn();

            // 2. Increment score
            score.value += 1;

            // 3. Spawn Explosion visual & sound
            spawn_explosion(&mut commands, &assets, transform.translation, raven.width);

            // Break after hitting top raven (or omit break to hit overlapping ravens)
            break;
        }
    }
}

fn spawn_explosion(commands: &mut Commands, assets: &CollisionAssets, position: Vec3, size: f32) {
    let scale = size / 200.0; // Scale relative to sprite base width (200px)

    commands.spawn((
        Sprite {
            image: assets.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(position.x, position.y - (size / 4.0), 10.0))
            .with_scale(Vec3::splat(scale)),
        Explosion {
            current_frame: 0,
            max_frame: 5,
        },
        ExplosionTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
    ));

    // Play boom sound effect
    commands.spawn(AudioPlayer(assets.boom_sound.clone()));
}

fn animate_explosions(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut Explosion, &mut Sprite)>,
) {
    for (entity, mut timer, mut explosion, mut sprite) in &mut query {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            explosion.current_frame += 1;

            if explosion.current_frame > explosion.max_frame {
                commands.entity(entity).despawn();
            } else if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = explosion.current_frame as usize;
            }
        }
    }
}
