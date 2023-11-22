use bevy::asset::Assets;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::control::PointerEvent;
use crate::gameplay::tile_token::TileToken;
use crate::gameplay::ActiveBoardId;
use crate::ledger::actions::MoveRobot;
use crate::ledger::reactions::{RobotCreated, RobotMoved};
use crate::ledger::Robot;

#[derive(Component, Debug)]
pub struct RobotToken {
    pub robot: Option<Robot>,
}

pub fn react_to_robot_created(
    mut commands: Commands,
    mut reactions: EventReader<RobotCreated>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tile_query: Query<(Entity, &TileToken)>,
    active_board: Res<ActiveBoardId>,
) {
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        let Some((tile_entity, _)) = tile_query
            .iter()
            .find(|(_, tile)| tile.x == reaction.x && tile.y == reaction.y)
        else {
            return;
        };
        spawn_robot(
            &mut commands,
            &mut meshes,
            &mut materials,
            tile_entity,
            reaction.robot.clone(),
        );
    }
}

pub fn react_to_robot_moved(
    mut commands: Commands,
    mut reactions: EventReader<RobotMoved>,
    tile_query: Query<(Entity, &TileToken)>,
    children_query: Query<&Children>,
    robot_query: Query<(Entity, &RobotToken)>,
    active_board: Res<ActiveBoardId>,
) {
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        let (source_entity, _) = tile_query
            .iter()
            .find(|(_, tile)| tile.x == reaction.from_x && tile.y == reaction.from_y)
            .unwrap();
        let (terminus_entity, _) = tile_query
            .iter()
            .find(|(_, tile)| tile.x == reaction.to_x && tile.y == reaction.to_y)
            .unwrap();
        for tile_child in children_query.iter_descendants(source_entity) {
            if let Ok((robot_entity, _)) = robot_query.get(tile_child) {
                commands.entity(robot_entity).set_parent(terminus_entity);
            };
        }
    }
}

pub fn act_on_robot_or_tile_drop(
    mut events: EventReader<PointerEvent>,
    mut actions: EventWriter<MoveRobot>,
    tile_query: Query<(Entity, &TileToken)>,
    parent_query: Query<&Parent>,
    active_board: Res<ActiveBoardId>,
) {
    let event = events
        .iter()
        .find(|&e| matches!(e, PointerEvent::OnDrop { .. }));
    let Some(PointerEvent::OnDrop { target, dropped }) = event else {
        return;
    };

    let from_dropped = tile_query.get(*dropped).ok();
    let from_dropped_parent = || {
        parent_query
            .get(*dropped)
            .ok()
            .and_then(|robot_parent| tile_query.get(robot_parent.get()).ok())
    };
    let Some((_, source_tile)) = from_dropped.or_else(from_dropped_parent) else {
        debug!("Invalid drop source: {:?}", dropped);
        return;
    };

    let from_target = tile_query.get(*target).ok();
    let from_target_parent = || {
        parent_query
            .get(*target)
            .ok()
            .and_then(|parent| tile_query.get(parent.get()).ok())
    };
    let Some((_, terminus_tile)) = from_target.or_else(from_target_parent) else {
        debug!("Invalid robot drop target: {:?}", target);
        return;
    };

    actions.send(MoveRobot {
        board_id: active_board.0,
        from_x: source_tile.x,
        from_y: source_tile.y,
        to_x: terminus_tile.x,
        to_y: terminus_tile.y,
    })
}

pub fn spawn_robot(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    robot: Robot,
) {
    let robot_color = robot.color();
    commands
        .spawn((
            Name::from(format!("RobotToken({})", robot.id)),
            RobotToken { robot: Some(robot) },
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.15 })),
                material: materials.add(robot_color.into()),
                transform: Transform::from_xyz(0.0, 0.075, 0.0),
                ..default()
            },
            // SWITCH THESE ON AND COMMENT OFF PICK IGNORE AND REMOVE SELECTION
            // TO BE ABLE TO DRAG THE MODELS THEMSELVES, NOT SURE IF NECESSARY
            // PickableBundle::default(),
            // RaycastPickTarget::default(),
            // On::<Pointer<DragStart>>::target_component_mut::<Visibility>(|_event, visibility| {
            //     // TODO: show icon
            //     *visibility = Visibility::Hidden;
            // }),
            // On::<Pointer<DragEnd>>::target_commands_mut(|_event, commands| {
            //     // TODO: hide icon
            //     commands.insert(SetVisibilitySoon {
            //         timer: Timer::from_seconds(0.05, TimerMode::Once),
            //         visibility: Visibility::Inherited,
            //     });
            // }),
            Pickable::IGNORE,
            NoDeselect,
        ))
        .remove::<PickSelection>()
        .set_parent(parent);
}
