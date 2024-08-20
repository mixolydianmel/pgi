use bevy::{
    asset::Assets,
    color::Color,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

use crate::apriltag::TagDetectionList;

pub struct StructureDescriptor {
    mesh: Mesh2dHandle,
    color: Handle<ColorMaterial>,
    index: usize,
}

#[derive(Resource, Default)]
pub struct LoadedStructures(Vec<StructureDescriptor>);

pub struct PgiStructuresPlugin;

impl Plugin for PgiStructuresPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedStructures::default())
            .add_systems(Startup, load_structures)
            .add_systems(Update, update_structures);
    }
}

pub fn load_structures(
    mut loaded_structures: ResMut<LoadedStructures>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 10.0 })),
        color: materials.add(Color::linear_rgb(0.9, 0.9, 0.9)),
        index: 0,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 20.0 })),
        color: materials.add(Color::linear_rgb(0.9, 0.9, 0.9)),
        index: 1,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 30.0 })),
        color: materials.add(Color::linear_rgb(0.9, 0.9, 0.9)),
        index: 2,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        color: materials.add(Color::linear_rgb(0.9, 0.9, 0.9)),
        index: 3,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 80.0 })),
        color: materials.add(Color::linear_rgb(0.9, 0.9, 0.9)),
        index: 4,
    });
}

pub fn generate_meshes(mut commands: Commands, loaded_structures: Res<LoadedStructures>) {
    for structure in loaded_structures.0.iter() {
        commands.spawn((MaterialMesh2dBundle {
            mesh: structure.mesh.clone(),
            material: structure.color.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 100.0 + structure.index as f32),
            visibility: Visibility::Hidden,
            ..default()
        },));
    }
}

pub fn update_structures(
    mut commands: Commands,
    loaded_structures: Res<LoadedStructures>,
    det_map: Res<TagDetectionList>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (key, value) in det_map.0.iter() {
        if !loaded_structures.0.iter().any(|desc| desc.index == *key) {
            continue;
        }
        let structure = loaded_structures.0.get(*key).unwrap();
        let window = windows_query
            .get_single()
            .unwrap_or_else(|e| panic!("Unable to get window: {e}"));
        commands.spawn(MaterialMesh2dBundle {
            mesh: structure.mesh.clone(),
            material: structure.color.clone(),
            transform: Transform::from_xyz(
                value.transform.0 * window.width(),
                value.transform.1 * window.height(),
                structure.index as f32,
            ),
            ..default()
        });
    }
}
