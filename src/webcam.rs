use bevy::prelude::*;
use image::RgbaImage;
use nokhwa::{
    pixel_format::RgbAFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

use crate::input::launch::WebcamIndex;

pub struct PgiWebcamPlugin;

impl Plugin for PgiWebcamPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WebcamImage(RgbaImage::new(800, 600)))
            .add_systems(
                Startup,
                setup_webcam.after(crate::input::launch::parse_args),
            )
            .add_systems(Update, update_webcam_image);
    }
}

#[derive(Resource, Default)]
pub struct WebcamImage(pub RgbaImage);

pub fn setup_webcam(world: &mut World) {
    let webcam_index = CameraIndex::Index(
        world
            .get_resource::<WebcamIndex>()
            .unwrap_or_else(|| panic!("Unable to retrieve webcam index resource."))
            .0,
    );
    let requested =
        RequestedFormat::new::<RgbAFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = match nokhwa::Camera::new(webcam_index.clone(), requested) {
        Ok(c) => c,
        Err(e) => {
            panic!(
                "Unable to create a webcam object with index `{}`: {}",
                webcam_index.clone().as_index().unwrap(),
                e
            )
        }
    };

    camera
        .open_stream()
        .unwrap_or_else(|e| panic!("Unable to open camera stream: {}", e));

    world.insert_non_send_resource(camera);
}

pub fn update_webcam_image(
    mut webcam: NonSendMut<nokhwa::Camera>,
    mut webcam_image: ResMut<WebcamImage>,
) {
    let new_frame = webcam
        .frame()
        .unwrap_or_else(|e| panic!("Unable to retrieve frame from webcam: {}", e))
        .decode_image::<RgbAFormat>()
        .unwrap_or_else(|e| panic!("Unable to decode webcam frame: {}", e));

    webcam_image.0 =
        image::ImageBuffer::from_raw(new_frame.width(), new_frame.height(), new_frame.into_raw())
            .expect("Unable to reformat image.");
}
