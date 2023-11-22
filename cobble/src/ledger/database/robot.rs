use bevy::prelude::*;
use uuid::Uuid;

use crate::ledger::database::prototype::{Prototype, PROTOTYPES};
use crate::ledger::database::robotype::Robotype;

// Prototype: Destroyer
// Type: <none>, Beastron, Aquatron, Magitron
// equipment slots...
// pilots... (can usually have one per bot, but also none or more for big bots)

// RECORD _EVERYTHING_ THAT MIGHT CHANGE!

#[derive(Debug, Clone)]
pub struct Robot {
    pub id: Uuid,
    pub prototype_id: String,
    pub robotypes: Vec<Robotype>,
    pub tier: u8,
    pub cost: u16,
}

impl Robot {
    pub fn is_prototype_id(prototype_id: &str) -> bool {
        PROTOTYPES.lock().unwrap().get(prototype_id).is_some()
    }
    pub fn from_prototype_id(prototype_id: &str) -> Result<Robot, String> {
        match PROTOTYPES.lock().unwrap().get(prototype_id) {
            Some(prototype) => Ok(Robot::from_prototype(prototype)),
            None => Err(format!("No prototype found with {:?}", prototype_id)),
        }
    }
    pub fn from_prototype(prototype: &Prototype) -> Robot {
        Robot {
            id: Uuid::new_v4(),
            prototype_id: prototype.id.to_string(),
            robotypes: prototype.robotypes.to_vec(),
            tier: prototype.tier,
            cost: prototype.cost,
        }
    }
    pub fn name(&self) -> String {
        self.prototype_id
            .split('_')
            .map(|part| {
                part.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if i == 0 {
                            c.to_uppercase().next().unwrap()
                        } else {
                            c
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    pub fn color(&self) -> Color {
        match self.prototype_id.as_str() {
            "barker" => Color::rgb(0.2, 0.5, 0.2),
            "magic_missile" => Color::rgb(0.5, 0.2, 0.5),
            "microwave" => Color::rgb(0.5, 0.5, 0.2),
            _ => panic!("Missing color for robot: {:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ergonomics() {
        let barker1 = Robot::from_prototype_id("barker").unwrap();
        let barker2 = Robot::from_prototype(&PROTOTYPES.lock().unwrap().get("barker").unwrap());
        assert_eq!(barker1.prototype_id, barker2.prototype_id);
        assert_ne!(barker1.id, barker2.id);
    }
}
