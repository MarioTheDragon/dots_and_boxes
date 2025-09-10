use bevy::{
    color::palettes::tailwind::{CYAN_200, GRAY_200, RED_200},
    prelude::*,
};

use crate::{TestEvent, common::FieldOwner};

#[derive(Bundle, Clone)]
pub struct Box {
    state: FieldOwner,
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
    Query<(&mut FieldOwner, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    move |trigger, mut query| {
        let (mut owner, mut mesh) = query.get_mut(trigger.target()).unwrap();
        *owner = FieldOwner::PlayerA;
        mesh.0 = box_material_set.player_a.clone();
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
        player_a: materials.add(Color::from(CYAN_200)),
        player_b: materials.add(Color::from(RED_200)),
    };

    let mut new_box = Box {
        state: FieldOwner::Unselected,
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(box_material_set.unselected.clone()),
        transform: Transform::from_xyz(50.0, 50.0, 0.0),
    };

    for _ in 0..9 {
        for _ in 0..4 {
            spawn_box(&mut commands, new_box.clone(), &box_material_set);
            new_box.transform.translation.y += 100.0;
        }
        new_box.transform.translation.y = 50.0;
        new_box.transform.translation.x += 100.0;
    }
}
