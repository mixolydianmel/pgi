use apriltag::{Detector, Family, Image};
use apriltag_image::prelude::*;
use bevy::{prelude::*, utils::HashMap};

use crate::webcam::WebcamImage;

const DETECTION_TIMEOUT: f32 = 0.5;

pub struct PgiAprilTagPlugin;

impl Plugin for PgiAprilTagPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TagDetectionList>()
            .add_systems(Startup, setup_detector)
            .add_systems(Update, (update_detections, clean_detections).chain());
    }
}

pub struct TagDetectionInfo {
    pub transform: (f32, f32),
    pub last_update: f32,
}

#[derive(Resource, Default)]
pub struct TagDetectionList(pub HashMap<usize, TagDetectionInfo>);

pub fn setup_detector(world: &mut World) {
    let detector = Detector::builder()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .unwrap_or_else(|e| panic!("Unable to create AprilTag Detector object: {}", e));
    world.insert_non_send_resource(detector);
}

pub fn update_detections(
    mut detector: NonSendMut<Detector>,
    mut det_map: ResMut<TagDetectionList>,
    webcam_image: Res<WebcamImage>,
    time: Res<Time>,
) {
    let webcam_luma = image::DynamicImage::ImageRgba8(webcam_image.0.clone()).to_luma8();
    let img = Image::from_image_buffer(&webcam_luma);
    let detections = detector.detect(&img);
    detections
        .into_iter()
        .filter(|det| det.decision_margin() > 33.0)
        .for_each(|det| {
            debug!(
                "Detected tag ID {} with {} certainty",
                det.id(),
                det.decision_margin()
            );
            let x: f32 = (det.center()[0] as f32 / webcam_image.0.width() as f32) - 0.5;
            let y: f32 = (1.0 - det.center()[1] as f32 / webcam_image.0.height() as f32) - 0.5;
            debug!("Tag at ({x}, {y})");
            det_map.0.insert(
                det.id(),
                TagDetectionInfo {
                    transform: (x, y),
                    last_update: time.elapsed_seconds(),
                },
            );
        });
}

pub fn clean_detections(mut det_map: ResMut<TagDetectionList>, time: Res<Time>) {
    det_map
        .0
        .retain(|_, value| time.elapsed_seconds() - value.last_update < DETECTION_TIMEOUT);
}
