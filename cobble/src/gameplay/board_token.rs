use bevy::prelude::*;
use uuid::Uuid;

use crate::cleanup::CleanOnPlayExit;
use crate::ledger::actions::SynchronizeBoard;

#[derive(Debug, Default, Resource)]
pub struct BoardDatabase {
    pub board_ids: Vec<Uuid>,
}

#[derive(Debug, Resource)]
pub struct ActiveBoardId(pub Uuid);

impl ActiveBoardId {
    pub fn is(&self, other: Uuid) -> bool {
        self.0 == other
    }
}

#[derive(Component, Debug)]
pub struct BoardToken;

pub fn handle_active_board_change(
    mut actions: EventWriter<SynchronizeBoard>,
    active_board: Res<ActiveBoardId>,
) {
    if active_board.is_changed() && !active_board.is_added() {
        debug!("Active board has changed to {:?}", active_board.0);

        // TODO:
        //  * [x] send an action to re-sync
        //  * [] go into no-updates state :shrug:
        //  * [x] the reacting system should be exclusive and delete the old board

        actions.send(SynchronizeBoard { board_id: active_board.0 });
    }
}

pub fn spawn_board(mut commands: Commands) {
    commands
        .spawn(BoardToken)
        .insert(SpatialBundle::default())
        .insert(CleanOnPlayExit);
}
