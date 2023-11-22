use bevy::prelude::*;
use uuid::Uuid;

use crate::ledger::actions::*;
use crate::ledger::reactions::BoardSynchronized;

#[derive(Debug, Clone, Event)]
pub struct SynchronizeBoard {
    pub board_id: Uuid,
}

impl Action for SynchronizeBoard {
    fn allowed_in(&self, _ledger: &Ledger) -> bool {
        true
    }
}

pub fn handle_synchronize_board(
    mut ledger: ResMut<Ledger>,
    mut actions: EventReader<SynchronizeBoard>,
    mut reactions: EventWriter<BoardSynchronized>,
) {
    for action in actions.iter() {
        if !action.allowed_in(&ledger) {
            continue;
        }

        let board = ledger.boards.get_mut(&action.board_id).unwrap();
        reactions.send(BoardSynchronized {
            board_id: action.board_id,
            gold: board.gold,
            shop: board.shop.slots.clone(),
            robots: board
                .tiles
                .iter()
                .filter(|(_, t)| t.robot.is_some())
                .map(|(p, t)| (p.x, p.y, t.robot.as_ref().unwrap().clone()))
                .collect(),
            tiles: board.tiles.iter().map(|(p, _t)| (p.x, p.y)).collect(),
        });
    }
}
