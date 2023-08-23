use clap::Parser;
use yup_oauth2::AccessToken;

use errors::AppErrors;
use Commands::*;

use crate::arguments::{Arguments, Commands};
use crate::authentication::get_token;
use crate::configuration::{get_config_values, Config};

mod arguments;
mod authentication;
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

    // let result = handle_command(args, config, token).await;

    println!("{:?}", config);
}

async fn handle_command(
    args: Arguments,
    config: Config,
    token: AccessToken,
) -> Result<String, AppErrors> {
    return match args.command {
        // create sqlite table for key-value pairs
        // check if minecraft-vcs folder exists
        //    create minecraft-vcs folder and update config with folder id
        // check if folder with world name exists in vcs folder
        //   create if not exists
        INIT => {
            println!("");
            return Ok("".to_string());
        }
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
