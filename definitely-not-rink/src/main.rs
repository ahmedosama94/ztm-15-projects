use definitely_not_rink::Temperature;

fn main() {
    let temperature = Temperature::from_kelvin(400.0, None);

    println!("{:#?}", temperature.kelvin(None));
}
