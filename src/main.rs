#![allow(clippy::type_complexity)]

mod arena;
mod current_player;
mod grid_position;
mod score;
mod ui;

use crate::arena::boxes::{BoxUpdateEvent, stick_selection_observer};
use crate::current_player::update_player_display;
use crate::score::update_score_display;
use crate::ui::main_menu::{button_system, spawn_start_menu};
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
        .add_systems(Startup, (setup, move_camera).chain())
        .add_systems(Startup, spawn_start_menu)
        .add_systems(Update, update_score_display)
        .add_systems(Update, update_player_display)
        .add_systems(Update, button_system)
        .run();
}
