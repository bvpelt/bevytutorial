mod background;
mod collisions;
mod enemies;
mod input;
mod player;
mod score;

use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::window::{PresentMode, WindowMode, WindowResized};
use collisions::CollisionsPlugin;
use enemies::EnemyPlugin;
use input::InputPlugin;
use player::PlayerPlugin;
use score::Score; // <-- Import Score resource

// ----------------------------------------------------------------------------
// States & Resources
// ----------------------------------------------------------------------------

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    InGame,
    GameOver,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct GameBounds {
    pub width: f32,
    pub height: f32,
}

impl Default for GameBounds {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: 700.0,
        }
    }
}

impl GameBounds {
    pub fn half_width(&self) -> f32 {
        self.width / 2.0
    }

    pub fn half_height(&self) -> f32 {
        self.height / 2.0
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct GameOverUI;

// ----------------------------------------------------------------------------
// Systems
// ----------------------------------------------------------------------------

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_ui(mut commands: Commands) {
    // Score display top-left corner
    commands.spawn((
        Text::new("Score: 0 | Hits: 0/3"),
        TextFont {
            font_size: FontSize::Px(30.0), // Fixed FontSize type
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        ScoreText,
    ));
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        for mut text in &mut query {
            **text = format!(
                "Score: {} | Hits: {}/{}",
                score.value, score.collisions, score.max_collisions
            );
        }
    }
}

fn check_game_over(score: Res<Score>, mut next_state: ResMut<NextState<GameState>>) {
    if score.collisions >= score.max_collisions {
        next_state.set(GameState::GameOver);
    }
}

fn show_game_over_ui(mut commands: Commands, score: Res<Score>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: FontSize::Px(60.0), // Fixed FontSize type
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
            ));
            parent.spawn((
                Text::new(format!("Final Score: {}", score.value)),
                TextFont {
                    font_size: FontSize::Px(40.0), // Fixed FontSize type
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn handle_window_resize(
    mut resize_reader: MessageReader<WindowResized>,
    mut game_bounds: ResMut<GameBounds>,
) {
    for event in resize_reader.read() {
        game_bounds.width = event.width;
        game_bounds.height = event.height;
    }
}

// ----------------------------------------------------------------------------
// Main Application
// ----------------------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JGame - Bevy Parallax & Animation & Enemies".into(),
                window_level: bevy::window::WindowLevel::Normal,
                mode: WindowMode::Windowed,
                resizable: true,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .init_resource::<GameBounds>()
        .init_resource::<Score>()
        // Feature Plugins
        .add_plugins((
            PlayerPlugin,
            BackgroundPlugin,
            EnemyPlugin,
            InputPlugin,
            CollisionsPlugin,
        ))
        // Environment Systems
        .add_systems(Startup, (setup_camera, setup_ui))
        .add_systems(
            Update,
            (
                handle_window_resize,
                update_score_text,
                check_game_over.run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(OnEnter(GameState::GameOver), show_game_over_ui)
        .run();
}
