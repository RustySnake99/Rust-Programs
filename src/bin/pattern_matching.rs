use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let x: i64 = input.trim().parse().expect("Please enter a valid integer value!");
    match x {
        1..=10 => println!("Number is between 1 and 10!"),
        11..=50 => println!("Number is between 11 and 50!"),
        51..=100 => println!("Number is between 51 and 100"),
        _ => println!("Number is greater than 100!!"),
    }
}
