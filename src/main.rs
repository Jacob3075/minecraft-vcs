extern crate core;

use clap::Parser;
use yup_oauth2::AccessToken;

use Commands::*;

use crate::arguments::{Arguments, Commands};
use crate::authentication::get_token;
use crate::commands::init::handle_init_command;
use crate::configuration::{Config, get_config_values};

mod arguments;
mod authentication;
mod commands;
mod configuration;
mod drive;
mod errors;

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    let config = get_config_values().expect("failed to get config values");
    let token = get_token(&config.credentials)
        .await
        .expect("failed to login to google, try deleting token cache and logging in again");

    let result = handle_command(args, config, token).await;
}

async fn handle_command(args: Arguments, config: Config, token: AccessToken) {
    match args.command {
        INIT => handle_init_command(&config, &token).await,
        PULL => {
            todo!();
        }
        PUSH => {
            todo!();
        }
        LOGS => {
            todo!();
        }
    };
}
