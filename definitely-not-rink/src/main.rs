use definitely_not_rink::{Temperature, UnitPrefix};

fn main() {
    let temperature = Temperature::from_kelvin(400, Some(UnitPrefix::Pico));

    println!("{:#?}", temperature);
    println!("{:.2} K", temperature.kelvin(None));

    let temperature = Temperature::from_celsius(400, Some(UnitPrefix::Milli));

    println!("{:#?}", temperature);
    println!("{:.2} K", temperature.kelvin(None));

    let temperature = Temperature::from_fahrenheit(1000, None);

    println!("{:#?}", temperature);
    println!("{:.2} K", temperature.kelvin(None));
}
