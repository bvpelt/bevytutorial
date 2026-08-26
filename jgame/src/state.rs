use crate::input::PlayerInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PlayerState {
    StandingLeft,
    #[default]
    StandingRight,
    SittingLeft,
    SittingRight,
    RunningLeft,
    RunningRight,
    JumpingLeft,
    JumpingRight,
    FallingLeft,
    FallingRight,
}

impl std::fmt::Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerState::StandingLeft => write!(f, "STANDING LEFT"),
            PlayerState::StandingRight => write!(f, "STANDING RIGHT"),
            PlayerState::SittingLeft => write!(f, "SITTING LEFT"),
            PlayerState::SittingRight => write!(f, "SITTING RIGHT"),
            PlayerState::RunningLeft => write!(f, "RUNNING LEFT"),
            PlayerState::RunningRight => write!(f, "RUNNING RIGHT"),
            PlayerState::JumpingLeft => write!(f, "JUMPING LEFT"),
            PlayerState::JumpingRight => write!(f, "JUMPING RIGHT"),
            PlayerState::FallingLeft => write!(f, "FALLING LEFT"),
            PlayerState::FallingRight => write!(f, "FALLING RIGHT"),
        }
    }
}

impl PlayerState {
    /// Returns tuple: (row index on spritesheet, max animation frame count - 1)
    pub fn get_sprite_info(&self) -> (usize, usize) {
        match self {
            PlayerState::StandingRight => (0, 6),
            PlayerState::StandingLeft => (1, 6),
            PlayerState::JumpingRight => (2, 6),
            PlayerState::JumpingLeft => (3, 6),
            PlayerState::FallingRight => (4, 6),
            PlayerState::FallingLeft => (5, 6),
            PlayerState::RunningRight => (6, 8),
            PlayerState::RunningLeft => (7, 8),
            PlayerState::SittingRight => (8, 4),
            PlayerState::SittingLeft => (9, 4),
        }
    }

    pub fn get_target_speed(&self, max_speed: f32) -> f32 {
        match self {
            PlayerState::StandingLeft
            | PlayerState::StandingRight
            | PlayerState::SittingLeft
            | PlayerState::SittingRight => 0.0,
            PlayerState::RunningLeft => -max_speed,
            PlayerState::RunningRight => max_speed,
            PlayerState::JumpingLeft | PlayerState::FallingLeft => -max_speed * 0.5,
            PlayerState::JumpingRight | PlayerState::FallingRight => max_speed * 0.5,
        }
    }

    pub fn next_state(&self, input: PlayerInput, on_ground: bool, vy: f32) -> Option<PlayerState> {
        match self {
            PlayerState::StandingLeft => match input {
                PlayerInput::PressRight => Some(PlayerState::RunningRight),
                PlayerInput::PressLeft => Some(PlayerState::RunningLeft),
                PlayerInput::PressDown => Some(PlayerState::SittingLeft),
                PlayerInput::PressUp => Some(PlayerState::JumpingLeft),
                _ => None,
            },
            PlayerState::StandingRight => match input {
                PlayerInput::PressLeft => Some(PlayerState::RunningLeft),
                PlayerInput::PressRight => Some(PlayerState::RunningRight),
                PlayerInput::PressDown => Some(PlayerState::SittingRight),
                PlayerInput::PressUp => Some(PlayerState::JumpingRight),
                _ => None,
            },
            PlayerState::SittingLeft => match input {
                PlayerInput::PressRight => Some(PlayerState::SittingRight),
                PlayerInput::ReleaseDown => Some(PlayerState::StandingLeft),
                _ => None,
            },
            PlayerState::SittingRight => match input {
                PlayerInput::PressLeft => Some(PlayerState::SittingLeft),
                PlayerInput::ReleaseDown => Some(PlayerState::StandingRight),
                _ => None,
            },
            PlayerState::RunningLeft => match input {
                PlayerInput::PressRight => Some(PlayerState::RunningRight),
                PlayerInput::ReleaseLeft | PlayerInput::PressDown => {
                    Some(PlayerState::StandingLeft)
                }
                _ => None,
            },
            PlayerState::RunningRight => match input {
                PlayerInput::PressLeft => Some(PlayerState::RunningLeft),
                PlayerInput::ReleaseRight | PlayerInput::PressDown => {
                    Some(PlayerState::StandingRight)
                }
                _ => None,
            },
            PlayerState::JumpingLeft => {
                if input == PlayerInput::PressRight {
                    Some(PlayerState::JumpingRight)
                } else if on_ground {
                    Some(PlayerState::StandingLeft)
                } else if vy < 0.0 {
                    Some(PlayerState::FallingLeft)
                } else {
                    None
                }
            }
            PlayerState::JumpingRight => {
                if input == PlayerInput::PressLeft {
                    Some(PlayerState::JumpingLeft)
                } else if on_ground {
                    Some(PlayerState::StandingRight)
                } else if vy < 0.0 {
                    Some(PlayerState::FallingRight)
                } else {
                    None
                }
            }
            PlayerState::FallingLeft => {
                if input == PlayerInput::PressRight {
                    Some(PlayerState::FallingRight)
                } else if on_ground {
                    Some(PlayerState::StandingRight)
                } else {
                    None
                }
            }
            PlayerState::FallingRight => {
                if input == PlayerInput::PressLeft {
                    Some(PlayerState::FallingLeft)
                } else if on_ground {
                    Some(PlayerState::StandingRight)
                } else {
                    None
                }
            }
        }
    }
}
