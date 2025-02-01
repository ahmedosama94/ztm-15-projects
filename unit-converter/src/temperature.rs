use std::fmt::Display;

use crate::{DimensionUnit, TypedValue};

pub trait Temperature:
    DimensionUnit<f64>
    + From<TemperatureCelsius>
    + From<TemperatureFahrenheit>
    + From<TemperatureKelvin>
{
}
type DefaultTemperatureUnit = TemperatureKelvin;

#[derive(Clone)]
pub struct TemperatureKelvin(pub f64);

impl Display for TemperatureKelvin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TypedValue<f64> for TemperatureKelvin {
    fn value(&self) -> f64 {
        self.0
    }
}

impl DimensionUnit<f64> for TemperatureKelvin {
    type DefaultUnit = DefaultTemperatureUnit;

    fn to_default(&self) -> Self::DefaultUnit {
        self.clone()
    }

    fn from_default(val: Self::DefaultUnit) -> Self {
        val.clone()
    }
}

impl Temperature for TemperatureKelvin {}

#[derive(Clone)]
pub struct TemperatureCelsius(pub f64);

impl Display for TemperatureCelsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TypedValue<f64> for TemperatureCelsius {
    fn value(&self) -> f64 {
        self.0
    }
}

impl DimensionUnit<f64> for TemperatureCelsius {
    type DefaultUnit = DefaultTemperatureUnit;

    fn to_default(&self) -> Self::DefaultUnit {
        TemperatureKelvin(self.value() + 273.15)
    }

    fn from_default(val: Self::DefaultUnit) -> Self {
        Self(val.value() - 273.15)
    }
}

impl Temperature for TemperatureCelsius {}

#[derive(Clone)]
pub struct TemperatureFahrenheit(pub f64);

impl Display for TemperatureFahrenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TypedValue<f64> for TemperatureFahrenheit {
    fn value(&self) -> f64 {
        self.0
    }
}

impl DimensionUnit<f64> for TemperatureFahrenheit {
    type DefaultUnit = DefaultTemperatureUnit;

    fn to_default(&self) -> Self::DefaultUnit {
        TemperatureKelvin((self.value() - 32.0) * (5.0 / 9.0) + 273.15)
    }

    fn from_default(val: Self::DefaultUnit) -> Self {
        Self((val.value() - 273.15) * (9.0 / 5.0) + 32.0)
    }
}

impl Temperature for TemperatureFahrenheit {}

impl From<TemperatureCelsius> for TemperatureKelvin {
    fn from(value: TemperatureCelsius) -> Self {
        Self(Self::from_default(value.to_default()).value())
    }
}

impl From<TemperatureFahrenheit> for TemperatureKelvin {
    fn from(value: TemperatureFahrenheit) -> Self {
        Self(Self::from_default(value.to_default()).value())
    }
}

impl From<TemperatureKelvin> for TemperatureCelsius {
    fn from(value: TemperatureKelvin) -> Self {
        Self(Self::from_default(value.to_default()).value())
    }
}

impl From<TemperatureFahrenheit> for TemperatureCelsius {
    fn from(value: TemperatureFahrenheit) -> Self {
        Self(Self::from_default(value.to_default()).value())
    }
}

impl From<TemperatureKelvin> for TemperatureFahrenheit {
    fn from(value: TemperatureKelvin) -> Self {
        Self(Self::from_default(value.to_default()).value())
    }
}

impl From<TemperatureCelsius> for TemperatureFahrenheit {
    fn from(value: TemperatureCelsius) -> Self {
        Self(Self::from_default(value.to_default()).value())
    }
}
