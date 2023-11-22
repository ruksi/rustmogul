#[allow(unused_imports)]
use std::str::FromStr;

use strum_macros::{Display, EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

// NB: I tried implementing From<>, but it doesn't allow Result<> (it always panics on failure)
// * maybe add partial eq against &str and &String?

#[derive(
    Debug, Clone, PartialEq, Display, EnumIter, EnumString, EnumVariantNames, IntoStaticStr,
)]
pub enum Robotype {
    Military,
    Arcane,
    Appliance,
}

impl Robotype {
    #[allow(dead_code)]
    fn to_str(&self) -> &'static str {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use strum::VariantNames;

    use super::*;

    #[test]
    fn ergonomics() {
        // strings work ok~
        let some_text: String = "Military".into();
        let some_robotype = Robotype::from_str(&some_text).unwrap();
        if some_robotype != Robotype::Military {
            assert!(false);
            return;
        }
        assert_eq!(Robotype::Military.to_string(), some_text);

        // string literals work
        let some_text: &str = "Arcane";
        let Some(some_robotype) = Robotype::from_str(some_text).ok() else {
            assert!(false);
            return;
        };
        assert_eq!(Robotype::Arcane, some_robotype);
        assert_eq!(Robotype::Arcane.to_str(), some_text);

        // bad types
        assert!(Robotype::from_str("I'm bad").is_err());

        // accessing all variants
        assert_eq!(Robotype::VARIANTS, ["Military", "Arcane", "Appliance"]);
    }
}
