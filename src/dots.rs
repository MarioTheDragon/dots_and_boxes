use bevy::asset::Assets;
use bevy::color::Color;
use bevy::color::palettes::tailwind::GRAY_100;
use bevy::prelude::{
    Bundle, ColorMaterial, Commands, Mesh, Mesh2d, MeshMaterial2d, Rectangle, ResMut, Transform,
};

#[derive(Bundle, Clone)]
pub struct Dot {
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

pub fn spawn_corners(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = materials.add(Color::from(GRAY_100));
    let shape = meshes.add(Rectangle::new(10.0, 10.0));

    let mut corner = Dot {
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
