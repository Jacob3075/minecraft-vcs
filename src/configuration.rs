use figment::providers::{Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub credentials: Credentials,
    pub world_name: String,
    pub local_location: String,
    /// The ID of the folder on Google Drive that contains the world.
    pub remote_root_id: String,
}

#[derive(Deserialize)]
pub struct Credentials {
    pub secrets_location: String,
    pub cache_location: String,
}

pub fn get_config_values() -> Config {
    return Figment::new()
        .merge(Toml::file("configs/configs.toml"))
        .extract()
        .unwrap();
}
