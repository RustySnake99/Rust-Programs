use std::io::{self, Write};

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    }
    else if n <= 2 {
        return n;
    } else {
        return n * factorial(n - 1);
    }
}

fn nCr(n: u32, r: u32) -> u32 {
    factorial(n) / (factorial(r) * factorial(n - r))
}

fn main() {
    let mut input = String::new();
    print!("Enter the number of rows to be displayed: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let r: u32 = input.trim().parse().expect("Please enter a valid positive integer....");

    for i in 0..r {
        for j in 0..=i {
            print!("{}\t", nCr(i, j));
        }
        println!();
    }
}