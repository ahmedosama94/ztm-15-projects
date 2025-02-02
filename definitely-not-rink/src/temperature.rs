use crate::map_error;
use crate::UnitPrefix;
use crate::MULTIPLIER_MAP;

const DEFAULT_PREFIX: UnitPrefix = UnitPrefix::Pico;

#[derive(Debug)]
pub struct Temperature {
    value: u64,
}

impl Temperature {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn from_kelvin(val: f64, unit_prefix: Option<UnitPrefix>) -> Self {
        let multiplier = if let Some(unit_prefix) = unit_prefix {
            *MULTIPLIER_MAP.get(&unit_prefix).expect(&map_error(unit_prefix))
        } else {
            1.0
        };

        let default_multiplier = *MULTIPLIER_MAP.get(&DEFAULT_PREFIX).expect(&map_error(DEFAULT_PREFIX));
        let multiplier = multiplier / default_multiplier;

        let val = (val * multiplier) as u64;

        Temperature::new(val)
    }

    pub fn from_celsius(val: f64, unit_prefix: Option<UnitPrefix>) -> Self {
        let val_kelvin = val + 273.15;

        Self::from_kelvin(val_kelvin, unit_prefix)
    }

    pub fn from_fahrenheit(val: f64, unit_prefix: Option<UnitPrefix>) -> Self {
        let val_celsius = (val - 32.0) * 5.0 / 9.0;

        Self::from_celsius(val_celsius, unit_prefix)
    }

    pub fn kelvin(&self, unit_prefix: Option<UnitPrefix>) -> f64 {
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
