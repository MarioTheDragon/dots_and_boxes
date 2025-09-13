mod boxes;
mod common;
mod dots;
mod sticks;

use crate::boxes::{BoxMarker, spawn_boxes};
use crate::dots::spawn_corners;
use crate::sticks::spawn_edges;
use bevy::prelude::*;

fn move_camera(mut camera: Single<&mut Transform, With<Camera2d>>) {
    camera.translation = Vec3::new(500.0, 250.0, 0.0);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Event)]
pub struct TestEvent(pub i32);

#[derive(Event)]
pub struct BoxEvent(pub i32);

fn main() {
    App::new()
        .add_event::<TestEvent>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_observer(
            |_: Trigger<BoxEvent>,
             mut boxes: Query<Entity, With<BoxMarker>>,
             mut commands: Commands| {
                for r#box in &mut boxes {
                    commands.trigger_targets(TestEvent(5), r#box);
                }
            },
        )
        .add_systems(Startup, (setup, move_camera).chain())
        .add_systems(Startup, (spawn_boxes, demo_trigger).chain())
        .add_systems(Startup, spawn_corners)
        .add_systems(Startup, spawn_edges)
        .run();
}

fn demo_trigger(mut commands: Commands) {
    commands.trigger(BoxEvent(5));
}
