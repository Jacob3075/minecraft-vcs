use figment::Figment;
use figment::providers::{Format, Toml};
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

pub fn get_config_values() -> figment::error::Result<Config> {
    Figment::new()
        .merge(Toml::file("configs/configs.toml"))
        .extract()
}
