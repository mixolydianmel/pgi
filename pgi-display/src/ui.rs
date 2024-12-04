use bevy::prelude::*;

use crate::apriltag::TagDetectionList;

pub struct PgiUiPlugin;

impl Plugin for PgiUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_elements, setup_health_bars).chain())
            .add_systems(
                Update,
                (update_elements, (update_health, update_health_bars).chain()),
            );
    }
}

#[derive(Component)]
pub struct PgiUiContainer;

pub const PLAYER_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct PgiPlayerNode {
    pub id: usize,
    pub health: f32,
}

#[derive(Component)]
pub struct PgiHealthBarNode;

pub fn setup_elements(mut commands: Commands) {
    let container_node = (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        PgiUiContainer,
    );

    let container = commands.spawn(container_node).id();

    let colors = [
        BackgroundColor(Color::linear_rgb(0.7, 0.7, 0.2)),
        BackgroundColor(Color::linear_rgb(0.2, 0.9, 0.2)),
        BackgroundColor(Color::linear_rgb(0.2, 0.7, 0.7)),
        BackgroundColor(Color::linear_rgb(0.2, 0.2, 0.9)),
        BackgroundColor(Color::linear_rgb(0.7, 0.2, 0.7)),
    ];

    for (i, c) in colors.iter().enumerate() {
        let tracked_node = (
            Node {
                position_type: PositionType::Absolute,
                overflow: Overflow::visible(),
                width: Val::Px(PLAYER_SIZE),
                height: Val::Px(PLAYER_SIZE),
                top: Val::Percent(50.0),
                left: Val::Percent(50.0),
                ..default()
            },
            Visibility::Visible,
            c.clone(),
            PgiPlayerNode { id: i, health: 1.0 },
        );

        let tracked = commands.spawn(tracked_node).id();

        commands.entity(container).insert(PickingBehavior::IGNORE);
        commands.entity(container).add_children(&[tracked]);
    }
}

pub fn setup_health_bars(
    mut commands: Commands,
    players_query: Query<Entity, With<PgiPlayerNode>>,
) {
    players_query.iter().for_each(|entity| {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(PLAYER_SIZE * 1.2),
                    height: Val::Percent(20.0),
                    bottom: Val::Percent(-30.0),
                    left: Val::Percent(-10.0),
                    ..default()
                },
                BackgroundColor(Color::linear_rgb(0.9, 0.2, 0.2)),
                PgiHealthBarNode,
            ));
        });
    });
}

pub fn update_elements(
    mut elems_query: Query<(&PgiPlayerNode, &mut Node, &mut Visibility)>,
    det_map: Res<TagDetectionList>,
) {
    for (player_node, mut node, mut vis) in elems_query.iter_mut() {
        if !det_map.0.contains_key(&player_node.id) {
            *vis = Visibility::Hidden;
            continue;
        }

        *vis = Visibility::Visible;

        let (ss_x, ss_y) = det_map.0.get(&player_node.id).unwrap().transform;
        info!(
            "Translating Entity_ID={}: ({},{})",
            player_node.id, ss_x, ss_y,
        );
        (*node).left = Val::Percent(100.0 * (ss_x + 0.5));
        (*node).top = Val::Percent(100.0 * (ss_y + 0.5));
    }
}

pub fn update_health(
    mut players_query: Query<&mut PgiPlayerNode>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // players_query
    //     .iter_mut()
    //     .for_each(|mut p| (*p).health = time.elapsed_secs().cos().remap(1.0, -1.0, 1.0, 0.0))

    let keys = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
    ];

    let mut presses: Vec<bool> = Vec::new();
    for k in keys {
        presses.push(keyboard.just_pressed(k.clone()));
    }

    for mut player in players_query.iter_mut() {
        if presses.get(player.id).unwrap().clone() {
            if player.health - 0.01 < 0.0 {
                (*player).health = 0.0;
            } else {
                (*player).health -= 0.01;
            }
        };
    }
}

pub fn update_health_bars(
    mut bars_query: Query<(&Parent, &mut Node), With<PgiHealthBarNode>>,
    players_query: Query<&PgiPlayerNode>,
) {
    bars_query.iter_mut().for_each(|(parent, mut node)| {
        let parent_player_node = players_query.get(parent.get()).unwrap();
        (*node).width = Val::Px(PLAYER_SIZE * 1.2 * parent_player_node.health);
    });
}
