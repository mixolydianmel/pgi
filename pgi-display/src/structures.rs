use bevy::{
    asset::Assets,
    color::Color,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Structure;

pub struct StructureDescriptor {
    mesh: Mesh2dHandle,
    color: Handle<ColorMaterial>,
}

#[derive(Resource, Default)]
pub struct LoadedStructures(Vec<StructureDescriptor>);

pub struct PgiStructuresPlugin;

impl Plugin for PgiStructuresPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedStructures::default())
            .add_systems(Startup, (load_structures, generate_meshes).chain());
    }
}

pub fn load_structures(
    mut loaded_structures: ResMut<LoadedStructures>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        color: materials.add(Color::linear_rgb(0.9, 0.2, 0.2)),
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 100.0))),
        color: materials.add(Color::linear_rgb(0.2, 0.2, 0.9)),
    });
}

pub fn generate_meshes(mut commands: Commands, loaded_structures: Res<LoadedStructures>) {
    for (index, structure) in loaded_structures.0.iter().enumerate() {
        commands.spawn(MaterialMesh2dBundle {
            mesh: structure.mesh.clone(),
            material: structure.color.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0 - index as f32),
            ..default()
        });
    }
}
