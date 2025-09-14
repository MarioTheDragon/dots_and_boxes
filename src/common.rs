use bevy::prelude::*;

pub fn spawn_current_player(mut commands: Commands) {
    commands.spawn(CurrentPlayer::PlayerA);
}

#[derive(Component, Clone, Copy, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Clone, Copy)]
pub enum CurrentPlayer {
    PlayerA,
    PlayerB,
}

impl CurrentPlayer {
    pub fn switch(&mut self) {
        match self {
            CurrentPlayer::PlayerA => *self = CurrentPlayer::PlayerB,
            CurrentPlayer::PlayerB => *self = CurrentPlayer::PlayerA,
        }
    }
}
