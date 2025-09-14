#![allow(clippy::type_complexity)]

mod boxes;
mod common;
mod dots;
mod sticks;
mod score;

use crate::boxes::{BoxMarker, spawn_boxes};
use crate::score::{spawn_score, update_score_display};
use crate::common::GridPosition;
use crate::dots::spawn_corners;
use crate::sticks::{StickOrientation, StickSelectEvent, spawn_edges};
use bevy::prelude::*;

fn move_camera(mut camera: Single<&mut Transform, With<Camera2d>>) {
    camera.translation = Vec3::new(500.0, 250.0, 0.0);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Event)]
pub struct TestEvent(pub i32);

fn main() {
    App::new()
        .add_event::<TestEvent>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_observer(stick_selection_observer)
        .add_systems(Startup, spawn_score)
        .add_systems(Startup, (setup, move_camera).chain())
        .add_systems(Startup, spawn_boxes)
        .add_systems(Startup, spawn_corners)
        .add_systems(Startup, spawn_edges)
        .add_systems(Update, update_score_display)
        .run();
}

fn stick_selection_observer(
    trigger: Trigger<StickSelectEvent>,
    boxes: Query<(Entity, &GridPosition), With<BoxMarker>>,
    mut commands: Commands,
) {
    let event = trigger.event();
    for (r#box, position) in &boxes {
        match event.orientation {
            StickOrientation::Vertical => {
                let is_left = position.x == event.position.x + 1;
                let is_right = position.x == event.position.x - 1;
                let same_y = position.y == event.position.y;
                if (is_left || is_right) && same_y {
                    commands.trigger_targets(TestEvent(5), r#box);
                }
            }
            StickOrientation::Horizontal => {
                let is_above = position.y == event.position.y + 1;
                let is_below = position.y == event.position.y - 1;
                let same_x = position.x == event.position.x;
                if (is_above || is_below) && same_x {
                    commands.trigger_targets(TestEvent(5), r#box);
                }
            }
        }
    }
}

