use std::sync::Mutex;

use hashbrown::HashMap;
use once_cell::sync::Lazy;

use crate::ledger::database::robotype::Robotype;

// Robots have a prototype that defines their _original_ stats and abilities
// before player-specific or global modifications.

#[derive(Debug)]
pub struct Prototype {
    pub id: &'static str,
    pub robotypes: Vec<Robotype>,
    pub tier: u8,
    pub cost: u16,
    pub health: u16,
}

#[derive(Debug)]
pub struct PrototypeDatabase {
    pub prototypes: HashMap<&'static str, Prototype>,
}

impl PrototypeDatabase {
    pub fn get(&self, id: &str) -> Option<&Prototype> {
        self.prototypes.get(id)
    }
    pub fn all(&self) -> Vec<&Prototype> {
        self.prototypes.values().collect()
    }
}

//noinspection SpellCheckingInspection
#[rustfmt::skip]
pub static PROTOTYPES: Lazy<Mutex<PrototypeDatabase>> = Lazy::new(|| {
    let as_vec = vec!(
        Prototype { tier: 1, cost: 1, health: 400, id: "barker", robotypes: vec!(Robotype::Military) },
        Prototype { tier: 1, cost: 1, health: 400, id: "microwave", robotypes: vec!(Robotype::Appliance) },
        Prototype { tier: 1, cost: 1, health: 400, id: "magic_missile", robotypes: vec!(Robotype::Arcane) },
        Prototype { tier: 2, cost: 2, health: 500, id: "ursabot", robotypes: vec!(Robotype::Military) },
        Prototype { tier: 2, cost: 2, health: 500, id: "spellweaver", robotypes: vec!(Robotype::Arcane) },
        Prototype { tier: 2, cost: 2, health: 500, id: "macrowave", robotypes: vec!(Robotype::Appliance) },
        Prototype { tier: 3, cost: 3, health: 600, id: "vaporwave", robotypes: vec!(Robotype::Appliance) },
    );
    let prototypes = as_vec.into_iter().map(|p| (p.id, p)).collect();
    let database = PrototypeDatabase { prototypes };
    Mutex::new(database)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    #[test]
    fn ergonomics() {
        // accessing individual prototypes
        assert_eq!(PROTOTYPES.lock().unwrap().get("barker").unwrap().id, "barker");

        // accessing all prototypes
        assert!(PROTOTYPES.lock().unwrap().all().iter().count() > 3);

        // accessing by some filtering
        // intentionally iterating twice to test that the casting to right type works
        assert!(
            PROTOTYPES.lock().unwrap().all().into_iter()
                .filter(|p| p.robotypes.contains(&Robotype::Arcane))
                .collect::<Vec<&Prototype>>()
                .iter().count() > 1
        )
    }
}
