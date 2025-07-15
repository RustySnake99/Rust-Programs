use qrcode::{QrCode, EcLevel};
use image::{Luma};
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("Enter the text or URL to generate a QR code: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let url = input.trim().to_string();

    input.clear();

    print!("Enter the name of the resultant image file (without extension): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let image_name = input.trim().to_string() + ".png";

    match QrCode::with_error_correction_level(url.as_bytes(), EcLevel::M) {
        Ok(qr) => {
            match save_as_image(&qr, &image_name) {
                Ok(_) => println!("✅ QR code generated successfully!"),
                Err(e) => eprintln!("❌ Error saving QR code image: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Error generating QR code: {}", e),
    }
}

fn save_as_image(code: &QrCode, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let image = code.render::<Luma<u8>>().min_dimensions(256, 256).build();
    image.save(name)?;
    Ok(())
}