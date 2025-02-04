use definitely_not_rink::{Temperature, Time};

fn main() {
    let temperature = Temperature::from_fahrenheit(1000.into());

    println!("{:.2}", temperature.to_kelvin());

    let time = Time::from_seconds(3671.into());

    println!("{}", time);
}
