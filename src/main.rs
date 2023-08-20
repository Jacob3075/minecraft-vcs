use crate::configuration::get_config_values;
use authentication::get_token;

mod authentication;
mod configuration;

#[tokio::main]
async fn main() {
    let config = get_config_values();
    let token = get_token(config.credentials)
        .await
        .expect("could not get token");
    dbg!(token);
}
