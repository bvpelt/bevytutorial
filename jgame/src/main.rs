mod background;
mod collisions;
mod enemies;
mod input;
mod player;

use background::BackgroundPlugin;
use bevy::prelude::*;
use collisions::CollisionsPlugin;
use enemies::EnemyPlugin;
use input::InputPlugin;
use player::PlayerPlugin;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JGame - Bevy Parallax & Animation & Enemies".into(),
                resolution: (800, 700).into(),
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
        .run();
}
