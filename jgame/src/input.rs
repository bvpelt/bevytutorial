use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerInput {
    #[default]
    None,
    PressLeft,
    PressRight,
    PressDown,
    PressUp,
    ReleaseLeft,
    ReleaseRight,
    ReleaseDown,
    ReleaseUp,
}

impl std::fmt::Display for PlayerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerInput::None => write!(f, "NONE"),
            PlayerInput::PressLeft => write!(f, "PRESS left"),
            PlayerInput::PressRight => write!(f, "PRESS right"),
            PlayerInput::PressDown => write!(f, "PRESS down"),
            PlayerInput::PressUp => write!(f, "PRESS up"),
            PlayerInput::ReleaseLeft => write!(f, "RELEASE left"),
            PlayerInput::ReleaseRight => write!(f, "RELEASE right"),
            PlayerInput::ReleaseDown => write!(f, "RELEASE down"),
            PlayerInput::ReleaseUp => write!(f, "RELEASE up"),
        }
    }
}

#[derive(Resource, Default)]
pub struct LastInput(pub PlayerInput);

pub fn input_handler_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut last_input: ResMut<LastInput>,
) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        last_input.0 = PlayerInput::PressLeft;
    } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        last_input.0 = PlayerInput::PressRight;
    } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        last_input.0 = PlayerInput::PressDown;
    } else if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        last_input.0 = PlayerInput::PressUp;
    } else if keyboard.just_released(KeyCode::ArrowLeft) || keyboard.just_released(KeyCode::KeyA) {
        last_input.0 = PlayerInput::ReleaseLeft;
    } else if keyboard.just_released(KeyCode::ArrowRight) || keyboard.just_released(KeyCode::KeyD) {
        last_input.0 = PlayerInput::ReleaseRight;
    } else if keyboard.just_released(KeyCode::ArrowDown) || keyboard.just_released(KeyCode::KeyS) {
        last_input.0 = PlayerInput::ReleaseDown;
    } else if keyboard.just_released(KeyCode::ArrowUp) || keyboard.just_released(KeyCode::KeyW) {
        last_input.0 = PlayerInput::ReleaseUp;
    }
}
