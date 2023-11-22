use bevy::prelude::*;
use rand::prelude::*;
use uuid::Uuid;

use crate::ledger::actions::*;
use crate::ledger::reactions::{GoldChanged, RobotCreated, ShopChanged};
use crate::ledger::Point;

#[derive(Debug, Clone, Event)]
pub struct BuyRobot {
    pub board_id: Uuid,
    pub robot_id: Uuid,
}

impl BuyRobot {
    #[allow(dead_code)]
    pub fn new(board_id: Uuid, robot_id: Uuid) -> Self {
        Self { board_id, robot_id }
    }
}

impl Action for BuyRobot {
    fn allowed_in(&self, ledger: &Ledger) -> bool {
        let &board = &ledger.boards.get(&self.board_id).unwrap();

        let Some(robot) = board.shop.get_robot(self.robot_id) else {
            warn!("BuyRobot Error: robot not in board shop {:?}", self);
            return false;
        };

        if robot.cost > board.gold {
            warn!("BuyRobot Error: not enough gold {:?}", self);
            return false;
        };

        // TODO: does the board have space (on the tiles or bench) for the minion?

        true
    }
}

pub fn handle_buy_robot(
    mut ledger: ResMut<Ledger>,
    mut actions: EventReader<BuyRobot>,
    mut reactions1: EventWriter<ShopChanged>,
    mut reactions2: EventWriter<GoldChanged>,
    mut reactions3: EventWriter<RobotCreated>,
) {
    for action in actions.iter() {
        if !action.allowed_in(&ledger) {
            continue;
        }

        // TODO:
        //  - remove gold
        //  - fix potential infinite loop

        let board = ledger.boards.get_mut(&action.board_id).unwrap();
        let robot = board.shop.take_robot(action.robot_id).unwrap();

        let robot_cost = robot.cost;

        let point_opt: Option<Point>;
        let mut rng = thread_rng();
        loop {
            let chose = board.tiles.iter().choose_multiple(&mut rng, 1);
            let tile_opt = chose.first();
            if let Some((point, tile)) = tile_opt {
                if tile.robot.is_none() {
                    point_opt = Some((*point).clone());
                    break;
                }
            }
        }
        let Some(point) = point_opt else {
            return;
        };

        let old_gold = board.gold;
        board.gold -= robot_cost;

        board.tiles.get_mut(&point).unwrap().robot = Some(robot.clone());
        reactions1.send(ShopChanged {
            board_id: action.board_id,
            shop: board.shop.slots.clone(),
        });
        reactions2.send(GoldChanged {
            board_id: action.board_id,
            old_gold,
            gold: board.gold,
        });
        reactions3.send(RobotCreated {
            board_id: action.board_id,
            robot,
            x: point.x,
            y: point.y,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::ledger::plugin::tests::{artificial_ledger, Art};
    use crate::ledger::Robot;

    use super::*;

    fn my_artificial_ledger() -> (Uuid, Art) {
        let mut art = artificial_ledger();
        let board_id = art.initialize_standard_board();
        (board_id, art)
    }

    #[test]
    fn cannot_buy_imaginary_robot() {
        let (board_id, mut art) = my_artificial_ledger();
        art.assert_forbidden(BuyRobot::new(board_id, Uuid::new_v4()));
    }

    #[test]
    fn can_buy_robot() {
        let (board_id, mut art) = my_artificial_ledger();

        let mut robot_opt: Option<Robot> = None;
        art.with_ledger(|ledger| {
            let board = ledger.boards.get(&board_id).unwrap();
            let robot = board.shop.slots.first().unwrap().clone().unwrap();
            robot_opt = Some(robot.clone());
        });
        let robot = robot_opt.unwrap();
        let robot_id = robot.id;

        art.assert_allowed(BuyRobot::new(board_id, robot_id));
        art.send(BuyRobot::new(board_id, robot_id));

        art.assert_forbidden(BuyRobot::new(board_id, robot_id));
        art.send(BuyRobot::new(board_id, robot_id));
    }
}
