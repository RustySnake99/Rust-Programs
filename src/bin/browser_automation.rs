use std::io::{self, Write};
use webbrowser;

fn main() {
    print!("Enter the URL to be opened: ");
    io::stdout().flush().unwrap();

    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read input....");
    let url = url.trim();

    if webbrowser::open(url).is_ok() {
        println!("Opened URL: {}", url);
    } else {
        eprintln!("Failed to open URL: {}", url);
    }
}