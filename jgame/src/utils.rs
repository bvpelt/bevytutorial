use crate::input::LastInput;
use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct StatusText;

pub fn setup_status_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Last input: NONE\nActive state: STANDING RIGHT"),
        TextFont {
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        StatusText,
    ));
}

pub fn update_status_text_system(
    last_input: Res<LastInput>,
    player_query: Query<&Player>,
    mut text_query: Query<&mut Text, With<StatusText>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    **text = format!(
        "Last input: {}\nActive state: {}",
        last_input.0, player.state
    );
}
