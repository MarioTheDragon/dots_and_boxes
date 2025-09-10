use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy)]
pub enum FieldOwner {
    Unselected,
    PlayerA,
    // PlayerB,
}
