use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};

#[derive(Component)]
pub struct MainCamera;

pub fn setup_windows(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    for mut window in windows.iter_mut() {
        window.mode = WindowMode::BorderlessFullscreen;
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub struct PgiRenderingPlugin;

impl Plugin for PgiRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_windows));
    }
}
