use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the number for which mulitplication table is to be generated: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().expect("Please enter a valid integer...");

    input.clear();
    print!("Enter the range for the multiplication table: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let range: i64 = input.trim().parse().expect("Please enter a valid integer...");

    println!("\nMultiplication table for {} is as follows:", n);
    for i in 1..=range {
        println!("{} x {} = {}", n, i, n * i);
    }
}