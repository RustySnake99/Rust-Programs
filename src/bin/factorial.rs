use std::io::{self, Write};

fn factorial(n: u64) -> u64 {
    if n <= 2 {
        n
    } else {
        n  * factorial(n - 1)
    }
}
fn main() {
    let mut input = String::new();

    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let number: u64 = input.trim().parse().unwrap();

    println!("The factorial of {} is {}", number, factorial(number));
}