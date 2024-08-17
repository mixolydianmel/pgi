use bevy::prelude::*;
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

use crate::input::launch::WebcamIndex;

pub struct PgiWebcamPlugin;

impl Plugin for PgiWebcamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            setup_webcam.after(crate::input::launch::parse_args),
        );
    }
}

/// TODO: DEPRECATE
pub fn setup_webcam(world: &mut World) {
    let webcam_index = world
        .get_resource::<WebcamIndex>()
        .unwrap_or_else(|| panic!("Unable to retrieve webcam index resource."));
    let index = CameraIndex::Index(webcam_index.0);
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let camera = match nokhwa::Camera::new(index, requested) {
        Ok(c) => c,
        Err(e) => {
            panic!(
                "Unable to create a webcam object with index `{}`: {}",
                webcam_index.0, e
            )
        }
    };

    world.insert_non_send_resource(camera);
}
