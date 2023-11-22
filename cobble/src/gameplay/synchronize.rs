use bevy::prelude::*;

use crate::gameplay::board_token::BoardToken;
use crate::gameplay::robot_token::spawn_robot;
use crate::gameplay::tile_token::spawn_tile;
use crate::gameplay::ActiveBoardId;
use crate::ledger::reactions::BoardSynchronized;

pub fn react_to_board_synchronized(
    mut commands: Commands,
    mut reactions: EventReader<BoardSynchronized>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    board_query: Query<Entity, With<BoardToken>>,
    active_board: Res<ActiveBoardId>,
) {
    let board_entity = board_query.single();
    for reaction in reactions.iter() {
        if !active_board.is(reaction.board_id) {
            continue;
        }
        debug!("Active board synchronized");

        commands.entity(board_entity).despawn_descendants();

        let fresh_tiles: &Vec<(Entity, i8, i8)> = &reaction
            .tiles
            .iter()
            .map(|(x, y)| {
                (
                    spawn_tile(&mut commands, &mut meshes, &mut materials, board_entity, *x, *y),
                    *x,
                    *y,
                )
            })
            .collect();

        for robot in &reaction.robots {
            let (terminus_tile, _, _) = *fresh_tiles
                .iter()
                .find(|(_, x, y)| x == &robot.0 && y == &robot.1)
                .unwrap();
            spawn_robot(&mut commands, &mut meshes, &mut materials, terminus_tile, robot.2.clone());
        }

        // TODO: trigger "select none" or somehow hide robot details
    }
}
