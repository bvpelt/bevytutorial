use bevy::prelude::*;

const SPRITE_WIDTH: f32 = 575.0;
const SPRITE_HEIGHT: f32 = 523.0;
const TEXTURE_COLS: u32 = 12;
const TEXTURE_ROWS: u32 = 10;

const LAYER_WIDTH: f32 = 2400.0;

// ----------------------------------------------------------------------------
// Components & Resources
// ----------------------------------------------------------------------------

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerState {
    Idle,
    Jump,
    Fall,
    Run,
    Dizzy,
    Sit,
    Roll,
    Bite,
    Ko,
    GetHit,
}

impl PlayerState {
    fn row_index(&self) -> u32 {
        match self {
            PlayerState::Idle => 0,
            PlayerState::Jump => 1,
            PlayerState::Fall => 2,
            PlayerState::Run => 3,
            PlayerState::Dizzy => 4,
            PlayerState::Sit => 5,
            PlayerState::Roll => 6,
            PlayerState::Bite => 7,
            PlayerState::Ko => 8,
            PlayerState::GetHit => 9,
        }
    }

    fn frame_count(&self) -> u32 {
        match self {
            PlayerState::Idle => 7,
            PlayerState::Jump => 7,
            PlayerState::Fall => 7,
            PlayerState::Run => 9,
            PlayerState::Dizzy => 11,
            PlayerState::Sit => 5,
            PlayerState::Roll => 7,
            PlayerState::Bite => 7,
            PlayerState::Ko => 12,
            PlayerState::GetHit => 4,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Player {
    pub current_frame: u32,
}

#[derive(Resource)]
pub struct SpriteSheetLayout(pub Handle<TextureAtlasLayout>);

/// Resource controlling overall game speed (equivalent to JS `gameSpeed`)
#[derive(Resource)]
pub struct GameSpeed(pub f32);

/// Component attached to parallax layers to control scrolling speed relative to base game speed
#[derive(Component)]
pub struct ParallaxLayer {
    pub speed_modifier: f32,
}

// ----------------------------------------------------------------------------
// Systems
// ----------------------------------------------------------------------------

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // 1. Spawn 2D Camera
    commands.spawn(Camera2d);

    // 2. Spawn Parallax Background Layers
    let background_configs = [
        ("layer-1.png", 0.2, -5.0),
        ("layer-2.png", 0.4, -4.0),
        ("layer-3.png", 0.6, -3.0),
        ("layer-4.png", 0.8, -2.0),
        ("layer-5.png", 1.0, -1.0),
    ];

    for (image_path, speed_modifier, z_index) in background_configs {
        let texture: Handle<Image> = asset_server.load(image_path);

        // Spawn two copies side-by-side to allow seamless infinite looping
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

    // 3. Load Player Sprite Sheet Image
    let texture = asset_server.load("shadow_dog.png");

    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_WIDTH as u32, SPRITE_HEIGHT as u32),
        TEXTURE_COLS,
        TEXTURE_ROWS,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(SpriteSheetLayout(layout_handle.clone()));

    let initial_state = PlayerState::Run;
    let initial_index = (initial_state.row_index() * TEXTURE_COLS) as usize;

    // 4. Spawn Player Entity (at Z = 0.0, in front of background layers)
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: initial_index,
            }),
            ..default()
        },
        // Scaled down slightly if desired to fit the 800x700 viewport cleanly
        Transform::from_xyz(0.0, -150.0, 0.0).with_scale(Vec3::splat(0.6)),
        Player { current_frame: 0 },
        initial_state,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
    ));
}

/// System to handle parallax movement and infinite wrapping
fn scroll_parallax(
    time: Res<Time>,
    game_speed: Res<GameSpeed>,
    mut query: Query<(&ParallaxLayer, &mut Transform)>,
) {
    for (layer, mut transform) in &mut query {
        // Move left based on Delta Time * Base Speed * Speed Modifier
        let movement = game_speed.0 * layer.speed_modifier * 10.0 * time.delta_secs();
        transform.translation.x -= movement;

        // Reset position once a panel moves completely past the left seam
        if transform.translation.x <= -LAYER_WIDTH {
            transform.translation.x += LAYER_WIDTH * 2.0;
        }
    }
}

/// System to adjust gameSpeed dynamically with keyboard controls (+ / - or Up/Down arrows)
fn update_game_speed(keyboard_input: Res<ButtonInput<KeyCode>>, mut game_speed: ResMut<GameSpeed>) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        game_speed.0 += 5.0;
        info!("Game speed increased to: {}", game_speed.0);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS)
    {
        game_speed.0 = (game_speed.0 - 5.0).max(0.0);
        info!("Game speed decreased to: {}", game_speed.0);
    }
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Player, &PlayerState, &mut Sprite), With<Player>>,
) {
    for (mut timer, mut player, state, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            player.current_frame = (player.current_frame + 1) % state.frame_count();
            let base_index = state.row_index() * TEXTURE_COLS;

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (base_index + player.current_frame) as usize;
            }
        }
    }
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerState, &mut Player, &mut Sprite), With<Player>>,
) {
    let mut new_state = None;

    if keyboard_input.just_pressed(KeyCode::Numpad5) {
        new_state = Some(PlayerState::Idle);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad8) {
        new_state = Some(PlayerState::Jump);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad2) {
        new_state = Some(PlayerState::Fall);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad6) {
        new_state = Some(PlayerState::Run);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad3) {
        new_state = Some(PlayerState::Dizzy);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad1) {
        new_state = Some(PlayerState::Sit);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad7) {
        new_state = Some(PlayerState::Roll);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad4) {
        new_state = Some(PlayerState::Bite);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad9) {
        new_state = Some(PlayerState::Ko);
    }
    if keyboard_input.just_pressed(KeyCode::Numpad0) {
        new_state = Some(PlayerState::GetHit);
    }

    if let Some(state) = new_state {
        for (mut current_state, mut player, mut sprite) in &mut query {
            if *current_state != state {
                *current_state = state;
                player.current_frame = 0;

                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = (state.row_index() * TEXTURE_COLS) as usize;
                }

                info!("Switched animation to: {:?}", state);
            }
        }
    }
}

// ----------------------------------------------------------------------------
// App Entry Point
// ----------------------------------------------------------------------------

fn main() {
    App::new()
        .insert_resource(GameSpeed(15.0)) // Initial game speed matching your JS variable
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JGame - Bevy Parallax & Animation".into(),
                resolution: (800, 700).into(), // Adjusted height to match 700px canvas
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                animate_player,
                handle_input,
                scroll_parallax,
                update_game_speed,
            ),
        )
        .run();
}
