use std::ops::Add;

use crate::map_error;
use crate::UnitPrefix;
use crate::MULTIPLIER_MAP;

#[derive(Debug)]
pub struct Temperature {
    value: f64,
}

impl Temperature {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn from_kelvin(val: f64) -> Self {
        Self::new(val)
    }

    pub fn from_kelvin_with_prefix(val: f64, unit_prefix: UnitPrefix) -> Self {
        let multiplier = *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix));

        Self::from_kelvin(val * multiplier)
    }

    pub fn from_celsius(val: f64) -> Self {
        Self::from_kelvin(val + 273.15)
    }

    pub fn from_celsius_with_prefix(val: f64, unit_prefix: UnitPrefix) -> Self {
        let multiplier = *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix));

        Self::from_celsius(val * multiplier)
    }

    pub fn from_fahrenheit(val: f64) -> Self {
        Self::from_celsius((val - 32.0) * 5.0 / 9.0)
    }

    pub fn from_fahrenheit_with_prefix(val: f64, unit_prefix: UnitPrefix) -> Self {
        let multiplier = *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix));

        Self::from_fahrenheit(val * multiplier)
    }

    pub fn to_kelvin(&self) -> f64 {
        self.value
    }

    pub fn to_kelvin_with_prefix(&self, unit_prefix: UnitPrefix) -> f64 {
        let multiplier = *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix));

        self.to_kelvin() / multiplier
    }

    pub fn to_celsius(&self) -> f64 {
        self.value - 273.15
    }

    pub fn to_celsius_with_prefix(&self, unit_prefix: UnitPrefix) -> f64 {
        let multiplier = *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix));

        self.to_celsius() / multiplier
    }

    pub fn to_fahrenheit(&self) -> f64 {
        ((self.value - 273.15) * 9.0 / 5.0) + 32.0
    }

    pub fn to_fahrenheit_with_prefix(&self, unit_prefix: UnitPrefix) -> f64 {
        let multiplier = *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix));

        self.to_fahrenheit() / multiplier
    }
}

impl Add for Temperature {
    type Output = Temperature;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}
