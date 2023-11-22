pub use database::*;
pub use plugin::*;

pub mod actions;
mod database;
mod plugin;
pub mod reactions;
pub mod reflexes;
pub mod resource;

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use uuid::Uuid;
    use crate::ledger::actions::CreateRobot;

    use crate::ledger::plugin::tests::{Art, artificial_ledger};
    use crate::ledger::reactions::*;

    fn my_artificial_ledger() -> (Uuid, Art) {
        let mut art = artificial_ledger();

        let board_id = art.initialize_standard_board();

        // how to listen for events...
        fn robot_listener(mut events: EventReader<RobotCreated>) {
            for _event in events.iter() {
                // println!("ROBOT CREATED: {:?}", _event);
            }
        }
        art.app.add_systems(Update, robot_listener.run_if(on_event::<RobotCreated>()));

        art.send(CreateRobot::new(board_id, "barker".into(), 0, 0));
        art.send(CreateRobot::new(board_id, "microwave".into(), 0, 1));
        (board_id, art)
    }

    #[test]
    fn query_tester() {
        let (board_id, mut art) = my_artificial_ledger();
        art.with_ledger(|ledger| {
            let board = ledger.boards.get(&board_id).unwrap();
            let tiles_with_robot = board.tiles.iter().filter(|(_, t)| t.robot.is_some());
            // for (point, tile) in tiles_with_robot {
            //     println!("{}: ({:?}, {:?})", &tile.robot.as_ref().unwrap().name, point.x, point.y);
            // }
            assert_eq!(tiles_with_robot.count(), 2);
        });
    }
}
