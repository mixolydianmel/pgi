use bevy::prelude::*;
use pgi_display::{rendering::setup_camera, structures::setup_shapes};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_shapes))
        .run();
}
