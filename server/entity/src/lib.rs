use serde::Deserialize;

pub mod prelude;

pub mod pokemon;

#[derive(Debug, Deserialize)]
pub struct PokemonWebhook {
    pub encounter_id: String,
    pub pokemon_id: u32,
    pub form: u32,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct Webhook {
    pub message: PokemonWebhook,
}
