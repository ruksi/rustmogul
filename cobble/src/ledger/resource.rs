use bevy::prelude::Resource;
use hashbrown::HashMap;
use uuid::Uuid;

use crate::ledger::{Board, Pool};

type BoardId = Uuid;

#[derive(Debug, Default, Resource)]
pub struct Ledger {
    pub started: bool,
    pub pool: Pool,
    pub boards: HashMap<BoardId, Board>,
}
