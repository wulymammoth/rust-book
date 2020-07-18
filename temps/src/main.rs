use std::io;

fn main() {
    println!("Please enter a temperator followed by F (for Fahrenheit) and C (for Celsius)");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    let temperature: f32 = temperature.trim().parse().unwrap();
    println!("Please enter the unit you want to convert this from");
    let mut unit = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");
    let unit = unit.trim(); // shadowed
    println!(
        "Your entry of {}{} is {}",
        temperature,
        unit,
        convert(temperature, &unit)
    );
}

fn convert(temp: f32, unit: &str) -> f32 {
    let unit = unit.to_lowercase();
    if unit == "c" {
        return (temp * 9.0 / 5.0) + 32.0;
    }
    if unit == "f" {
        return (temp - 32.0) * 5.0 / 9.0;
    }
    panic!("invalid unit");
}
