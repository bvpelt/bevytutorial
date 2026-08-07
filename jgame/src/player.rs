use bevy::prelude::*;

pub const SPRITE_WIDTH: f32 = 575.0;
pub const SPRITE_HEIGHT: f32 = 523.0;
pub const TEXTURE_COLS: u32 = 12;
pub const TEXTURE_ROWS: u32 = 10;

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
    pub fn row_index(&self) -> u32 {
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

    pub fn frame_count(&self) -> u32 {
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

// ----------------------------------------------------------------------------
// Plugin & Systems
// ----------------------------------------------------------------------------

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, animate_player);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
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

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout_handle,
                index: initial_index,
            }),
            ..default()
        },
        Transform::from_xyz(0.0, -150.0, 0.0).with_scale(Vec3::splat(0.6)),
        Player { current_frame: 0 },
        initial_state,
        AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)),
    ));
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
