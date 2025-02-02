use std::ops::Add;

use crate::core::Op;
use crate::map_error;
use crate::UnitPrefix;
use crate::OPERATION_MAP;

const DEFAULT_PREFIX: UnitPrefix = UnitPrefix::Femto;

#[derive(Debug)]
pub struct Temperature {
    value: i128,
}

impl Add for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl Temperature {
    pub fn new(value: i128) -> Self {
        Self { value }
    }

    pub fn from_kelvin(val: i128, unit_prefix: Option<UnitPrefix>) -> Self {
        let operation = if let Some(unit_prefix) = unit_prefix {
            OPERATION_MAP.get(&unit_prefix).expect(&map_error(unit_prefix))
        } else {
            &Op::Mult(1)
        };
        let default_operation = OPERATION_MAP.get(&DEFAULT_PREFIX).expect(&map_error(DEFAULT_PREFIX));

        let val = match operation {
            Op::Div(d1) => {
                match default_operation {
                    Op::Div(d2) => val * d2 / d1,
                    Op::Mult(m2) => val / d1 / m2,
                }
            },
            Op::Mult(m1) => {
                match default_operation {
                    Op::Div(d2) => val / m1 * d2,
                    Op::Mult(m2) => val / m1 / m2,
                }
            },
        };

        Temperature::new(val)
    }

    pub fn from_celsius(val: i128, unit_prefix: Option<UnitPrefix>) -> Self {
        let offset = Self::from_kelvin(273150, Some(UnitPrefix::Kilo));

        Self::from_kelvin(val, unit_prefix) + offset
    }

    pub fn from_fahrenheit(val: i128, unit_prefix: Option<UnitPrefix>) -> Self {
        let offset = Self::from_kelvin(32, None);
        let temp = Self::from_kelvin(val, unit_prefix);

        let val = (temp.value - offset.value) * 5 / 9;

        Self::from_celsius(val, Some(DEFAULT_PREFIX))
    }

    pub fn kelvin(&self, unit_prefix: Option<UnitPrefix>) -> f64 {
        let default_operation = OPERATION_MAP.get(&DEFAULT_PREFIX).expect(&map_error(DEFAULT_PREFIX));
        let operation = if let Some(unit_prefix) = unit_prefix {
            OPERATION_MAP.get(&unit_prefix).expect(&map_error(unit_prefix))
        } else {
            &Op::Mult(1)
        };

        match operation {
            Op::Div(d1) => {
                let d1 = *d1 as f64;

                match default_operation {
                    Op::Div(d2) => self.value_f64() * d1 / (*d2 as f64),
                    Op::Mult(m2) => self.value_f64() * d1 * (*m2 as f64),
                }
            },
            Op::Mult(m1) => {
                let m1 = *m1 as f64;
                match default_operation {
                    Op::Div(d2) => self.value_f64() / (*d2 as f64) / m1,
                    Op::Mult(m2) => self.value_f64() * (*m2 as f64) / m1,
                }
            }
        }
    }

    pub fn value_f64(&self) -> f64 {
        self.value as f64
    }
}
