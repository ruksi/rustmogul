use bevy::prelude::*;

use crate::ledger::actions::SynchronizeBoard;
use crate::ledger::reactions::ConflictStarted;
use crate::ledger::resource::Ledger;

#[derive(Debug, Clone, Event)]
pub struct StartConflict;

pub fn handle_start_conflict(
    mut ledger: ResMut<Ledger>,
    mut actions: EventReader<StartConflict>,
    mut reactions: EventWriter<ConflictStarted>,
    mut sync_actions: EventWriter<SynchronizeBoard>,
) {
    for _action in actions.iter() {
        if ledger.started {
            panic!("Trying to start conflict on board that is already started");
        }
        ledger.pool.initialize();

        let board_indices: Vec<_> = ledger.boards.keys().cloned().collect();
        let mut starter_shops: Vec<_> = board_indices
            .iter()
            .map(|_| ledger.pool.take_random_robots_upto_tier(1, 5).clone())
            .collect();

        for board_id in board_indices {
            let starter_shop = starter_shops.pop().unwrap();
            let board = ledger.boards.get_mut(&board_id).unwrap();
            board.initialize();
            sync_actions.send(SynchronizeBoard { board_id });
            board.shop.add_robots(starter_shop);
        }
        ledger.started = true;
        reactions.send(ConflictStarted {});
        return;
    }
    panic!("Triggered start conflict handling even no conflicts were to start")
}
