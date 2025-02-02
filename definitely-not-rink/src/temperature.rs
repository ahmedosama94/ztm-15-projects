use std::ops::Add;

use crate::map_error;
use crate::UnitPrefix;
use crate::MULTIPLIER_MAP;

const DEFAULT_PREFIX: UnitPrefix = UnitPrefix::None;

#[derive(Debug)]
pub struct Temperature {
    value: f64,
}

impl Temperature {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn from_kelvin(val: f64) -> Self {
        Self::from_kelvin_with_prefix(val, None)
    }

    pub fn from_kelvin_with_prefix(val: f64, unit_prefix: Option<UnitPrefix>) -> Self {
        let multiplier = if let Some(unit_prefix) = unit_prefix {
            *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix))
        } else {
            1.0
        };

        let default_multiplier = *MULTIPLIER_MAP.get(&DEFAULT_PREFIX).expect(&map_error(DEFAULT_PREFIX));
        let multiplier = multiplier / default_multiplier;

        let val = (val * multiplier) as f64;

        Temperature::new(val)
    }

    pub fn from_celsius(val: f64) -> Self {
        Self::from_celsius_with_prefix(val, None)
    }

    pub fn from_celsius_with_prefix(val: f64, unit_prefix: Option<UnitPrefix>) -> Self {
        let val_kelvin = val + 273.15;

        Self::from_kelvin_with_prefix(val_kelvin, unit_prefix)
    }

    pub fn from_fahrenheit(val: f64) -> Self {
        Self::from_fahrenheit_with_prefix(val, None)
    }

    pub fn from_fahrenheit_with_prefix(val: f64, unit_prefix: Option<UnitPrefix>) -> Self {
        let val_celsius = (val - 32.0) * 5.0 / 9.0;

        Self::from_celsius_with_prefix(val_celsius, unit_prefix)
    }

    pub fn kelvin(&self) -> f64 {
        self.kelvin_with_prefix(None)
    }

    pub fn kelvin_with_prefix(&self, unit_prefix: Option<UnitPrefix>) -> f64 {
        let multiplier = if let Some(unit_prefix) = unit_prefix {
            *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix))
        } else {
            1.0
        };

        let default_multiplier = *MULTIPLIER_MAP.get(&DEFAULT_PREFIX).expect(&map_error(DEFAULT_PREFIX));
        let multiplier = default_multiplier / multiplier;

        (self.value as f64) * multiplier
    }
}

impl Add for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}
