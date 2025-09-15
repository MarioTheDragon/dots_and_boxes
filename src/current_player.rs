use std::fmt::Display;

use bevy::{color::palettes::tailwind::{GRAY_50, RED_200}, prelude::*};

#[derive(Component, Clone, Copy)]
pub enum CurrentPlayer {
    PlayerA,
    PlayerB,
}

impl Display for CurrentPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentPlayer::PlayerA => write!(f, "A"),
            CurrentPlayer::PlayerB => write!(f, "B"),        
        }
    }
}

impl CurrentPlayer {
    pub fn switch(&mut self) {
        match self {
            CurrentPlayer::PlayerA => *self = CurrentPlayer::PlayerB,
            CurrentPlayer::PlayerB => *self = CurrentPlayer::PlayerA,
        }
    }
}

pub fn update_player_display(
    mut player_display: Single<(Entity, &mut CurrentPlayer)>,
    mut writer: TextUiWriter,
) {
    let (entity, ref mut player) = *player_display;
    *writer.text(entity, 2) = player.to_string();
}

pub fn spawn_current_player(mut commands: Commands) {
    commands
        .spawn((
            Text::default(),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(12.0),
                ..default()
            },
            CurrentPlayer::PlayerA,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("Current player: "),
                TextColor(GRAY_50.into()),
            ));
            parent.spawn((TextSpan::default(), TextColor(RED_200.into())));
        });
}
