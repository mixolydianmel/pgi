use bevy::{
    asset::Assets,
    color::Color,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub fn setup_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        material: materials.add(Color::linear_rgb(0.9, 0.7, 0.5)),
        transform: Transform::default(),
        ..default()
    });
}
