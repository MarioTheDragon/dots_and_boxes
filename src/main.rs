#![allow(clippy::type_complexity)]

mod boxes;
mod common;
mod current_player;
mod dots;
mod score;
mod sticks;

use crate::boxes::{BoxUpdateEvent, spawn_boxes, stick_selection_observer};
use crate::current_player::{spawn_current_player, update_player_display};
use crate::dots::spawn_corners;
use crate::score::{spawn_score, update_score_display};
use crate::sticks::spawn_edges;
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
        .add_systems(Startup, spawn_boxes)
        .add_systems(Startup, spawn_corners)
        .add_systems(Startup, spawn_edges)
        .add_systems(Update, update_score_display)
        .add_systems(Update, update_player_display)
        .run();
}
