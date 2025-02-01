use not_rink::Temperature;

fn main() {
    let temperature = Temperature::Kelvin(100.0);
    let temperature = temperature.to_fahrenheit();

    println!("{}", temperature);
}
