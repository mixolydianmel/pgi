use bevy::prelude::*;
use pgi_display::{
    apriltag::PgiAprilTagPlugin, input::PgiInputPlugin, rendering::PgiRenderingPlugin,
    structures::PgiStructuresPlugin, webcam::PgiWebcamPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PgiInputPlugin,
            PgiRenderingPlugin,
            PgiStructuresPlugin,
            PgiWebcamPlugin,
            // PgiDebugCameraView,
            PgiAprilTagPlugin,
        ))
        .run();
}
