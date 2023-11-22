use bevy::prelude::*;
use uuid::Uuid;

use crate::ledger::actions::*;
use crate::ledger::reactions::{GoldChanged, ShopChanged};

#[derive(Debug, Clone, Event)]
pub struct RerollShop {
    pub board_id: Uuid,
}

impl RerollShop {
    #[allow(dead_code)]
    pub fn new(board_id: Uuid) -> Self {
        Self { board_id }
    }
}

impl Action for RerollShop {
    fn allowed_in(&self, ledger: &Ledger) -> bool {
        let &board = &ledger.boards.get(&self.board_id).unwrap();
        if board.gold < 2 {
            warn!("RerollShop Error: not enough gold {:?}", self);
            return false;
        };
        true
    }
}

pub fn handle_reroll_shop(
    mut ledger: ResMut<Ledger>,
    mut actions: EventReader<RerollShop>,
    mut reactions1: EventWriter<ShopChanged>,
    mut reactions2: EventWriter<GoldChanged>,
) {
    for action in actions.iter() {
        if !action.allowed_in(&ledger) {
            continue;
        }

        let mut pool = ledger.pool.clone();

        let board = ledger.boards.get_mut(&action.board_id).unwrap();
        board.shop.return_to(&mut pool);
        board
            .shop
            .add_robots(pool.take_random_robots_upto_tier(1, 5));

        let old_gold = board.gold;
        board.gold -= 2;

        reactions1.send(ShopChanged {
            board_id: action.board_id,
            shop: board.shop.slots.clone(),
        });
        reactions2.send(GoldChanged {
            board_id: action.board_id,
            old_gold,
            gold: board.gold,
        });

        ledger.pool = pool;
    }
}
