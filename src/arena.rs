use bevy::prelude::*;

use crate::arena::{boxes::spawn_boxes, dots::spawn_corners, sticks::spawn_edges};

pub mod boxes;
pub mod dots;
pub mod sticks;

pub fn spawn_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_boxes(&mut commands, &mut meshes, &mut materials);
    spawn_edges(&mut commands, &mut meshes, &mut materials);
    spawn_corners(&mut commands, &mut meshes, &mut materials);
}
