mod dots;
mod sticks;
mod arena;

use crate::dots::spawn_corners;
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
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, (setup, move_camera).chain())
        .add_systems(Startup, spawn_corners)
        .add_systems(Startup, spawn_edges)
        .run();
}
