use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_fixed_timer;

use crate::ledger::actions::*;
use crate::ledger::reactions;
use crate::ledger::resource::Ledger;

pub struct LedgerPlugin;

#[rustfmt::skip]
impl Plugin for LedgerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Ledger>();

        app.add_event::<reactions::BoardSynchronized>();
        app.add_event::<reactions::ConflictStarted>();
        app.add_event::<reactions::GoldChanged>();
        app.add_event::<reactions::RobotCreated>();
        app.add_event::<reactions::RobotMoved>();
        app.add_event::<reactions::ShopChanged>();
        app.add_event::<reactions::TileCreated>();

        // TODO: when the plugin isn't tied to the game rendering, use fixed update?

        app.add_event::<SynchronizeBoard>();
        app.add_systems(Update, handle_synchronize_board.run_if(on_event::<SynchronizeBoard>()));

        app.add_event::<StartConflict>();
        app.add_systems(Update, handle_start_conflict.run_if(on_event::<StartConflict>()));

        app.add_event::<BuyRobot>();
        app.add_systems(Update, handle_buy_robot.run_if(on_event::<BuyRobot>()));

        app.add_event::<CreateRobot>();
        app.add_systems(Update, handle_create_robot.run_if(on_event::<CreateRobot>()));

        app.add_event::<MoveRobot>();
        app.add_systems(Update, handle_move_robot.run_if(on_event::<MoveRobot>()));

        app.add_event::<RerollShop>();
        app.add_systems(Update, handle_reroll_shop.run_if(on_event::<RerollShop>()));

        app.add_event::<CreateTile>();
        app.add_systems(Update, handle_create_tile.run_if(on_event::<CreateTile>()));

        // TODO: this could be smarter... maybe use states if when not tried to game world?
        app.add_systems(Update, check_pending_conflicts.run_if(on_fixed_timer(Duration::from_millis(400))));
    }
}

pub fn check_pending_conflicts(ledger: Res<Ledger>, mut actions: EventWriter<StartConflict>) {
    if !ledger.started && ledger.boards.iter().count() >= 2 {
        actions.send(StartConflict {});
    }
}

impl LedgerPlugin {
    pub fn cleanup(mut ledger: ResMut<Ledger>) {
        ledger.clear();
    }
}

#[cfg(test)]
pub mod tests {
    use bevy::app::MainScheduleOrder;
    use uuid::Uuid;

    use crate::ledger::Board;

    use super::*;

    pub fn ledger_in_app() -> App {
        let mut app = App::empty();
        // let mut main_schedule = Schedule::new();
        // main_schedule.set_executor_kind(ExecutorKind::SingleThreaded);
        app.init_resource::<Time>();
        app.init_resource::<FixedTime>();
        app.add_schedule(Main, Schedule::new());
        app.init_resource::<MainScheduleOrder>();
        app.add_systems(Main, Main::run_main);
        app.add_plugins(LedgerPlugin);
        app
    }

    pub fn artificial_ledger() -> Art {
        Art { app: ledger_in_app() }
    }

    // Art aka. Artificial Ledger wrapped in an App for assertions
    pub struct Art {
        pub app: App,
    }

    impl Art {
        pub fn initialize_standard_board(&mut self) -> Uuid {
            let board_id = Uuid::new_v4();
            self.with_ledger(|mut ledger| {
                ledger.pool.initialize();

                let starter_shop = ledger.pool.take_random_robots_upto_tier(1, 5).clone();

                ledger.boards.insert(board_id, Board::new(Uuid::new_v4()));
                let board = ledger.boards.get_mut(&board_id).unwrap();
                board.initialize();

                board.shop.add_robots(starter_shop);

                ledger.started = true;
            });
            board_id
        }

        pub fn send<T: Event>(&mut self, event: T) {
            self.app.world.send_event(event);
            self.app.update();
        }

        pub fn with_ledger(&mut self, f: impl FnOnce(Mut<Ledger>)) {
            self.app.world.resource_scope(|_, ledger: Mut<Ledger>| {
                f(ledger);
            });
        }

        pub fn assert_allowed<T: Action>(&mut self, action: T) {
            self.with_ledger(|ledger| {
                assert!(action.allowed_in(&ledger));
            });
        }

        pub fn assert_forbidden<T: Action>(&mut self, action: T) {
            self.with_ledger(|ledger| {
                assert!(!action.allowed_in(&ledger));
            });
        }
    }
}
