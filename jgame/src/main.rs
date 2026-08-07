use bevy::prelude::*;

const SPRITE_WIDTH: f32 = 575.0;
const SPRITE_HEIGHT: f32 = 523.0;
const TEXTURE_COLS: u32 = 12;
const TEXTURE_ROWS: u32 = 10;

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
    /// Maps each state to its row index in the sprite sheet
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

    /// Total number of animation frames for each state
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

/// Timer component to control animation playback speed (stagger frames)
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Component attached to the Player sprite entity
#[derive(Component)]
pub struct Player {
    pub current_frame: u32,
}

/// Resource holding the TextureAtlasLayout handle for lookups
#[derive(Resource)]
pub struct SpriteSheetLayout(pub Handle<TextureAtlasLayout>);

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

    // 2. Load Sprite Sheet Image
    let texture = asset_server.load("shadow_dog.png");

    // 3. Define the Grid Layout (12 columns, 10 rows of 575x523)
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(SPRITE_WIDTH as u32, SPRITE_HEIGHT as u32),
        TEXTURE_COLS,
        TEXTURE_ROWS,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(SpriteSheetLayout(layout_handle.clone()));

    // Initial state setup: 'Run' state starting frame calculation
    let initial_state = PlayerState::Run;
    let initial_index = (initial_state.row_index() * TEXTURE_COLS) as usize;

    // 4. Spawn Player Entity with Sprite containing TextureAtlas
    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: initial_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(1.0)),
        Player { current_frame: 0 },
        initial_state,
        // ~12 FPS animation speed (equivalent to staggerFrame = 5 at 60 FPS)
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
    ));
}

/// System that advances the animation frame index every tick
fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Player, &PlayerState, &mut Sprite), With<Player>>,
) {
    for (mut timer, mut player, state, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            // Cycle frame index: 0..state.frame_count()
            player.current_frame = (player.current_frame + 1) % state.frame_count();

            // Calculate linear index in the grid: (row * cols) + col
            let base_index = state.row_index() * TEXTURE_COLS;

            // Access the atlas stored inside the Sprite component
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = (base_index + player.current_frame) as usize;
            }
        }
    }
}

/// System to switch state via keyboard inputs
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
    debug!("New state requested: {:?}", new_state);

    if let Some(state) = new_state {
        for (mut current_state, mut player, mut sprite) in &mut query {
            if *current_state != state {
                *current_state = state;
                player.current_frame = 0; // Reset to frame 0 on state transition

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "JGame - Bevy Animation".into(),
                resolution: (800, 600).into(), // Changed floats to integers (u32, u32)
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_player, handle_input))
        .run();
}
