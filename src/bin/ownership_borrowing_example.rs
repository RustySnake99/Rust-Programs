fn print_length(s: &String) {
    println!("Length of string: {}", s.len());
}

fn main() {
    let s = String::from("Rustacean");
    print_length(&s); // borrow
    println!("Original string: {}", s); // still usable (not moved)
}
