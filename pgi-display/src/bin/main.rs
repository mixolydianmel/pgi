use bevy::prelude::*;
use pgi_display::{
    rendering::{setup_camera, setup_windows},
    structures::{generate_meshes, load_structures, LoadedStructures},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(LoadedStructures::default())
        .add_systems(
            Startup,
            (
                (setup_windows, setup_camera).chain(),
                (load_structures, generate_meshes).chain(),
            ),
        )
        .run();
}
