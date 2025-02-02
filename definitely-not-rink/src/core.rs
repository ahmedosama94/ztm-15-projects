use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
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

pub enum Op {
    Mult(i128),
    Div(i128),
}

fn generate_prefix_multiplier_map() -> HashMap<UnitPrefix, Op> {
    let mut map = HashMap::new();

    map.insert(UnitPrefix::Femto, Op::Div(1_000_000_000_000_000));
    map.insert(UnitPrefix::Pico, Op::Div(1_000_000_000_000));
    map.insert(UnitPrefix::Nano, Op::Div(1_000_000_000));
    map.insert(UnitPrefix::Micro, Op::Div(1_000_000));
    map.insert(UnitPrefix::Milli, Op::Div(1000));
    map.insert(UnitPrefix::Centi, Op::Div(100));
    map.insert(UnitPrefix::Deci, Op::Div(10));
    map.insert(UnitPrefix::None, Op::Mult(1));
    map.insert(UnitPrefix::Deca, Op::Mult(10));
    map.insert(UnitPrefix::Hecto, Op::Mult(100));
    map.insert(UnitPrefix::Kilo, Op::Mult(1000));
    map.insert(UnitPrefix::Mega, Op::Mult(1_000_000));
    map.insert(UnitPrefix::Giga, Op::Mult(1_000_000_000));
    map.insert(UnitPrefix::Tera, Op::Mult(1_000_000_000_000));
    map.insert(UnitPrefix::Peta, Op::Mult(1_000_000_000_000_000));

    map
}

lazy_static! {
    pub static ref OPERATION_MAP: HashMap<UnitPrefix, Op> = generate_prefix_multiplier_map();
}

pub fn map_error(unit_prefix: UnitPrefix) -> String {
    format!("Missing unit prefix multiplier for {}", "todo!macro")
}
