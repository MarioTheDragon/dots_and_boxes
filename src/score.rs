use bevy::{
    color::palettes::tailwind::{BLUE_200, RED_200},
    prelude::*,
};

use crate::common::{CurrentPlayer};

#[derive(Component, Clone, Copy)]
pub struct Score {
    player_a: usize,
    player_b: usize,
    has_updated: bool,
}

impl Score {
    pub fn new() -> Self {
        Self {
            player_a: 0,
            player_b: 0,
            has_updated: true,
        }
    }

    pub fn update(&mut self, current_player: CurrentPlayer) {
        match current_player {
            CurrentPlayer::PlayerA => self.player_a += 1,
            CurrentPlayer::PlayerB => self.player_b += 1,
        }

        self.has_updated = true;
    }
}

pub fn update_score_display(
    mut score_display: Single<(Entity, &mut Score)>,
    mut writer: TextUiWriter,
) {
    let (entity, ref mut score) = *score_display;
    if score.has_updated {
        *writer.text(entity, 2) = score.player_a.to_string();
        *writer.text(entity, 4) = score.player_b.to_string();
        score.has_updated = false;
    }
}

pub fn spawn_score(mut commands: Commands) {
    commands
        .spawn((
            Text::default(),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            },
            Score::new(),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::new("Player A: "),
                TextColor(RED_200.into()),
            ));
            parent.spawn((TextSpan::default(), TextColor(RED_200.into())));
            parent.spawn((
                TextSpan::new("\nPlayer B: "),
                TextColor(BLUE_200.into()),
            ));
            parent.spawn((TextSpan::default(), TextColor(BLUE_200.into())));
        });
}
