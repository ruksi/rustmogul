use bevy::prelude::*;
use uuid::Uuid;

use crate::ledger::actions::*;
use crate::ledger::reactions::TileCreated;
use crate::ledger::*;

#[derive(Debug, Clone, Event)]
pub struct CreateTile {
    pub board_id: Uuid,
    pub x: i8,
    pub y: i8,
}

impl CreateTile {
    pub fn new(board_id: Uuid, x: i8, y: i8) -> Self {
        Self { board_id, x, y }
    }
}

impl Action for CreateTile {
    fn allowed_in(&self, ledger: &Ledger) -> bool {
        let board = ledger.boards.get(&self.board_id).unwrap();
        board.tiles.get(&Point::new(self.x, self.y)).is_none()
    }
}

pub fn handle_create_tile(
    mut ledger: ResMut<Ledger>,
    mut actions: EventReader<CreateTile>,
    mut reactions: EventWriter<TileCreated>,
) {
    for action in actions.iter() {
        if !action.allowed_in(&ledger) {
            continue;
        }
        let board = ledger.boards.get_mut(&action.board_id).unwrap();
        board
            .tiles
            .insert(Point::new(action.x, action.y), Tile::default());
        reactions.send(TileCreated {
            board_id: action.board_id,
            x: action.x,
            y: action.y,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ledger::plugin::tests::artificial_ledger;

    use super::*;

    #[test]
    fn can_create_many_tiles() {
        let mut art = artificial_ledger();
        let board_id = art.initialize_standard_board();
        art.send(CreateTile::new(board_id, 1, 2));
        art.with_ledger(|ledger| {
            assert_eq!(32, ledger.boards.get(&board_id).unwrap().tile_count());
        });
        art.assert_forbidden(CreateTile::new(board_id, 1, 2));
        art.assert_allowed(CreateTile::new(board_id, 5, 1));
    }
}
