use bevy::{
    color::palettes::tailwind::{BLUE_200, GRAY_200, RED_200},
    prelude::*,
};

use crate::{
    common::{GridPosition, CurrentPlayer}, score::Score, TestEvent
};

#[derive(Component, Clone, Copy)]
pub struct BoxMarker;

#[derive(Component, Clone, Copy)]
struct NumSelectedNeighbors(usize);

#[derive(Bundle, Clone)]
pub struct Box {
    num_selected_neighbors: NumSelectedNeighbors,
    marker: BoxMarker,
    grid_position: GridPosition,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

#[derive(Clone)]
struct BoxMaterialSet {
    unselected: Handle<ColorMaterial>,
    player_a: Handle<ColorMaterial>,
    player_b: Handle<ColorMaterial>,
}

impl BoxMaterialSet {
    fn get(&self, player: CurrentPlayer) -> Handle<ColorMaterial> {
        match player {
            CurrentPlayer::PlayerA => self.player_a.clone(),
            CurrentPlayer::PlayerB => self.player_b.clone(),
        }
    }
}

fn spawn_box(
    commands: &mut Commands,
    stick: Box,
    box_material_set: &BoxMaterialSet,
) {
    commands
        .spawn(stick.clone())
        .observe(update_material(box_material_set.clone()));
}

fn update_material(
    box_material_set: BoxMaterialSet,
) -> impl Fn(
    Trigger<TestEvent>,
    Query<(
        &mut MeshMaterial2d<ColorMaterial>,
        &mut NumSelectedNeighbors,
    )>,
    Single<&mut Score>,
    Single<&mut CurrentPlayer>,
) {
    move |trigger, mut box_query, mut score, mut current_player| {
        let (mut material, mut num_selected_neighbors) =
            box_query.get_mut(trigger.target()).unwrap();
        num_selected_neighbors.0 += 1;

        if num_selected_neighbors.0 == 4 {
            material.0 = box_material_set.get(**current_player);
            score.update(**current_player);
        } else {
            current_player.switch();
        }
    }
}

pub fn spawn_boxes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(90.0, 90.0));
    let box_material_set = BoxMaterialSet {
        unselected: materials.add(Color::from(GRAY_200)),
        player_a: materials.add(Color::from(RED_200)),
        player_b: materials.add(Color::from(BLUE_200)),
    };

    let mut r#box = Box {
        grid_position: GridPosition::new(1, 1),
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(box_material_set.unselected.clone()),
        transform: Transform::from_xyz(50.0, 50.0, 0.0),
        marker: BoxMarker,
        num_selected_neighbors: NumSelectedNeighbors(0),
    };

    for _ in 0..9 {
        for _ in 0..4 {
            spawn_box(&mut commands, r#box.clone(), &box_material_set);
            r#box.transform.translation.y += 100.0;
            r#box.grid_position.y += 2;
        }
        r#box.grid_position.y = 1;
        r#box.grid_position.x += 2;
        r#box.transform.translation.y = 50.0;
        r#box.transform.translation.x += 100.0;
    }
}
