use std::io::{self, Write};
use std::fs;
use quircs::Quirc;
use image::ImageReader;

fn main() {
    let mut input = String::new();
    let entries = fs::read_dir(".").expect("Failed to read the current directory");

    println!("📂 Scanning current directory for image files...");

    let image_files: Vec<_> = entries.filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();
        let file_extension = path.extension()?.to_str()?.to_lowercase();
        if ["png", "jpg", "jpeg", "bmp", "gif"].contains(&file_extension.as_str()) {
            Some(path)
        } else {
            None
        }
    }).collect();

    if image_files.is_empty() {
        println!("❌ No image files found in the project directory.");
        return;
    }

    println!("📸 The following image files were found:");
    for (i, path) in image_files.iter().enumerate() {
        if let Some(file_name) = path.file_name() {
            println!("  {}) {}", i + 1, file_name.to_string_lossy());
        }
    }

    print!("\n➡️ Enter the number corresponding to your choice: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().parse::<usize>().unwrap_or(0);

    if choice == 0 || choice > image_files.len() {
        println!("⚠️ Invalid choice! Please select a valid number.");
        return;
    }

    let selected_path = &image_files[choice - 1];
    println!("\n🖼️ Selected image: {}\n", selected_path.to_string_lossy());

    // Load and convert image to grayscale
    let img = ImageReader::open(selected_path)
        .expect("Failed to open image!")
        .decode()
        .expect("Failed to decode image!");
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();

    // Decode QR codes
    let mut decoder = Quirc::default();
    let codes: Vec<_> = decoder.identify(w as usize, h as usize, &gray).collect();

    if codes.is_empty() {
        println!("❌ No QR codes found in the image.");
        return;
    }

    // Decode each code
    for (i, result) in codes.iter().enumerate() {
        match result {
            Ok(code) => {
                match code.decode() {
                    Ok(decoded) => {
                        println!("✅ QR Code {}: {}", i + 1, String::from_utf8_lossy(&decoded.payload));
                    }
                    Err(e) => {
                        println!("⚠️ Failed to decode QR Code {}: {}", i + 1, e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to extract QR Code {}: {}", i + 1, e);
            }
        }
    }
}
