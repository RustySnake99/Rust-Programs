use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    let vowels = ['a','e','i','o','u'];

    print!("Enter a String: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let count = input.to_lowercase().chars().filter(|c| vowels.contains(c)).count();

    println!("Number of vowels found: {}", count);
}