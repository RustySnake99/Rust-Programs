use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the number of rows to be displayed: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let range: u64 = input.trim().parse().expect("Please enter a valid positive integer.....");

    for i in 1..=range {
        for _ in 1..=i {
            print!("*\t");
        }
        println!();
    }
}