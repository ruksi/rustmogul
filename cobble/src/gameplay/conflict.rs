use bevy::prelude::*;
use uuid::Uuid;

use crate::gameplay::board_token::BoardDatabase;
use crate::gameplay::ActiveBoardId;
use crate::ledger::resource::Ledger;

pub fn exclusively_initialize_conflict(world: &mut World) {
    debug!("Initialize board database and select active board");

    // TODO: fix when there are implemented :P
    let conflict_id = Uuid::new_v4();
    let player_id = Uuid::new_v4();

    let mut first_board_id: Option<Uuid> = None;
    world.resource_scope(|_, mut ledger: Mut<Ledger>| {
        let board_id = ledger.join_conflict(conflict_id, player_id);
        first_board_id = Some(board_id);
    });

    let mut second_board_id: Option<Uuid> = None;
    world.resource_scope(|_, mut ledger: Mut<Ledger>| {
        let board_id = ledger.join_conflict(conflict_id, player_id);
        second_board_id = Some(board_id);
    });

    let mut database = world.resource_mut::<BoardDatabase>();
    database.board_ids = vec![first_board_id.unwrap(), second_board_id.unwrap()];

    let active = ActiveBoardId(database.board_ids[0]);
    world.insert_resource(active);
}
