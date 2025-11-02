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
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let dimensions = Dimensions::from(10, 5);

    spawn_boxes(&mut commands, &mut meshes, &mut materials, dimensions);
    spawn_edges(&mut commands, &mut meshes, &mut materials, dimensions);
    spawn_corners(&mut commands, &mut meshes, &mut materials, dimensions);
}
