use bevy::color::palettes::tailwind::{CYAN_300, GRAY_100, RED_300};
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
enum StickState {
    Unselected,
    PlayerA,
    // PlayerB,
}

#[derive(Bundle, Clone)]
struct Stick {
    state: StickState,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

struct StickMaterialSet {
    default: Handle<ColorMaterial>,
    hover: Handle<ColorMaterial>,
    selected: Handle<ColorMaterial>,
}

#[derive(Bundle, Clone)]
struct Corner {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

fn spawn_corners(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = materials.add(Color::from(GRAY_100));
    let shape = meshes.add(Rectangle::new(10.0, 10.0));

    let mut corner = Corner {
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(color),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
    };

    for _ in 0..10 {
        for _ in 0..5 {
            commands.spawn(corner.clone());
            corner.transform.translation.y += 100.0;
        }
        corner.transform.translation.y = 0.0;
        corner.transform.translation.x += 100.0;
    }
}

fn spawn_edge(commands: &mut Commands, stick: Stick, stick_material_set: &StickMaterialSet) {
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

fn spawn_edges(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = Color::hsl(100.0, 0.5, 0.5);
    let shape = meshes.add(Rectangle::new(10.0, 90.0));

    let stick_material_set = StickMaterialSet {
        default: materials.add(color.clone()),
        hover: materials.add(Color::from(CYAN_300)),
        selected: materials.add(Color::from(RED_300)),
    };

    let mut stick = Stick {
        state: StickState::Unselected,
        mesh: Mesh2d(shape),
        material: MeshMaterial2d(stick_material_set.default.clone()),
        transform: Transform::from_xyz(0.0, 50.0, 0.0),
    };

    for _ in 0..10 {
        for _ in 0..4 {
            spawn_edge(&mut commands, stick.clone(), &stick_material_set);
            stick.transform.translation.y += 100.0;
        }
        stick.transform.translation.y = 50.0;
        stick.transform.translation.x += 100.0;
    }

    stick.transform = Transform::from_xyz(50.0, 0.0, 0.0);
    stick.transform.rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0);
    for _ in 0..9 {
        for _ in 0..5 {
            spawn_edge(&mut commands, stick.clone(), &stick_material_set);
            stick.transform.translation.y += 100.0;
        }
        stick.transform.translation.y = 0.0;
        stick.transform.translation.x += 100.0;
    }
}

fn move_camera(mut camera: Single<&mut Transform, With<Camera2d>>) {
    camera.translation = Vec3::new(500.0, 250.0, 0.0);
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn on_click(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(Trigger<Pointer<Click>>, Query<&mut MeshMaterial2d<ColorMaterial>>, Query<&mut StickState>)
{
    move |trigger, mut material_query, mut stick_state_query| {
        if let Ok(mut material) = material_query.get_mut(trigger.target()) {
            material.0 = new_material.clone();
        }
        if let Ok(mut stick_state) = stick_state_query.get_mut(trigger.target()) {
            *stick_state = StickState::PlayerA;
        }
    }
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial2d<ColorMaterial>>, Query<&mut StickState>) {
    move |trigger, mut material_query, mut stick_state_query| {
        if let Ok(stick_state) = stick_state_query.get_mut(trigger.target()) {
            if !matches!(*stick_state, StickState::Unselected) {
                return;
            }
        }

        if let Ok(mut material) = material_query.get_mut(trigger.target()) {
            material.0 = new_material.clone();
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, (setup, move_camera).chain())
        .add_systems(Startup, spawn_corners)
        .add_systems(Startup, spawn_edges)
        .run();
}
