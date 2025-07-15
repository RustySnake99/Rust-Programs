use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter the range: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let range: i32 = input.trim().parse().expect("Please enter a valid number....");

    for i in (1..=range).rev() {
        for _ in 1..=i {
            print!("*\t");
        }
        println!();
    }
}