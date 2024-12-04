use bevy::prelude::*;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Sets the index of which webcam to use
    #[arg(short, long, value_name = "INDEX", default_value_t = 0)]
    camera_index: u32,
}

#[derive(Resource, Default)]
pub struct WebcamIndex(pub u32);

pub fn parse_args(mut webcam_index: ResMut<WebcamIndex>) {
    let args = Args::parse();
    webcam_index.0 = args.camera_index;
}
