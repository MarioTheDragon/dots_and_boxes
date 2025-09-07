use crate::dots::Dot;
use crate::sticks::Stick;
use bevy::asset::Assets;
use bevy::prelude::{ColorMaterial, Commands, Mesh, ResMut};

pub struct ArenaPosition {
    x: usize,
    y: usize,
}

impl ArenaPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

enum FieldValue {
    Dot(Dot),
    Box(()),
    Stick(Stick),
}

struct Field {
    field_type: FieldValue,
    position: ArenaPosition,
}

pub struct Arena {
    fields: Vec<Vec<FieldValue>>,
    dot_size: f32,
    stick_length: f32,
    x_num_corners: usize,
    y_num_corners: usize,
}

impl Arena {
    pub fn new(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        dot_size: f32,
        stick_length: f32,
        x_num_corners: usize,
        y_num_corners: usize,
    ) {
    }
}
