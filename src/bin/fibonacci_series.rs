use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the number of terms to be displayed: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let range: u64 = input.trim().parse().expect("Please enter a positive integer...");

    let (mut a, mut b) = (0, 1);
    println!("Fibonacci series up to {}:", range);

    for _ in 0..range {
        print!("{}\t", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    println!();
}