use std::fmt::Display;

#[derive(Clone)]
pub enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl Temperature {
    pub fn to_celsius(&self) -> Self {
        match self {
            Self::Celsius(_) => self.clone(),
            Self::Fahrenheit(val) => Self::Celsius((val - 32.0) * 5.0 / 9.0),
            Self::Kelvin(val) => Self::Celsius(val + 273.15),
        }
    }

    pub fn to_fahrenheit(&self) -> Self {
        match self {
            Self::Celsius(val) => Self::Fahrenheit((val * 9.0 / 5.0) + 32.0),
            Self::Fahrenheit(_) => self.clone(),
            Self::Kelvin(val) => Self::Fahrenheit(((val - 273.15) * 9.0 / 5.0) + 32.0),
        }
    }

    pub fn to_kelvin(&self) -> Self {
        match self {
            Self::Celsius(val) => Self::Kelvin(val + 273.15),
            Self::Fahrenheit(val) => Self::Kelvin(((val + 273.15) - 32.0) * 5.0 / 9.0),
            Self::Kelvin(_) => self.clone(),
        }
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (val, unit) = match self {
            Self::Celsius(val) => (val, "° C"),
            Self::Fahrenheit(val) => (val, "° F"),
            Self::Kelvin(val) => (val, " K"),
        };

        write!(f, "{:.2}{}", *val, unit)
    }
}
