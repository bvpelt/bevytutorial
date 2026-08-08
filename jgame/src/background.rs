use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const LAYER_WIDTH: f32 = 2400.0;

#[derive(Resource)]
pub struct GameSpeed(pub f32);

#[derive(Component)]
pub struct ParallaxLayer {
    pub speed_modifier: f32,
}

/// Resource holding asset handles for the layers we want to measure
#[derive(Resource)]
pub struct BackgroundLayerHandles {
    pub constraints_applied: bool,
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameSpeed(15.0))
            .add_systems(Startup, setup_background)
            .add_systems(Update, (enforce_min_window_height, scroll_parallax));
    }
}

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Store handles in dynamic resource
    commands.insert_resource(BackgroundLayerHandles {
        constraints_applied: false,
    });

    let background_configs = [
        ("layer-1.png", 0.2, -5.0),
        ("layer-2.png", 0.4, -4.0),
        ("layer-3.png", 0.6, -3.0),
        ("layer-4.png", 0.8, -2.0),
        ("layer-5.png", 1.0, -1.0),
    ];

    for (image_path, speed_modifier, z_index) in background_configs {
        let texture: Handle<Image> = asset_server.load(image_path);

        for copy_index in 0..2 {
            let initial_x = (copy_index as f32) * LAYER_WIDTH;

            commands.spawn((
                Sprite {
                    image: texture.clone(),
                    ..default()
                },
                Transform::from_xyz(initial_x, 0.0, z_index),
                ParallaxLayer { speed_modifier },
            ));
        }
    }
}

fn scroll_parallax(
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut query: Query<(&ParallaxLayer, &mut Transform)>,
) {
    for (layer, mut transform) in &mut query {
        let movement = game_speed.0 * layer.speed_modifier * 10.0 * time.delta_secs();
        transform.translation.x -= movement;

        if transform.translation.x <= -LAYER_WIDTH {
            transform.translation.x += LAYER_WIDTH * 2.0;
        }
    }
}

/// Checks image metadata once loaded and updates primary window constraints dynamically.
fn enforce_min_window_height(
    images: Res<Assets<Image>>,
    mut layer_handles: ResMut<BackgroundLayerHandles>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if layer_handles.constraints_applied {
        return;
    }

    let min_height = 720.0; // fixed value in all layer-*.png
    if let Ok(mut window) = window_query.single_mut() {
        window.resize_constraints.min_height = min_height;

        // If current window is smaller than min_height, clamp it right now
        if window.height() < min_height {
            let current_width = window.width();
            window.resolution.set(current_width, min_height);
        }

        info!(
            "Background layers loaded. Enforced min window height: {} px",
            min_height
        );

        layer_handles.constraints_applied = true;
    }
}
