use crate::UnitPrefix;

#[derive(Debug)]
pub struct Time {
    value: f64,
}

impl Time {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn from_seconds(value: f64) -> Self {
        Self::new(value)
    }

    pub fn from_seconds_with_prefix(value: f64, prefix: UnitPrefix) -> Self {
        todo!()
    }
}
