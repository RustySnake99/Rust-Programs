use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct LocationResponse {
    status: String,
    message: Option<String>,
    city: Option<String>,
    country: Option<String>,
    lat: Option<f64>,
    lon: Option<f64>,
    #[serde(rename = "query")]
    ip_address: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://ip-api.com/json/"; //Don't use the 'http' URL for ANY OTHER purpose except this code!!
    let response: LocationResponse = reqwest::get(url).await?.json().await?;

    if response.status != "success" {
        eprintln!("API Error: {}", response.message.unwrap_or("Unknown Error".to_string()));
        return Ok(());
    }

    println!("Your IP: {}", response.ip_address.clone().unwrap_or("N/A".to_string()));
    println!("Location: {}, {}", response.city.clone().unwrap_or("Unknown".to_string()), response.country.clone().unwrap_or("Unknown".to_string()));
    println!("Geographic Coordinates: ({}, {})", response.lat.unwrap_or(0.0), response.lon.unwrap_or(0.0));

    Ok(())
}