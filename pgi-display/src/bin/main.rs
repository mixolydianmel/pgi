use bevy::prelude::*;
use pgi_display::{
    input::PgiInputPlugin, rendering::PgiRenderingPlugin, structures::PgiStructuresPlugin,
    webcam::PgiWebcamPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PgiInputPlugin,
            PgiRenderingPlugin,
            PgiWebcamPlugin,
            PgiStructuresPlugin,
        ))
        .run();
}
