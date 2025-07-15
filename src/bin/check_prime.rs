use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter a number to check: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input

    io::stdin().read_line(&mut input).unwrap();
    let number: u64 = input.trim().parse().expect("Please enter a positive integer (> 0)!");

    for i in 2..=(number as f64).sqrt() as u64 {
        if number % i == 0 {
            println!("{} is not a prime number!", number);
            return;
        }
    }
    println!("{} is a prime number!", number);
}