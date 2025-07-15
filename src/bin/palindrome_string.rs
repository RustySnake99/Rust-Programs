use std::io::{self, Write};

fn main() {
    let mut input = String::new();

    print!("Enter a String (case-sensitive): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let is_palindrome = input.chars().eq(input.chars().rev());

    if is_palindrome {
        println!("The entered string is palindrome.");
    } else {
        println!("The entered string is not palindrome.");
    }
}