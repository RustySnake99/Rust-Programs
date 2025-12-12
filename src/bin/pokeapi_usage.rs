use reqwest;
use std::io::{self, Write};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct PokemonType {
    #[serde(rename = "slot")]
    _slot: u8,
    #[serde(rename = "type")]
    pokemon_types: Type,
}
#[derive(Deserialize, Debug)]
struct Type {
    name: String,
    #[serde(rename = "url")]
    _url: String,
}
#[derive(Deserialize, Debug)]
struct Sprite {
    front_default: Option<String>,
    rear_default: Option<String>,
    front_shiny: Option<String>,
}
#[derive(Deserialize, Debug)]
struct Cries {
    latest: String,
    legacy: String,
}
#[derive(Deserialize, Debug)]
struct Description {
    #[serde(rename = "flavor_text")]
    text: String,
    language: Language,
}
#[derive(Deserialize, Debug)]
struct Language {
    name: String,
}
#[derive(Deserialize, Debug)]
struct PokemonSpecies {
    #[serde(rename = "flavor_text_entries")]
    text_entries: Vec<Description>,
}

#[derive(Deserialize, Debug)]
struct PokemonResponse {
    name: String,
    types: Vec<PokemonType>,
    sprites: Sprite,
    cries: Option<Cries>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut input = String::new();
    print!("Enter the name of the Pokemon (Ex: Charizard-mega-y): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let api_input = input.trim().replace(" ", "-").to_lowercase();

    let pokemon_url = format!("https://pokeapi.co/api/v2/pokemon/{}", api_input);
    let species_url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", api_input);

    let response = reqwest::get(&pokemon_url).await?;
    if !response.status().is_success() {
        println!("Oops! No data for the entered pokemon could be found....");
        return Ok(());
    }

    let pokemon: PokemonResponse = response.json().await?;
    let species: PokemonSpecies = reqwest::get(&species_url).await?.json().await?;
    let description = species.text_entries
        .iter()
        .find(|entry| entry.language.name == "en")
        .map(|entry| entry.text.replace("\n", " "))
        .unwrap_or("No description available.".to_string());

    println!("Details about '{}':", pokemon.name);
    println!("- Types:\n\t{}", pokemon.types.iter().map(|t| t.pokemon_types.name.clone().to_uppercase()).collect::<Vec<_>>().join("\t"));
    println!("- Sprites:\n\tFront: {:?}\n\tBack: {:?}\n\tShiny: {:?}",
        pokemon.sprites.front_default.clone().unwrap_or("N/A".to_string()),
        pokemon.sprites.rear_default.clone().unwrap_or("N/A".to_string()),
        pokemon.sprites.front_shiny.clone().unwrap_or("N/A".to_string()));

    if let Some(cries) = pokemon.cries {
        println!("- Cries:\n\tLatest: {:?}\n\tLegacy (OG): {:?}", cries.latest, cries.legacy);
    } else {
        println!("No cries were found....");
    }
    println!("Description of the Pokemon:\n\t'{}'", description);

    Ok(())

}