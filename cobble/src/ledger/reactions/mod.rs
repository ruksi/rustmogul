use bevy::prelude::Event;
use uuid::Uuid;

use rustmogul_derive::Reaction;

use crate::ledger::Robot;

// TODO: implement some mechanism to get all Reactions and initialize them in the plugin...
pub trait Reaction: Event {}

#[derive(Debug, Clone, Event, Reaction)]
pub struct ConflictStarted;

#[derive(Debug, Clone, Event, Reaction)]
pub struct BoardSynchronized {
    pub board_id: Uuid,
    pub gold: u16,
    pub shop: Vec<Option<Robot>>,
    pub robots: Vec<(i8, i8, Robot)>,
    pub tiles: Vec<(i8, i8)>,
}

#[derive(Debug, Clone, Event, Reaction)]
pub struct GoldChanged {
    pub board_id: Uuid,
    pub old_gold: u16,
    pub gold: u16,
}

#[derive(Debug, Clone, Event, Reaction)]
pub struct RobotCreated {
    pub board_id: Uuid,
    pub robot: Robot,
    pub x: i8,
    pub y: i8,
}

#[derive(Debug, Clone, Event, Reaction)]
pub struct RobotMoved {
    pub board_id: Uuid,
    pub from_x: i8,
    pub from_y: i8,
    pub to_x: i8,
    pub to_y: i8,
}

#[derive(Debug, Clone, Event, Reaction)]
pub struct ShopChanged {
    pub board_id: Uuid,
    pub shop: Vec<Option<Robot>>,
}

#[derive(Debug, Clone, Event, Reaction)]
pub struct TileCreated {
    pub board_id: Uuid,
    pub x: i8,
    pub y: i8,
}
