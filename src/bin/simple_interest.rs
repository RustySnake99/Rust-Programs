use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the Principal amount: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let p: f64 = input.trim().parse().expect("Please enter a valid positive number!");

    input.clear();

    print!("Enter the rate of interest (in percentage): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let r: f64 = input.trim().parse().expect("Please enter a valid positive number!");

    input.clear();
    print!("Enter the time period (in years): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let t: f64 = input.trim().parse().expect("Please enter a valid positive number!");

    let interest = (p * r * t) / 100.0;
    println!("The resultant Simple Interest is = {:.2}", interest);
}