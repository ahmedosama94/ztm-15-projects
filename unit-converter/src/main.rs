use unit_converter::{TemperatureFahrenheit, TemperatureKelvin};

fn main() {
    let temp_k = TemperatureKelvin(100.0);
    let temp_f = TemperatureFahrenheit::from(temp_k);

    println!("{}", temp_f);
}
