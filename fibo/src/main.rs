// Generate the nth Fibonacci number.
use std::io;

fn main() {
    let n: u32 = input_u32("n is?".to_string());
    println!("{}th fibonaci is {}", n, fib(n));
}

fn input_u32 (prompt: String) -> u32 {
    println!("Please input {}.", prompt);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse()
        .expect("Can not convert");
    input
}

fn fib (n: u32) -> u32 {
    let x: u32;
    if n <= 2 {
        x = 1;
    } else {
        x = fib(n - 1) + fib(n - 2);
    }
    x
}
