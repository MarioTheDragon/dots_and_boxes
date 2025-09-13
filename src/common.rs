use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy)]
pub enum FieldOwner {
    Unselected,
    PlayerA,
    PlayerB,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
