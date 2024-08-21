use bevy::{
    asset::Assets,
    color::Color,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};

use crate::apriltag::TagDetectionList;

#[derive(Component)]
pub struct Structure {
    index: usize,
}

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
            .add_systems(Startup, (load_structures, generate_meshes).chain())
            .add_systems(Update, update_structures);
    }
}

pub fn load_structures(
    mut loaded_structures: ResMut<LoadedStructures>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(400.0, 400.0))),
        color: materials.add(Color::linear_rgb(0.1, 0.9, 0.1)),
        index: 0,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 400.0 })),
        color: materials.add(Color::linear_rgb(0.9, 0.1, 0.1)),
        index: 1,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Annulus::new(200.0, 400.0))),
        color: materials.add(Color::linear_rgb(0.1, 0.1, 0.9)),
        index: 2,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::new(-400.0, -400.0),
            Vec2::new(400.0, -400.0),
            Vec2::new(0.0, 400.0),
        ))),
        color: materials.add(Color::linear_rgb(0.9, 0.1, 0.9)),
        index: 3,
    });
    loaded_structures.0.push(StructureDescriptor {
        mesh: Mesh2dHandle(meshes.add(Capsule2d::new(200.0, 400.0))),
        color: materials.add(Color::linear_rgb(0.1, 0.9, 0.9)),
        index: 4,
    });
}

pub fn generate_meshes(mut commands: Commands, loaded_structures: Res<LoadedStructures>) {
    for structure in loaded_structures.0.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: structure.mesh.clone(),
                material: structure.color.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            Structure {
                index: structure.index,
            },
        ));
    }
}

pub fn update_structures(
    mut structure_query: Query<(&mut Transform, &mut Structure, &mut Visibility)>,
    det_map: Res<TagDetectionList>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut transform, structure, mut vis) in structure_query.iter_mut() {
        if !det_map.0.contains_key(&structure.index) {
            *vis = Visibility::Hidden;
            continue;
        }

        *vis = Visibility::Visible;

        let window = windows_query
            .get_single()
            .unwrap_or_else(|e| panic!("Unable to get primary window: {e}"));
        let (ss_x, ss_y) = det_map.0.get(&structure.index).unwrap().transform;
        debug!("Translation: {}", transform.translation);
        *transform = Transform::from_xyz(
            window.width() * ss_x,
            window.height() * ss_y,
            structure.index as f32,
        );
    }
}
