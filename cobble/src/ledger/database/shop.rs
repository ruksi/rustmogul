use uuid::Uuid;

use crate::ledger::database::pool::Pool;
use crate::ledger::database::robot::Robot;

// how to model:
//  * shop-specific effects applied to robots on buy?
//    (buffs, debuffs, etc.)
//  * shop-specific effects applied to robots before buy?
//    (cost reduction, etc.)
// maybe a "pipe" of effects that can be reversed?

#[derive(Debug, Clone)]
pub struct Shop {
    pub slots: Vec<Option<Robot>>,
}

impl Default for Shop {
    fn default() -> Self {
        let slots = vec![None, None, None, None, None];
        Self { slots }
    }
}

impl Shop {
    #[rustfmt::skip]
    pub fn add_robots(&mut self, mut robots: Vec<Robot>) {
        if robots.len() > self.slots.iter().filter(|s| s.is_none()).collect::<Vec<_>>().len() {
            panic!("Shop Error: Cannot add more robots than there are slots in the shop");
            // TODO: maybe this could be fixed automatically... just take as much as you can?
        }
        for i in 0..self.slots.len() {
            if robots.is_empty() { break; }
            if self.slots[i].is_some() { continue; };
            self.slots[i] = Some(robots.remove(0));
        }
    }

    pub fn get_robot(&self, robot_id: Uuid) -> Option<&Robot> {
        for i in 0..self.slots.len() {
            let found_robot = &self.slots[i].as_ref();
            if found_robot.is_none() {
                continue;
            };
            if found_robot.as_ref().unwrap().id == robot_id {
                return *found_robot;
            }
        }
        None
    }

    pub fn take_robot(&mut self, robot_id: Uuid) -> Option<Robot> {
        let mut robot = None;
        for i in 0..self.slots.len() {
            let Some(found_robot) = &self.slots[i] else {
                continue;
            };
            if found_robot.id == robot_id {
                robot = self.slots[i].take();
                break;
            }
        }
        robot
    }

    pub fn return_to(&mut self, pool: &mut Pool) {
        for i in 0..self.slots.len() {
            if self.slots[i].is_none() {
                continue;
            };
            pool.add_robot(self.slots[i].take().unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn my_pool() -> Pool {
        let mut pool = Pool::default();
        pool.add_robot(Robot::from_prototype_id("barker").unwrap());
        pool.add_robot(Robot::from_prototype_id("barker").unwrap());
        pool.add_robot(Robot::from_prototype_id("magic_missile").unwrap());
        pool.add_robot(Robot::from_prototype_id("ursabot").unwrap());
        pool.add_robot(Robot::from_prototype_id("spellweaver").unwrap());
        pool.add_robot(Robot::from_prototype_id("spellweaver").unwrap());
        pool
    }

    #[test]
    fn ergonomics() {
        let mut pool = my_pool();
        let mut shop = Shop::default();
        assert_robot_counts(&pool, &shop, 6, 0);

        // refresh shop with some robots in a couple of steps
        shop.add_robots(pool.take_random_robots_upto_tier(1, 2));
        assert_robot_counts(&pool, &shop, 4, 2);
        shop.add_robots(pool.take_random_robots_upto_tier(2, 1));
        assert_robot_counts(&pool, &shop, 3, 3);
        shop.return_to(&mut pool);
        assert_robot_counts(&pool, &shop, 6, 0);

        // can keep on refreshing in steps after return
        shop.add_robots(pool.take_random_robots_upto_tier(1, 1));
        assert_robot_counts(&pool, &shop, 5, 1);
        shop.add_robots(pool.take_random_robots_upto_tier(0, 3));
        assert_robot_counts(&pool, &shop, 5, 1);

        let robot_id = shop.slots.iter().next().unwrap().as_ref().unwrap().id;

        // "get" doesn't remove the robot
        assert!(shop.get_robot(Uuid::new_v4()).is_none());
        assert!(shop.get_robot(robot_id).is_some());
        assert!(shop.get_robot(robot_id).is_some());
        assert_robot_counts(&pool, &shop, 5, 1);

        // "take" removes the robot from the system
        assert!(shop.take_robot(robot_id).is_some());
        shop.return_to(&mut pool);
        assert_robot_counts(&pool, &shop, 5, 0);
    }

    fn assert_robot_counts(pool: &Pool, shop: &Shop, pool_count: usize, shop_count: usize) {
        assert_pool_robot_count(pool, pool_count);
        assert_shop_robot_count(shop, shop_count);
        asset_show_slot_count(shop, 5); // should stay constant all the time
    }

    fn assert_pool_robot_count(pool: &Pool, count: usize) {
        assert_eq!(pool.robots.len(), count);
    }

    fn assert_shop_robot_count(shop: &Shop, count: usize) {
        assert_eq!(
            shop.slots
                .iter()
                .filter(|s| s.is_some())
                .collect::<Vec<_>>()
                .len(),
            count
        );
    }

    fn asset_show_slot_count(shop: &Shop, count: usize) {
        assert_eq!(shop.slots.len(), count);
    }
}
