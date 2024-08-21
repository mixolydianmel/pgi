use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{TextureDimension, TextureUsages},
    },
    window::{PrimaryWindow, WindowMode},
};

use crate::webcam::{setup_webcam, WebcamImage};

pub struct PgiRenderingPlugin;

impl Plugin for PgiRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.0)))
            .add_systems(Startup, (setup_camera, setup_windows));
    }
}

pub struct PgiDebugCameraView;

impl Plugin for PgiDebugCameraView {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugCameraView>()
            .add_systems(Startup, debug_setup_camview.after(setup_webcam))
            .add_systems(Update, debug_update_camview);
    }
}

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

#[derive(Resource, Default)]
pub struct DebugCameraView(pub Handle<Image>);

pub fn debug_setup_camview(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut debug_camview: ResMut<DebugCameraView>,
    webcam: NonSend<nokhwa::Camera>,
) {
    let mut bg_img = Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: webcam.resolution().width(),
            height: webcam.resolution().height(),
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    bg_img.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;

    debug_camview.0 = images.add(bg_img);

    commands.spawn(ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: debug_camview.0.clone().into(),
        transform: Transform::from_xyz(0.0, 0.0, -100.0),
        ..default()
    });
}

pub fn debug_update_camview(
    mut images: ResMut<Assets<Image>>,
    debug_camview: Res<DebugCameraView>,
    webcam: Res<WebcamImage>,
) {
    let image = images
        .get_mut(&debug_camview.0.clone())
        .unwrap_or_else(|| panic!("Unable to load resource"));
    image.data.copy_from_slice(webcam.0.as_raw());
}
