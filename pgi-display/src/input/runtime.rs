use bevy::{prelude::*, sprite::Mesh2dHandle};

pub fn clear_on_space(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mesh_entities: Query<Entity, With<Mesh2dHandle>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for entity in mesh_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn exit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
