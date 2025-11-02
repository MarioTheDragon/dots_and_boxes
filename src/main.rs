#![allow(clippy::type_complexity)]

mod grid_position;
mod current_player;
mod score;
mod arena;

use crate::arena::boxes::{BoxUpdateEvent, stick_selection_observer};
use crate::arena::spawn_arena;
use crate::current_player::{spawn_current_player, update_player_display};
use crate::score::{spawn_score, update_score_display};
use bevy::prelude::*;

fn move_camera(mut camera: Single<&mut Transform, With<Camera2d>>) {
    camera.translation = Vec3::new(500.0, 250.0, 0.0);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn main() {
    App::new()
        .add_event::<BoxUpdateEvent>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_observer(stick_selection_observer)
        .add_systems(Startup, spawn_score)
        .add_systems(Startup, spawn_current_player)
        .add_systems(Startup, (setup, move_camera).chain())
        .add_systems(Startup, spawn_arena)
        .add_systems(Update, update_score_display)
        .add_systems(Update, update_player_display)
        .run();
}
