use definitely_not_rink::Temperature;

fn main() {
    let temperature = Temperature::from_fahrenheit(1000.into());

    println!("{:.2}", temperature.to_kelvin());
}
