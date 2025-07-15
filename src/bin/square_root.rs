use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let number: f64 = input.trim().parse().expect("Please enter a valid number....");

    if number < 0.0 {
        println!("The square root of negative numbers are imaginary...");
    } else {
        let square_root = number.sqrt();
        println!("The square root of {} is {}", number, square_root);
    }
}