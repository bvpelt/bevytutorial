mod input;
mod player;
mod state;
mod utils;

use bevy::camera::ClearColor;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode, WindowResizeConstraints, WindowResolution};

use input::{LastInput, input_handler_system};
use player::{
    AnimationTimer, Player, player_animation_system, player_physics_system, player_state_system,
};
use utils::{setup_status_ui, update_status_text_system};

pub const WINDOW_HEIGHT: f32 = 720.0;
pub const WINDOW_WIDTH: f32 = 1400.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
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
        .init_resource::<LastInput>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                input_handler_system,
                player_state_system,
                player_physics_system,
                player_animation_system,
                update_status_text_system,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Spawn 2D Camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: WINDOW_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    // Load Dog Spritesheet (Grid: 200x182 per frame, 9 columns, 10 rows)
    let texture: Handle<Image> = asset_server.load("images/dog_spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(200, 182), 9, 10, None, None);
    let atlas_layout = texture_atlas_layouts.add(layout);

    let default_player = Player::default();
    let (initial_frame_y, _) = default_player.state.get_sprite_info();

    // Spawn Player
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout,
                index: initial_frame_y * 9,
            }),
            custom_size: Some(Vec2::new(200.0, 181.83)),
            ..default()
        },
        Transform::from_xyz(0.0, default_player.ground_y, 1.0),
        default_player,
        AnimationTimer(Timer::from_seconds(1.0 / 25.0, TimerMode::Repeating)),
    ));

    // Setup Status Text UI
    setup_status_ui(commands);
}
