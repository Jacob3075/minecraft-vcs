use figment::providers::{Format, Toml};
use figment::Figment;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub authentication: Authentication,
    pub rustc: Option<String>,
    pub rustdoc: Option<String>,
}

#[derive(Deserialize)]
pub struct Authentication {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub revocation_uri: String,
}

pub fn get_config_values() -> Config {
    return Figment::new()
        .merge(Toml::file("configs/configs.toml"))
        .extract()
        .unwrap();
}
