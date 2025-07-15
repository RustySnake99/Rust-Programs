use std::io::{self, Write};

fn hcf(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let mut input = String::new();

    print!("Enter the first integer: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let a: u64 = input.trim().parse().expect("Plese enter a valid positive integer!");

    input.clear();
    print!("Enter the second integer: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let b: u64 = input.trim().parse().expect("Please enter a valid positive integer!");

    println!("The highest common factor (HCF) of {} and {} is: {}", a, b, hcf(a, b));
}