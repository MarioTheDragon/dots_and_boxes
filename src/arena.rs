use bevy::prelude::*;

use self::{
    boxes::spawn_boxes, dimensions::Dimensions, dots::spawn_corners,
    sticks::spawn_edges,
};

pub mod boxes;
mod dimensions;
pub mod dots;
pub mod sticks;

pub fn spawn_arena(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let dimensions = Dimensions::from(10, 5);

    spawn_boxes(commands, meshes, materials, dimensions);
    spawn_edges(commands, meshes, materials, dimensions);
    spawn_corners(commands, meshes, materials, dimensions);
}
