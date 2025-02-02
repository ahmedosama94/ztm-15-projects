use definitely_not_rink::Temperature;
use definitely_not_rink_macro::FromWithPrefix;

fn main() {
    let temperature = Temperature::from_fahrenheit(1000.into());
    temperature.test();

    println!("{:.2}", temperature.to_kelvin());
}
