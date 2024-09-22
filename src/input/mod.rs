use bevy::app::{Plugin, Startup, Update};
use launch::WebcamIndex;

pub mod launch;
pub mod runtime;

pub struct PgiInputPlugin;

impl Plugin for PgiInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(WebcamIndex(0))
            .add_systems(Startup, launch::parse_args)
            .add_systems(Update, runtime::exit_on_esc);
    }
}
