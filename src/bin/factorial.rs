use std::io::{self, Write};

fn facrotial(x: u32) -> u64 {
  if x <= 2 {
    x
  } else {
    x * factorial(x - 1)
  }
}

fn main() {
  let mut input = String::new();
  print!("Enter the number: ");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).unwrap();

  let n: u32 = input.trim().parse().unwrap();
  println!("Factorial of {} = {}", n, factorial(n));
}
