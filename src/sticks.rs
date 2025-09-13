use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::color::palettes::tailwind::{CYAN_300, RED_300};
use bevy::ecs::component::Component;
use bevy::math::Quat;
use bevy::prelude::{
    Bundle, Click, ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Out,
    Over, Pointer, Query, Rectangle, ResMut, Transform, Trigger,
};

use crate::StickSelectEvent;
use crate::common::{FieldOwner, GridPosition};

#[derive(Component, Clone, Copy, Debug)]
pub enum StickOrientation {
    Vertical,
    Horizontal,
}

#[derive(Bundle, Clone)]
pub struct Stick {
    grid_position: GridPosition,
    orientation: StickOrientation,
    owner: FieldOwner,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

struct StickMaterialSet {
    default: Handle<ColorMaterial>,
    hover: Handle<ColorMaterial>,
    selected: Handle<ColorMaterial>,
}

fn spawn_edge(
    commands: &mut Commands,
    stick: Stick,
    stick_material_set: &StickMaterialSet,
) {
    commands
        .spawn(stick.clone())
        .observe(update_material_on::<Pointer<Over>>(
            stick_material_set.hover.clone(),
        ))
        .observe(update_material_on::<Pointer<Out>>(
            stick_material_set.default.clone(),
        ))
        .observe(on_click(stick_material_set.selected.clone()));
}

pub fn spawn_edges(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(100.0, 0.5, 0.5);
    let shape = meshes.add(Rectangle::new(10.0, 90.0));

    let stick_material_set = StickMaterialSet {
        default: materials.add(color),
        hover: materials.add(Color::from(CYAN_300)),
        selected: materials.add(Color::from(RED_300)),
    };

    let mut stick = Stick {
        grid_position: GridPosition::new(0, 1),
        owner: FieldOwner::Unselected,
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(stick_material_set.default.clone()),
        transform: Transform::from_xyz(0.0, 50.0, 0.0),
        orientation: StickOrientation::Vertical,
    };

    for _ in 0..10 {
        for _ in 0..4 {
            spawn_edge(&mut commands, stick.clone(), &stick_material_set);
            stick.transform.translation.y += 100.0;
            stick.grid_position.y += 2;
        }
        stick.grid_position.y = 1;
        stick.grid_position.x += 2;
        stick.transform.translation.y = 50.0;
        stick.transform.translation.x += 100.0;
    }

    stick.grid_position = GridPosition::new(1, 0);
    stick.transform = Transform::from_xyz(50.0, 0.0, 0.0);
    stick.orientation = StickOrientation::Horizontal;
    stick.transform.rotation =
        Quat::from_rotation_z(std::f32::consts::PI / 2.0);
    for _ in 0..9 {
        for _ in 0..5 {
            spawn_edge(&mut commands, stick.clone(), &stick_material_set);
            stick.transform.translation.y += 100.0;
            stick.grid_position.y += 2;
        }
        stick.grid_position.y = 0;
        stick.grid_position.x += 2;
        stick.transform.translation.y = 0.0;
        stick.transform.translation.x += 100.0;
    }
}

fn on_click(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(
    Trigger<Pointer<Click>>,
    Commands,
    Query<(
        &mut FieldOwner,
        &mut MeshMaterial2d<ColorMaterial>,
        &GridPosition,
        &StickOrientation,
    )>,
) {
    move |trigger, mut commands, mut sticks| {
        if let Ok((mut owner, mut material, position, orientation)) =
            sticks.get_mut(trigger.target())
        {
            commands.trigger(StickSelectEvent {
                position: *position,
                orientation: *orientation,
            });
            material.0 = new_material.clone();
            *owner = FieldOwner::PlayerA;
        }
    }
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(Trigger<E>, Query<(&mut FieldOwner, &mut MeshMaterial2d<ColorMaterial>)>)
{
    move |trigger, mut sticks| {
        if let Ok(mut stick) = sticks.get_mut(trigger.target()) {
            if !matches!(*stick.0, FieldOwner::Unselected) {
                return;
            }
            stick.1.0 = new_material.clone();
        }
    }
}
