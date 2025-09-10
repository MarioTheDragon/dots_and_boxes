mod boxes;
mod common;
mod dots;
mod sticks;

use crate::boxes::spawn_boxes;
use crate::common::FieldOwner;
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
#[event(auto_propagate)]
pub struct TestEvent(pub i32);

fn main() {
    App::new()
        .add_event::<TestEvent>()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_observer(
            |_: Trigger<TestEvent>,
             mut sticks: Query<Entity, With<FieldOwner>>,
             mut commands: Commands| {
                for stick in &mut sticks {
                    commands.trigger_targets(TestEvent(5), stick);
                }
            },
        )
        .add_systems(Startup, (setup, move_camera).chain())
        .add_systems(Startup, spawn_boxes)
        .add_systems(Startup, spawn_corners)
        .add_systems(Startup, spawn_edges)
        // .add_systems(Update, |mut commands: Commands| {
        //     commands.trigger(TestEvent(6));
        // })
        .run();
}
