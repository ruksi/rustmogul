use bevy::prelude::*;
use uuid::Uuid;

use crate::ledger::actions::*;
use crate::ledger::reactions::RobotMoved;
use crate::ledger::*;

#[derive(Debug, Clone, Event)]
pub struct MoveRobot {
    pub board_id: Uuid,
    pub from_x: i8,
    pub from_y: i8,
    pub to_x: i8,
    pub to_y: i8,
}

impl MoveRobot {
    pub fn new(board_id: Uuid, from: (i8, i8), to: (i8, i8)) -> Self {
        Self {
            board_id,
            from_x: from.0,
            from_y: from.1,
            to_x: to.0,
            to_y: to.1,
        }
    }
}

impl Action for MoveRobot {
    fn allowed_in(&self, ledger: &Ledger) -> bool {
        let board = ledger.boards.get(&self.board_id).unwrap();

        if board.tiles.get(&Point::new(self.to_x, self.to_y)).is_none() {
            warn!("MoveRobot Error: invalid destination tile {:?}", self);
            return false;
        };

        let Some(tile) = board.tiles.get(&Point::new(self.from_x, self.from_y)) else {
            warn!("MoveRobot Error: invalid origin tile {:?}", self);
            return false;
        };

        if tile.robot.is_none() {
            warn!("MoveRobot Error: no robot on origin tile {:?}", self);
            return false;
        }

        true
    }
}

pub fn handle_move_robot(
    mut ledger: ResMut<Ledger>,
    mut actions: EventReader<MoveRobot>,
    mut reactions: EventWriter<RobotMoved>,
) {
    for action in actions.iter() {
        if !action.allowed_in(&ledger) {
            continue;
        }

        let board = ledger.boards.get_mut(&action.board_id).unwrap();
        let origin = Point::new(action.from_x, action.from_y);
        let terminus = Point::new(action.to_x, action.to_y);

        if let Some([origin_tile, terminus_tile]) = board.tiles.get_many_mut([&origin, &terminus]) {
            if terminus_tile.robot.is_some() {
                reactions.send(RobotMoved {
                    board_id: action.board_id,
                    from_x: action.to_x,
                    from_y: action.to_y,
                    to_x: action.from_x,
                    to_y: action.from_y,
                });
            }
            if origin_tile.robot.is_some() {
                reactions.send(RobotMoved {
                    board_id: action.board_id,
                    from_x: action.from_x,
                    from_y: action.from_y,
                    to_x: action.to_x,
                    to_y: action.to_y,
                });
            }
            // NB: this swaps between two Option<Robot>s so also None <-> Some
            std::mem::swap(&mut origin_tile.robot, &mut terminus_tile.robot);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ledger::plugin::tests::{artificial_ledger, Art};

    use super::*;

    fn my_artificial_ledger() -> (Uuid, Art) {
        let mut art = artificial_ledger();
        let board_id = art.initialize_standard_board();
        art.send(CreateRobot::new(board_id, "barker".into(), 0, 0));
        art.with_ledger(|ledger| {
            assert_eq!(1, ledger.boards.get(&board_id).unwrap().robot_count());
        });
        (board_id, art)
    }

    #[test]
    fn cannot_move_from_unexisting() {
        let (board_id, mut art) = my_artificial_ledger();
        art.assert_forbidden(MoveRobot::new(board_id, (-1, -1), (0, 0)));
    }

    #[test]
    fn cannot_move_to_unexisting() {
        let (board_id, mut art) = my_artificial_ledger();
        art.assert_forbidden(MoveRobot::new(board_id, (0, 0), (-1, -1)));
    }

    #[test]
    fn cannot_move_imaginary_robot() {
        let (board_id, mut art) = my_artificial_ledger();
        art.assert_forbidden(MoveRobot::new(board_id, (1, 1), (0, 1)));
    }

    #[test]
    fn can_move_robot() {
        let (board_id, mut art) = my_artificial_ledger();
        assert_robot_name(&mut art, board_id, 0, 0, "barker");

        art.send(MoveRobot::new(board_id, (0, 0), (0, 1)));
        assert_robot_name(&mut art, board_id, 0, 1, "barker");
    }

    #[test]
    fn can_swap_robot() {
        let (board_id, mut art) = my_artificial_ledger();
        art.send(CreateRobot::new(board_id, "magic_missile".into(), 0, 1));
        assert_robot_name(&mut art, board_id, 0, 0, "barker");
        assert_robot_name(&mut art, board_id, 0, 1, "magic_missile");

        art.send(MoveRobot::new(board_id, (0, 0), (0, 1)));
        assert_robot_name(&mut art, board_id, 0, 0, "magic_missile");
        assert_robot_name(&mut art, board_id, 0, 1, "barker");
    }

    fn assert_robot_name(art: &mut Art, board_id: Uuid, x: i8, y: i8, name: &str) {
        art.with_ledger(|ledger| {
            let point = Point::new(x, y);
            let board = ledger.boards.get(&board_id).unwrap();
            let tile = board.tiles.get(&point).unwrap();
            let robot = tile.robot.as_ref().unwrap();
            assert_eq!(robot.prototype_id, name);
        });
    }
}
