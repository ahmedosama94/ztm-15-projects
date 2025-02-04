use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};

const YEAR: &str = "year";
const DAY: &str = "day";
const HOUR: &str = "hour";
const MINUTE: &str = "minute";
const SECOND: &str = "second";

const STEPS: [(&str, f64); 5] = [
    (YEAR, (60 * 60 * 24 * 365) as f64),
    (DAY, (60 * 60 * 24) as f64),
    (HOUR, (60 * 60) as f64),
    (MINUTE, 60.0),
    (SECOND, 1.0),
];

#[derive(Debug)]
pub struct Time {
    value: f64,
}

impl Time {
    fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn from_seconds(value: f64) -> Self {
        Self::new(value * get_multiplier_for(SECOND))
    }

    pub fn from_minutes(value: f64) -> Self {
        Self::new(value * get_multiplier_for(MINUTE))
    }

    pub fn from_hours(value: f64) -> Self {
        Self::new(value * get_multiplier_for(HOUR))
    }

    pub fn from_days(value: f64) -> Self {
        Self::new(value * get_multiplier_for(DAY))
    }

    pub fn from_years(value: f64) -> Self {
        Self::new(value * get_multiplier_for(YEAR))
    }

    pub fn to_seconds(&self) -> f64 {
        self.value / get_multiplier_for(SECOND)
    }

    pub fn to_minutes(&self) -> f64 {
        self.value / get_multiplier_for(MINUTE)
    }

    pub fn to_hours(&self) -> f64 {
        self.value / get_multiplier_for(HOUR)
    }

    pub fn to_days(&self) -> f64 {
        self.value / get_multiplier_for(DAY)
    }

    pub fn to_years(&self) -> f64 {
        self.value / get_multiplier_for(YEAR)
    }
}

impl Add for Time {
    type Output = Time;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl Sub for Time {
    type Output = Time;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl Mul for Time {
    type Output = Time;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.value)
    }
}

impl Div for Time {
    type Output = Time;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value / rhs.value)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut val = self.value;
        let mut step_idx = 0;

        let mut entries = Vec::new();

        while val > 0.0 && step_idx < STEPS.len() {
            let (step, multiplier) = STEPS[step_idx];

            let step_units = (val / multiplier) as u64;

            if step_units > 0 {
                entries.push(format!("{} {}{}", step_units, step, if step_units == 1 { "" } else { "s" }));
            }

            val -= (step_units as f64) * multiplier;
            step_idx += 1;
        }

        if entries.len() == 1 {
            write!(f, "{}", entries[0])
        } else {
            let mut out = entries[..entries.len() - 1].join(", ");
            out.push_str(&format!(" and {}", entries.last().unwrap()));

            write!(f, "{}", out)
        }
    }
}

pub fn get_multiplier_for(key: &str) -> f64 {
    STEPS.iter().find(|e| e.0 == key).expect(&map_step_error(key)).1
}

fn map_step_error(key: &str) -> String {
    format!("No entry for {}", key)
}

#[cfg(test)]
mod tests {
    use super::Time;

    #[test]
    fn test_new() {
        let value = 20.0;
        let time = Time::new(value);

        assert_eq!(time.value, value);
    }

    #[test]
    fn test_from_seconds() {
        let value = 20.0;
        let time = Time::from_seconds(value);

        assert_eq!(time.value, value);
    }

    #[test]
    fn test_from_minutes() {
        let value = 20.0;
        let time = Time::from_minutes(value);

        assert_eq!(time.value, 1200.0);
    }

    #[test]
    fn test_from_hours() {
        let value = 2.0;
        let time = Time::from_hours(value);

        assert_eq!(time.value, 7200.0);
    }

    #[test]
    fn test_from_days() {
        let value = 3.0;
        let time = Time::from_days(value);

        assert_eq!(time.value, 259200.0);
    }

    #[test]
    fn test_from_years() {
        let value = 5.0;
        let time = Time::from_years(value);

        assert_eq!(time.value, 157680000.0);
    }

    #[test]
    fn test_display() {
        let value = 3671.0;
        let time = Time::from_seconds(value);
        let display = format!("{}", time);

        assert_eq!(display, String::from("1 hour, 1 minute and 11 seconds"));
    }

    #[test]
    fn test_simple_add() {
        let v1 = 3.0;
        let t1 = Time::from_seconds(v1);

        let v2 = 5.0;
        let t2 = Time::from_seconds(v2);

        assert_eq!((t1 + t2).value, v1 + v2);
    }

    #[test]
    fn test_add_with_multiples() {
        let v1 = 3.0;
        let t1 = Time::from_hours(v1);

        let v2 = 5.0;
        let t2 = Time::from_days(v2);

        assert_eq!((t1 + t2).value, v1 * 3600.0 + v2 * 3600.0 * 24.0);
    }

    #[test]
    fn test_to_seconds() {
        let value = 3.0;
        let time = Time::from_hours(value);

        assert_eq!(time.to_seconds(), 10800.0);
    }

    #[test]
    fn test_to_minutes() {
        let value = 3.0;
        let time = Time::from_hours(value);

        assert_eq!(time.to_minutes(), 180.0);
    }

    #[test]
    fn test_to_hours() {
        let value = 30.0;
        let time = Time::from_minutes(value);

        assert_eq!(time.to_hours(), 0.5);
    }

    #[test]
    fn test_to_days() {
        let value = 60.0;
        let time = Time::from_hours(value);

        assert_eq!(time.to_days(), 2.5);
    }

    #[test]
    fn test_to_years() {
        let value = 438.0;
        let time = Time::from_days(value);

        assert_eq!(time.to_years(), 1.2);
    }
}
