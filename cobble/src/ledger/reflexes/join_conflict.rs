use uuid::Uuid;

use crate::ledger::resource::Ledger;
use crate::ledger::Board;

type BoardId = Uuid;

impl Ledger {
    pub fn join_conflict(&mut self, _conflict_id: Uuid, player_id: Uuid) -> BoardId {
        if self.started {
            panic!("Conflict already started but trying to join it");
        }
        let board_id = Uuid::new_v4();
        self.boards.insert(board_id, Board::new(player_id));
        board_id
    }
}
