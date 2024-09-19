// Declare constant for freezing point of water
const FREEZING_POINT_CELSIUS: u8 = 32;

// Implement two functions
    // f is a variable
    // have to use 32.0, 5.0, etc because f is a float
fn fahrenheit_to_celsius(f: f64) -> f64 {
    let c = (f - 32.0) * 5.0 / 9.0;
    c
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let f = ((9.0/5.0) * c) + 32.0;
    f
}

fn main() {
    let mut temp_fahrenheit:f64 = 48.0;
    let mut celsius = fahrenheit_to_celsius(temp_fahrenheit);
    println!("{} celsius", celsius);

    for _ in 0..5 {
        temp_fahrenheit += 1.0;
        celsius = fahrenheit_to_celsius(temp_fahrenheit);
        println!("{} celsius", celsius);
    }
}


