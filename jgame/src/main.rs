mod background;
mod components;
mod enemy;
mod player;

use background::background_scroll_system;
use bevy::window::MonitorSelection::Current;
use components::*;
use enemy::{enemy_movement_system, enemy_spawner_system};
use player::{player_animation_system, player_movement_system};

use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::{
    PresentMode, PrimaryWindow, WindowMode, WindowResizeConstraints, WindowResolution,
};

pub const WINDOW_HEIGHT: f32 = 700.0;
pub const WINDOW_WIDTH: f32 = 2400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JGame - Bevy Parallax & Animation & Enemies".into(),
                resolution: WindowResolution::new(1200, WINDOW_HEIGHT as u32),
                resize_constraints: WindowResizeConstraints {
                    min_height: WINDOW_HEIGHT,
                    max_height: WINDOW_HEIGHT,
                    ..default()
                },
                window_level: bevy::window::WindowLevel::Normal,
                mode: WindowMode::Windowed,
                resizable: true,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(Score(0))
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement_system,
                player_animation_system,
                enemy_spawner_system,
                enemy_movement_system,
                background_scroll_system,
                collision_system,
                restart_game_system,
                toggle_fullscreen_system,
                update_ui_system,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 2D Camera with fixed vertical scaling mode (700 units tall)
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: WINDOW_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Load Sprites and Layouts
    let player_texture: Handle<Image> = asset_server.load("images/player.png");
    let bg_texture: Handle<Image> = asset_server.load("images/background_single.png");
    let enemy_texture: Handle<Image> = asset_server.load("images/enemy_1.png");

    let player_layout = TextureAtlasLayout::from_grid(UVec2::new(200, 200), 9, 2, None, None);
    let player_atlas_layout = texture_atlas_layouts.add(player_layout);

    let enemy_layout = TextureAtlasLayout::from_grid(UVec2::new(160, 119), 6, 1, None, None);
    let enemy_atlas_layout = texture_atlas_layouts.add(enemy_layout);

    commands.insert_resource(GameAssets {
        player_atlas_layout: player_atlas_layout.clone(),
        enemy_atlas_layout: enemy_atlas_layout.clone(),
        player_texture: player_texture.clone(),
        bg_texture: bg_texture.clone(),
        enemy_texture,
    });

    // Spawn Backgrounds (seamless tiling pair for 2400x700 canvas)
    let bg_width = WINDOW_WIDTH;
    for i in 0..2 {
        commands.spawn((
            Sprite {
                image: bg_texture.clone(),
                custom_size: Some(Vec2::new(bg_width, 700.0)),
                ..default()
            },
            Transform::from_xyz(i as f32 * bg_width, 0.0, 0.0),
            Background {
                speed: 100.0,
                width: bg_width,
            },
        ));
    }

    // Spawn Player
    let ground_y = -350.0 + 100.0;
    commands.spawn((
        Sprite {
            image: player_texture,
            texture_atlas: Some(TextureAtlas {
                layout: player_atlas_layout,
                index: 0,
            }),
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        Transform::from_xyz(-250.0, ground_y, 1.0),
        Player {
            vy: 0.0,
            weight: 1500.0,
            ground_y,
            is_grounded: true,
            speed: 300.0,
        },
        AnimationIndices { first: 0, last: 8 },
        AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
    ));

    // UI Overlay Score
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: FontSize::Px(40.0),
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        ScoreText,
    ));

    // UI Game Over Banner
    commands.spawn((
        Text::new("GAME OVER, press Enter to restart!"),
        TextFont {
            font_size: FontSize::Px(44.0),
            ..default()
        },
        TextColor(Color::NONE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(300.0),
            left: Val::Percent(20.0),
            ..default()
        },
        GameOverText,
    ));
}

fn collision_system(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
) {
    if *game_state.get() != GameState::Playing {
        return;
    }

    if let Ok(player_transform) = player_query.single() {
        let player_pos = player_transform.translation.truncate();

        for enemy_transform in enemy_query.iter() {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = player_pos.distance(enemy_pos);

            if distance < 80.0 {
                next_state.set(GameState::GameOver);
                break;
            }
        }
    }
}

fn restart_game_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut commands: Commands,
    mut game_over_query: Query<(&mut Text, &mut TextColor), With<GameOverText>>,
) {
    if *game_state.get() == GameState::GameOver && keyboard_input.just_pressed(KeyCode::Enter) {
        // 1. Reset Score
        score.0 = 0;

        // 2. Despawn active enemies
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn();
        }

        // 3. Reset Player position and movement variables
        if let Ok((mut transform, mut player)) = player_query.single_mut() {
            transform.translation.x = -250.0;
            transform.translation.y = player.ground_y;
            player.vy = 0.0;
            player.is_grounded = true;
        }

        // 4. Hide Game Over text
        if let Ok((_, mut text_color)) = game_over_query.single_mut() {
            text_color.0 = Color::NONE;
        }

        // 5. Resume Game State
        next_state.set(GameState::Playing);
    }
}

fn toggle_fullscreen_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::F11) {
        if let Ok(mut window) = window_query.single_mut() {
            window.mode = match window.mode {
                WindowMode::Windowed => WindowMode::BorderlessFullscreen(Current),
                _ => WindowMode::Windowed,
            };
        }
    }
}

fn update_ui_system(
    score: Res<Score>,
    game_state: Res<State<GameState>>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<GameOverText>)>,
    mut game_over_query: Query<(&mut Text, &mut TextColor), With<GameOverText>>,
) {
    if let Ok(mut score_text) = score_query.single_mut() {
        **score_text = format!("Score: {}", score.0);
    }

    if *game_state.get() == GameState::GameOver {
        if let Ok((_, mut text_color)) = game_over_query.single_mut() {
            text_color.0 = Color::srgb(0.8, 0.1, 0.1);
        }
    }
}
