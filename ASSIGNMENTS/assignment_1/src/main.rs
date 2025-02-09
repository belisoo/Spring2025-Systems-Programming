// Assignment 1: Temperature Converter (09rust-assignments.md)
const FAHRENHEIT_FREEZING_POINT: f64 = 32.0;
fn fahrenheit_to_celsius(f: f64) -> f64 {
    return (f - FAHRENHEIT_FREEZING_POINT) * (5.0 / 9.0);
}

fn celsius_to_fahrenheit(c:f64) -> f64 {
    return (c * 9.0 / 5.0) + FAHRENHEIT_FREEZING_POINT;
}

fn main() {
    let mut temp: f64 = 85.0;
    
    for i in 1..7 {
        println!("{}",fahrenheit_to_celsius(temp));       
        temp = temp + 1.0;   
    }
}