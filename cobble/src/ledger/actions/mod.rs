use bevy::prelude::*;

pub use buy_robot::*;
pub use create_robot::*;
pub use create_tile::*;
pub use move_robot::*;
pub use reroll_shop::*;
pub use start_conflict::*;
pub use synchronized_board::*;

use crate::ledger::resource::Ledger;

mod buy_robot;
mod create_robot;
mod create_tile;
mod move_robot;
mod reroll_shop;
mod start_conflict;
mod synchronized_board;

// TODO: implement some mechanism to get all Actions and initialize them in the plugin...
pub trait Action: Event {
    fn allowed_in(&self, ledger: &Ledger) -> bool;
}
