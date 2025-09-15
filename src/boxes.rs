use bevy::{
    color::palettes::tailwind::{BLUE_200, GRAY_200, RED_200},
    prelude::*,
};

use crate::{
    common::GridPosition,
    current_player::CurrentPlayer,
    score::Score,
    sticks::{StickOrientation, StickSelectEvent},
};

#[derive(Event)]
pub struct BoxUpdateEvent;

#[derive(Component, Clone, Copy)]
pub struct NumSelectedNeighbors(pub usize);

#[derive(Bundle, Clone)]
pub struct Box {
    num_selected_neighbors: NumSelectedNeighbors,
    grid_position: GridPosition,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

#[derive(Clone)]
pub struct BoxMaterialSet {
    unselected: Handle<ColorMaterial>,
    player_a: Handle<ColorMaterial>,
    player_b: Handle<ColorMaterial>,
}

impl BoxMaterialSet {
    pub fn new(
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            unselected: materials.add(Color::from(GRAY_200)),
            player_a: materials.add(Color::from(RED_200)),
            player_b: materials.add(Color::from(BLUE_200)),
        }
    }
}

impl BoxMaterialSet {
    fn get(&self, player: CurrentPlayer) -> Handle<ColorMaterial> {
        match player {
            CurrentPlayer::PlayerA => self.player_a.clone(),
            CurrentPlayer::PlayerB => self.player_b.clone(),
        }
    }
}

pub fn stick_selection_observer(
    trigger: Trigger<StickSelectEvent>,
    mut boxes: Query<(
        &GridPosition,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut NumSelectedNeighbors,
    )>,
    mut score: Single<&mut Score>,
    mut current_player: Single<&mut CurrentPlayer>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    let box_material_set = BoxMaterialSet::new(materials);
    let event = trigger.event();
    let mut should_player_switch = true;

    for (position, mut material, mut neighbors) in &mut boxes {
        let mut claim_box = || {
            neighbors.0 += 1;
            if neighbors.0 == 4 {
                material.0 = box_material_set.get(**current_player);
                score.update(**current_player);
                should_player_switch = false;
            }
        };

        match event.orientation {
            StickOrientation::Vertical => {
                let is_left = position.x == event.position.x + 1;
                let is_right = position.x == event.position.x - 1;
                let same_y = position.y == event.position.y;
                if (is_left || is_right) && same_y {
                    claim_box();
                }
            }
            StickOrientation::Horizontal => {
                let is_above = position.y == event.position.y + 1;
                let is_below = position.y == event.position.y - 1;
                let same_x = position.x == event.position.x;
                if (is_above || is_below) && same_x {
                    claim_box();
                }
            }
        }
    }

    if should_player_switch {
        current_player.switch();
    }
}

pub fn spawn_boxes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>
) {
    let box_material_set = BoxMaterialSet::new(materials);
    let shape = meshes.add(Rectangle::new(90.0, 90.0));

    let mut r#box = Box {
        grid_position: GridPosition::new(1, 1),
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(box_material_set.unselected.clone()),
        transform: Transform::from_xyz(50.0, 50.0, 0.0),
        num_selected_neighbors: NumSelectedNeighbors(0),
    };

    for _ in 0..9 {
        for _ in 0..4 {
            commands.spawn(r#box.clone());
            r#box.transform.translation.y += 100.0;
            r#box.grid_position.y += 2;
        }
        r#box.grid_position.y = 1;
        r#box.grid_position.x += 2;
        r#box.transform.translation.y = 50.0;
        r#box.transform.translation.x += 100.0;
    }
}
