use hashbrown::HashMap;
use uuid::Uuid;

use crate::ledger::database::shop::Shop;
use crate::ledger::{Point, Tile};

#[derive(Debug, Clone)]
pub struct Board {
    pub player_id: Uuid,
    pub gold: u16,
    pub shop: Shop,
    pub tiles: HashMap<Point, Tile>,
}

impl Board {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            gold: 0,
            shop: Shop::default(),
            tiles: HashMap::new(),
        }
    }
    pub fn initialize(&mut self) {
        self.tiles.insert(Point::new(-4, 0), Tile::default());
        self.tiles.insert(Point::new(-4, 1), Tile::default());
        self.tiles.insert(Point::new(-4, 2), Tile::default());
        self.tiles.insert(Point::new(-4, 3), Tile::default());
        self.tiles.insert(Point::new(-3, 0), Tile::default());
        self.tiles.insert(Point::new(-3, 1), Tile::default());
        self.tiles.insert(Point::new(-3, 2), Tile::default());
        self.tiles.insert(Point::new(-3, 3), Tile::default());
        self.tiles.insert(Point::new(-2, 0), Tile::default());
        self.tiles.insert(Point::new(-2, 1), Tile::default());
        self.tiles.insert(Point::new(-2, 2), Tile::default());
        self.tiles.insert(Point::new(-2, 3), Tile::default());
        self.tiles.insert(Point::new(-1, 0), Tile::default());
        self.tiles.insert(Point::new(-1, 1), Tile::default());
        self.tiles.insert(Point::new(-1, 2), Tile::default());
        self.tiles.insert(Point::new(-1, 3), Tile::default());
        self.tiles.insert(Point::new(0, 0), Tile::default());
        self.tiles.insert(Point::new(0, 1), Tile::default());
        self.tiles.insert(Point::new(0, 2), Tile::default());
        self.tiles.insert(Point::new(0, 3), Tile::default());
        self.tiles.insert(Point::new(1, 0), Tile::default());
        self.tiles.insert(Point::new(1, 1), Tile::default());
        self.tiles.insert(Point::new(1, 2), Tile::default());
        self.tiles.insert(Point::new(1, 3), Tile::default());
        self.tiles.insert(Point::new(2, 0), Tile::default());
        self.tiles.insert(Point::new(2, 1), Tile::default());
        self.tiles.insert(Point::new(2, 2), Tile::default());
        self.tiles.insert(Point::new(2, 3), Tile::default());
        self.tiles.insert(Point::new(3, 0), Tile::default());
        self.tiles.insert(Point::new(3, 1), Tile::default());
        self.tiles.insert(Point::new(3, 2), Tile::default());
        self.tiles.insert(Point::new(3, 3), Tile::default());
        self.gold += 5;
    }
}

impl Board {
    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }
    pub fn robot_count(&self) -> usize {
        self.tiles.iter().filter(|(_, t)| t.robot.is_some()).count()
    }
}
