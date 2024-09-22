use crate::apriltag::TagDetectionList;
use bevy::{
    asset::Assets,
    color::Color,
    prelude::*,
    reflect::TypePath,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::PrimaryWindow,
};
use bevy_common_assets::json::JsonAssetPlugin;
use serde::Deserialize;

#[derive(Component)]
pub struct Structure {
    index: usize,
}

pub struct StructureDescriptor {
    mesh: Mesh2dHandle,
    color: Handle<ColorMaterial>,
    index: usize,
}

#[derive(Deserialize, Debug)]
enum StructureShape {
    SQUARE,
    CIRCLE,
}

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct StructureData {
    id: u32,
    shape: StructureShape,
    color: String,
}

#[derive(Resource)]
pub struct StructureDataHandle(Handle<StructureData>);

#[derive(Resource, Default)]
pub struct LoadedStructures(Vec<StructureDescriptor>);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Hash, States)]
enum LoadState {
    #[default]
    Loading,
    Done,
}

pub struct PgiStructuresPlugin;

impl Plugin for PgiStructuresPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedStructures::default())
            .add_plugins(JsonAssetPlugin::<StructureData>::new(&["structures.json"]))
            .add_systems(Startup, load_structure_data)
            .add_systems(
                Update,
                (
                    parse_structures.run_if(in_state(LoadState::Loading)),
                    update_structures.run_if(in_state(LoadState::Done)),
                ),
            )
            .init_state::<LoadState>();
    }
}

pub fn load_structure_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = StructureDataHandle(asset_server.load("structures.json"));
    commands.insert_resource(handle);
}

pub fn parse_structures(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut data_files: ResMut<Assets<StructureData>>,
    data_handle: Res<StructureDataHandle>,
) {
    if let Some(data) = data_files.remove(data_handle.0.id()) {
        info!("{data:?}");
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(400.0, 400.0))),
                material: materials.add(Color::linear_rgb(0.1, 0.9, 0.1)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                visibility: Visibility::Hidden,
                ..default()
            },
            Structure { index: 0 },
        ));
    }
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
