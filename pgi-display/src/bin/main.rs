use bevy::prelude::*;
use pgi_display::{
    apriltag::PgiAprilTagPlugin, input::PgiInputPlugin, rendering::PgiRenderingPlugin,
    ui::PgiUiPlugin, webcam::PgiWebcamPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PgiRenderingPlugin,
            PgiInputPlugin,
            PgiWebcamPlugin,
            PgiAprilTagPlugin,
            PgiUiPlugin,
        ))
        .run();
}
