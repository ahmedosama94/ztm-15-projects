use std::collections::HashMap;

use definitely_not_rink_macro_derive::SimpleEnumToString;
use lazy_static::lazy_static;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, SimpleEnumToString)]
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
    format!("Missing unit prefix multiplier for {}", "todo!macro")
}
