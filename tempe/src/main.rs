// Convert temperatures between Fahrenheit and Celsius.

use std::io;

fn main() {
    let x: f32 = input_f32("celsius.".to_string());
    println!("Fahrenheit: {}", c2f(x));

    let x: f32 = input_f32("Fahrenheit.".to_string());
    println!("celsius: {}", f2c(x));
}

fn input_f32 (prompt: String) -> f32 {
    println!("Please input {}.", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: f32 = input.trim().parse()
        .expect("Can not convert");
    input
}


fn c2f (celsius: f32) -> f32{
    let fahrenheit = 1.8*celsius + 32.0;
    fahrenheit
}

fn f2c (fahrenheit: f32) -> f32{
    (fahrenheit - 32.0)/1.8
}

