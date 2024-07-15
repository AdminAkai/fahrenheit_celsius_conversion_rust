fn main() {
    let converted = fahrenheit_celsius_conversion('F', 32);
    println!("F to C: {converted}");

    let converted = fahrenheit_celsius_conversion('C', 0);
    println!("C to F: {converted}");
}

fn fahrenheit_celsius_conversion(temp: char, v: i32) -> i32 {
    let conversion =  match temp {
        'F' => {
            (v - 32) * 5/9
        },
        'C' => {
            (v * 9/5) + 32
        },
        _ => v
    };

    conversion
}
