use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let x: f64 = input.trim().parse().unwrap();

    println!("Default scientific form: {:e}", x);
}