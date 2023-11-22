use std::iter;

use rand::prelude::*;
use uuid::Uuid;

use crate::ledger::database::prototype::PROTOTYPES;
use crate::ledger::database::robot::Robot;

// Bots are taken from the pool and returned to the pool as-is.
// + On pool exit, they can get buffed/nerfed
// + On pool re-entry, they _usually_ get reset to their original state
#[derive(Debug, Default, Clone)]
pub struct Pool {
    pub robots: Vec<Robot>,
}

impl Pool {
    pub fn initialize(&mut self) {
        for prototype in PROTOTYPES.lock().unwrap().all().iter() {
            let count = match prototype.tier {
                1 => 30,
                2 => 20,
                3 => 20,
                4 => 10,
                5 => 10,
                _ => 0,
            };
            for robot in iter::repeat_with(|| Robot::from_prototype(prototype)).take(count) {
                self.add_robot(robot);
            }
        }
    }

    pub fn add_robot(&mut self, robot: Robot) {
        self.robots.push(robot);
    }

    pub fn add_robots(&mut self, robots: Vec<Robot>) {
        self.robots.extend(robots);
    }

    pub fn get_robot(&self, robot_id: Uuid) -> Option<&Robot> {
        self.robots.iter().find(|robot| robot.id == robot_id)
    }

    pub fn take_robot(&mut self, robot_id: Uuid) -> Option<Robot> {
        let mut robot = None;
        for i in 0..self.robots.len() {
            if self.robots[i].id == robot_id {
                robot = Some(self.robots.remove(i));
                break;
            }
        }
        robot
    }

    // TODO: change to take_random(sieve: PoolSieve) {}
    pub fn take_random_robots_upto_tier(&mut self, tier: u8, count: usize) -> Vec<Robot> {
        let mut rng = thread_rng();

        let robots = self
            .robots
            .iter()
            .filter(|robot| robot.tier <= tier)
            .choose_multiple(&mut rng, count);

        let robot_ids = robots.iter().map(|robot| robot.id).collect::<Vec<Uuid>>();
        robot_ids
            .iter()
            .map(|robot_id| self.take_robot(*robot_id).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ergonomics() {
        let mut pool = Pool::default();
        pool.initialize();
        assert!(pool.robots.len() >= 60);
    }

    #[test]
    fn get_and_take_robot() {
        let mut pool = Pool::default();
        assert_eq!(pool.robots.len(), 0);

        let robot = Robot::from_prototype_id("barker").unwrap();
        let robot_id = robot.id;
        pool.add_robot(robot);
        assert_eq!(pool.robots.len(), 1);

        let gotten = pool.get_robot(robot_id);
        assert_eq!(gotten.unwrap().id, robot_id);
        assert_eq!(pool.robots.len(), 1);

        let taken = pool.take_robot(robot_id);
        assert_eq!(taken.unwrap().id, robot_id);
        assert_eq!(pool.robots.len(), 0);
        assert!(pool.take_robot(robot_id).is_none());
    }

    #[test]
    fn taking_random_and_returning() {
        let mut pool = Pool::default();
        pool.add_robot(Robot::from_prototype_id("barker").unwrap());
        pool.add_robot(Robot::from_prototype_id("barker").unwrap());
        pool.add_robot(Robot::from_prototype_id("barker").unwrap());
        pool.add_robot(Robot::from_prototype_id("ursabot").unwrap());
        pool.add_robot(Robot::from_prototype_id("ursabot").unwrap());
        pool.add_robot(Robot::from_prototype_id("ursabot").unwrap());
        assert_eq!(pool.robots.len(), 6);

        let two_barkers = pool.take_random_robots_upto_tier(1, 2);
        assert_eq!(two_barkers.len(), 2);
        assert_eq!(pool.take_random_robots_upto_tier(1, 2).len(), 1);
        assert_eq!(pool.take_random_robots_upto_tier(1, 2).len(), 0);

        let one_ursa = pool.take_random_robots_upto_tier(2, 1);
        assert_eq!(one_ursa.len(), 1);
        assert_eq!(pool.take_random_robots_upto_tier(3, 2).len(), 2);
        assert_eq!(pool.take_random_robots_upto_tier(3, 3).len(), 0);

        pool.add_robots(two_barkers);
        pool.add_robots(one_ursa);
        pool.add_robots(vec![]);
        assert_eq!(pool.robots.len(), 3);
    }
}
