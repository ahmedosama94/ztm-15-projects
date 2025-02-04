use std::collections::HashMap;

use derive_more::Display;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub enum UnitPrefix {
    Femto,
    Pico,
    Nano,
    Micro,
    Milli,
    Centi,
    Deci,
    None,
    Deca,
    Hecto,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
}

fn generate_prefix_multiplier_map() -> HashMap<UnitPrefix, f64> {
    let mut map = HashMap::new();

    map.insert(UnitPrefix::Femto, 1e-15);
    map.insert(UnitPrefix::Pico, 1e-12);
    map.insert(UnitPrefix::Nano, 1e-9);
    map.insert(UnitPrefix::Micro, 1e-6);
    map.insert(UnitPrefix::Milli, 1e-3);
    map.insert(UnitPrefix::Centi, 1e-2);
    map.insert(UnitPrefix::Deci, 1e-1);
    map.insert(UnitPrefix::None, 1e0);
    map.insert(UnitPrefix::Deca, 1e1);
    map.insert(UnitPrefix::Hecto, 1e2);
    map.insert(UnitPrefix::Kilo, 1e3);
    map.insert(UnitPrefix::Mega, 1e6);
    map.insert(UnitPrefix::Giga, 1e9);
    map.insert(UnitPrefix::Tera, 1e12);
    map.insert(UnitPrefix::Peta, 1e15);

    map
}

lazy_static! {
    pub static ref MULTIPLIER_MAP: HashMap<UnitPrefix, f64> = generate_prefix_multiplier_map();
}

pub fn map_error(unit_prefix: UnitPrefix) -> String {
    format!("Missing unit prefix multiplier for {}", unit_prefix)
}

#[cfg(test)]
mod tests {
    use super::map_error;
    use super::UnitPrefix;

    #[test]
    fn test_map_error() {
        assert_eq!(
            map_error(UnitPrefix::Femto),
            String::from("Missing unit prefix multiplier for Femto"),
        );
        assert_eq!(
            map_error(UnitPrefix::Pico),
            String::from("Missing unit prefix multiplier for Pico"),
        );
        assert_eq!(
            map_error(UnitPrefix::Nano),
            String::from("Missing unit prefix multiplier for Nano"),
        );
        assert_eq!(
            map_error(UnitPrefix::Micro),
            String::from("Missing unit prefix multiplier for Micro"),
        );
        assert_eq!(
            map_error(UnitPrefix::Milli),
            String::from("Missing unit prefix multiplier for Milli"),
        );
        assert_eq!(
            map_error(UnitPrefix::Centi),
            String::from("Missing unit prefix multiplier for Centi"),
        );
        assert_eq!(
            map_error(UnitPrefix::Deci),
            String::from("Missing unit prefix multiplier for Deci"),
        );
        assert_eq!(
            map_error(UnitPrefix::None),
            String::from("Missing unit prefix multiplier for None"),
        );
        assert_eq!(
            map_error(UnitPrefix::Deca),
            String::from("Missing unit prefix multiplier for Deca"),
        );
        assert_eq!(
            map_error(UnitPrefix::Hecto),
            String::from("Missing unit prefix multiplier for Hecto"),
        );
        assert_eq!(
            map_error(UnitPrefix::Kilo),
            String::from("Missing unit prefix multiplier for Kilo"),
        );
        assert_eq!(
            map_error(UnitPrefix::Giga),
            String::from("Missing unit prefix multiplier for Giga"),
        );
        assert_eq!(
            map_error(UnitPrefix::Tera),
            String::from("Missing unit prefix multiplier for Tera"),
        );
        assert_eq!(
            map_error(UnitPrefix::Peta),
            String::from("Missing unit prefix multiplier for Peta"),
        );
    }
}
