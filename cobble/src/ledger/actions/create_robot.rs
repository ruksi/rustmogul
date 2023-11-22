use bevy::prelude::*;
use uuid::Uuid;

use crate::ledger::actions::*;
use crate::ledger::reactions::RobotCreated;
use crate::ledger::*;

#[derive(Debug, Clone, Event)]
pub struct CreateRobot {
    pub board_id: Uuid,
    pub prototype_id: String,
    pub x: i8,
    pub y: i8,
}

impl CreateRobot {
    #[allow(dead_code)]
    pub fn new(board_id: Uuid, prototype_id: String, x: i8, y: i8) -> Self {
        Self { board_id, prototype_id, x, y }
    }
}

impl Action for CreateRobot {
    fn allowed_in(&self, ledger: &Ledger) -> bool {
        let &board = &ledger.boards.get(&self.board_id).unwrap();

        if !Robot::is_prototype_id(self.prototype_id.as_str()) {
            warn!("CreateRobot Error: bad prototype id {:?}", self);
            return false;
        }

        let Some(tile) = board.tiles.get(&Point::new(self.x, self.y)) else {
            warn!("CreateRobot Error: bad destination tile {:?}", self);
            return false;
        };

        if let Some(_robot) = &tile.robot {
            warn!("CreateRobot Error: tile already has a robot {:?}", self);
            return false;
        };

        true
    }
}

pub fn handle_create_robot(
    mut ledger: ResMut<Ledger>,
    mut actions: EventReader<CreateRobot>,
    mut reactions: EventWriter<RobotCreated>,
) {
    for action in actions.iter() {
        if !action.allowed_in(&ledger) {
            continue;
        }

        let board = ledger.boards.get_mut(&action.board_id).unwrap();

        let robot = Robot::from_prototype_id(action.prototype_id.as_str()).unwrap();

        board
            .tiles
            .get_mut(&Point::new(action.x, action.y))
            .unwrap()
            .robot = Some(robot.clone());

        reactions.send(RobotCreated {
            board_id: action.board_id,
            robot,
            x: action.x,
            y: action.y,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::ledger::plugin::tests::{artificial_ledger, Art};

    use super::*;

    fn my_artificial_ledger() -> (Uuid, Art) {
        let mut art = artificial_ledger();
        let board_id = art.initialize_standard_board();
        (board_id, art)
    }

    #[test]
    fn can_create_many_robots() {
        let (board_id, mut art) = my_artificial_ledger();
        assert_robot_count(&mut art, &board_id, 0);
        art.assert_allowed(CreateRobot::new(board_id, "barker".into(), 0, 0));

        art.send(CreateRobot::new(board_id, "barker".into(), 0, 0));
        assert_robot_count(&mut art, &board_id, 1);
        art.assert_forbidden(CreateRobot::new(board_id, "barker".into(), 0, 0));

        art.send(CreateRobot::new(board_id, "magic_missile".into(), 1, 0));
        assert_robot_count(&mut art, &board_id, 2);
    }

    #[test]
    fn cannot_create_overlapping_robots() {
        let (board_id, mut art) = my_artificial_ledger();
        assert_robot_count(&mut art, &board_id, 0);

        art.send(CreateRobot::new(board_id, "barker".into(), 0, 0));
        assert_robot_count(&mut art, &board_id, 1);
        art.assert_forbidden(CreateRobot::new(board_id, "magic_missile".into(), 0, 0));

        art.send(CreateRobot::new(board_id, "magic_missile".into(), 0, 0));
        assert_robot_count(&mut art, &board_id, 1);
    }

    #[test]
    fn cannot_create_robots_outside_of_tiles() {
        let (board_id, mut art) = my_artificial_ledger();
        assert_robot_count(&mut art, &board_id, 0);
        art.assert_forbidden(CreateRobot::new(board_id, "barker".into(), -1, -1));
        art.send(CreateRobot::new(board_id, "barker".into(), -1, -1));
        assert_robot_count(&mut art, &board_id, 0);
    }

    fn assert_robot_count(art: &mut Art, board_id: &Uuid, count: usize) {
        art.with_ledger(|ledger| {
            assert_eq!(count, ledger.boards.get(board_id).unwrap().robot_count());
        });
    }
}
