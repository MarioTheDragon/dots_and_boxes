use bevy::color::palettes::tailwind::{CYAN_300, CYAN_400, RED_300, RED_400};
use bevy::math::Quat;
use bevy::prelude::*;
use bevy::prelude::{
    Bundle, Click, ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Out,
    Over, Pointer, Query, Rectangle, ResMut, Transform, Trigger,
};

use crate::common::GridPosition;
use crate::current_player::CurrentPlayer;

#[derive(Component, Clone, Copy, Debug)]
pub enum StickOrientation {
    Vertical,
    Horizontal,
}

#[derive(Event, Debug)]
pub struct StickSelectEvent {
    pub position: GridPosition,
    pub orientation: StickOrientation,
}

#[derive(Component, Clone, Copy)]
pub struct StickSelected(bool);

#[derive(Bundle, Clone)]
pub struct Stick {
    selected: StickSelected,
    grid_position: GridPosition,
    orientation: StickOrientation,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

#[derive(Clone)]
struct StickMaterialSet {
    default: Handle<ColorMaterial>,
    hover_a: Handle<ColorMaterial>,
    hover_b: Handle<ColorMaterial>,
    selected_a: Handle<ColorMaterial>,
    selected_b: Handle<ColorMaterial>,
}

fn spawn_edge(
    commands: &mut Commands,
    stick: Stick,
    stick_material_set: &StickMaterialSet,
) {
    commands
        .spawn(stick.clone())
        .observe(on_hover(stick_material_set.clone()))
        .observe(on_out(stick_material_set.default.clone()))
        .observe(on_click(stick_material_set.clone()));
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
        hover_a: materials.add(Color::from(RED_300)),
        hover_b: materials.add(Color::from(CYAN_300)),
        selected_a: materials.add(Color::from(RED_400)),
        selected_b: materials.add(Color::from(CYAN_400)),
    };

    let mut stick = Stick {
        grid_position: GridPosition::new(0, 1),
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(stick_material_set.default.clone()),
        transform: Transform::from_xyz(0.0, 50.0, 0.0),
        orientation: StickOrientation::Vertical,
        selected: StickSelected(false),
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
    stick_material_set: StickMaterialSet
) -> impl Fn(
    Trigger<Pointer<Click>>,
    Commands,
    Query<(
        &mut MeshMaterial2d<ColorMaterial>,
        &mut StickSelected,
        &GridPosition,
        &StickOrientation,
    )>,
    Single<&CurrentPlayer>,
) {
    move |trigger, mut commands, mut sticks, current_player| {
        if let Ok((mut material, mut selected, position, orientation)) =
            sticks.get_mut(trigger.target())
        {
            selected.0 = true;
            material.0 = match *current_player {
                CurrentPlayer::PlayerA => stick_material_set.selected_a.clone(),
                CurrentPlayer::PlayerB => stick_material_set.selected_b.clone(),
            };

            commands.trigger(StickSelectEvent {
                position: *position,
                orientation: *orientation,
            });
        }
    }
}

fn on_hover(
    stick_material_set: StickMaterialSet,
) -> impl Fn(
    Trigger<Pointer<Over>>,
    Query<(&StickSelected, &mut MeshMaterial2d<ColorMaterial>)>,
    Single<&CurrentPlayer>,
) {
    move |trigger, mut sticks, current_player| {
        if let Ok(stick) = sticks.get_mut(trigger.target()) {
            let (already_selected, mut material) = stick;
            if already_selected.0 {
                return;
            }

            material.0 = match *current_player {
                CurrentPlayer::PlayerA => stick_material_set.hover_a.clone(),
                CurrentPlayer::PlayerB => stick_material_set.hover_b.clone(),
            };
        }
    }
}

fn on_out(
    default_material: Handle<ColorMaterial>,
) -> impl Fn(
    Trigger<Pointer<Out>>,
    Query<(&StickSelected, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    move |trigger, mut sticks| {
        if let Ok(stick) = sticks.get_mut(trigger.target()) {
            let (already_selected, mut stick_material) = stick;
            if already_selected.0 {
                return;
            }

            stick_material.0 = default_material.clone();
        }
    }
}

