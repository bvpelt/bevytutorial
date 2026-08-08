mod background;
mod collisions;
mod enemies;
mod input;
mod player;

use background::BackgroundPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode, WindowResized};
use collisions::CollisionsPlugin;
use enemies::EnemyPlugin;
use input::InputPlugin;
use player::PlayerPlugin;

// ----------------------------------------------------------------------------
// Resources
// ----------------------------------------------------------------------------

/// Standard Bevy Resource to replace mutable static variables safely.
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

// ----------------------------------------------------------------------------
// Systems
// ----------------------------------------------------------------------------

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// Sets up initial game bounds resource based on the launched window size.
fn window_setup(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut game_bounds: ResMut<GameBounds>,
) {
    if let Ok(window) = window_query.single() {
        game_bounds.width = window.width();
        game_bounds.height = window.height();

        info!(
            "Window initialized: width = {}, height = {}, half_width = {}, half_height = {}",
            game_bounds.width,
            game_bounds.height,
            game_bounds.half_width(),
            game_bounds.half_height()
        );
    }
}

/// System that automatically updates GameBounds whenever the user resizes the window.
/// Uses `MessageReader` for Bevy 0.16+ / 0.19.
pub fn handle_window_resize(
    mut resize_reader: MessageReader<WindowResized>,
    mut game_bounds: ResMut<GameBounds>,
) {
    for event in resize_reader.read() {
        game_bounds.width = event.width;
        game_bounds.height = event.height;

        info!(
            "Window resized: width = {}, height = {}, half_width = {}, half_height = {}",
            game_bounds.width,
            game_bounds.height,
            game_bounds.half_width(),
            game_bounds.half_height()
        );
    }
}

// ----------------------------------------------------------------------------
// Main Application
// ----------------------------------------------------------------------------

fn main() {
    App::new()
        // Register the GameBounds resource with default fallback values
        .init_resource::<GameBounds>()
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
        // Feature Plugins
        .add_plugins((
            PlayerPlugin,
            BackgroundPlugin,
            EnemyPlugin,
            InputPlugin,
            CollisionsPlugin,
        ))
        // Environment Systems
        .add_systems(Startup, setup_camera)
        .add_systems(Update, handle_window_resize)
        .run();
}
