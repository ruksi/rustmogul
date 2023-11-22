use crate::ledger::database::robot::Robot;

#[derive(Debug, Default, Clone)]
pub struct Tile {
    pub robot: Option<Robot>,
}
