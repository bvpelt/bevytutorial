mod background;
mod enemies;

use background::BackgroundPlugin;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::{
    PresentMode, WindowMode, WindowResizeConstraints, WindowResized, WindowResolution,
};
use enemies::*;

pub const WINDOW_HEIGHT: f32 = 700.0;

#[derive(Resource, Debug, Clone, Copy)]
pub struct GameBounds {
    pub width: f32,
    pub height: f32,
}

impl Default for GameBounds {
    fn default() -> Self {
        Self {
            width: 800.0,
            height: WINDOW_HEIGHT,
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

pub fn handle_window_resize(
    mut resize_reader: MessageReader<WindowResized>,
    mut windows: Query<&mut Window>,
    mut game_bounds: ResMut<GameBounds>,
) {
    for event in resize_reader.read() {
        // Force height back to 700 if the OS window manager allowed a vertical resize
        if (event.height - WINDOW_HEIGHT).abs() > 1.0 {
            if let Ok(mut window) = windows.get_mut(event.window) {
                window.resolution.set(event.width, WINDOW_HEIGHT);
            }
        }

        game_bounds.width = event.width;
        game_bounds.height = WINDOW_HEIGHT;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JGame - Bevy Parallax & Animation & Enemies".into(),
                resolution: WindowResolution::new(800, WINDOW_HEIGHT as u32),
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
        .init_resource::<GameBounds>()
        .add_plugins((BackgroundPlugin, EnemyPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_window_resize)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: WINDOW_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
