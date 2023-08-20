use figment::providers::{Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub  credentials: Credentials,
}

#[derive(Deserialize)]
pub  struct Credentials {
    pub secrets_location: String,
    pub cache_location: String,
}

pub fn get_config_values() -> Config {
    return Figment::new()
        .merge(Toml::file("configs/configs.toml"))
        .extract()
        .unwrap();
}
