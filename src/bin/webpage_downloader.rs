use std::io::{self, Write};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut url = String::new();
    let mut fname = String::new();
    print!("Enter the URL of the page to be downloaded (Ex: https://youtube.com): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut url).unwrap();

    print!("Enter the name of the file for saving (without any extensions): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut fname).unwrap();
    fname = format!("{}.html", fname.trim());

    let html_content = reqwest::get(&url).await?.text().await?;
    let mut file = File::create(fname).await?;
    file.write_all(html_content.as_bytes()).await?;

    println!("Download Successful!");

    Ok(())
}
