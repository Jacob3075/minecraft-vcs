extern crate core;

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::process::id;
use tokio::select;
use yup_oauth2::AccessToken;

use errors::AppErrors;
use Commands::*;

use crate::arguments::{Arguments, Commands};
use crate::authentication::get_token;
use crate::configuration::{get_config_values, Config};
use crate::drive::{create_folder_in_drive, find_vcs_folder_in_remote};

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

/// - check if minecraft-vcs folder exists
///   - create minecraft-vcs folder and update config with folder id
///   - prompt user to update config
/// - check if folder with world name exists in vcs folder
///   - create if not exists
async fn handle_init_command(configs: &Config, token: &AccessToken) {
    // TODO: CHECK IF FOLDER ACTUALLY EXISTS
    if configs.remote_root_id.is_empty() {
        println!("folder id for vcs remote folder not set");
    } else {
        println!(
            "folder id for vcs remote folder already set to {}",
            configs.remote_root_id
        );

        return;
    }

    println!("searching for folder in drive with name `minecraft-vcs`");

    let files = find_vcs_folder_in_remote(&token).await.unwrap();

    if files.is_empty() {
        println!("no vcs folder found, creating one");
        let created_folder = create_folder_in_drive(&token).await;

        println!(
            "created folder with name: {}, id: {}",
            created_folder.name, created_folder.id
        );

        println!("update config.toml with folder id");
        return;
    };

    if files.len() > 1 {
        println!();
        println!(
            "** more than one vcs folder found, please update config.toml with appropriate one **"
        );
        println!();
    }

    println!("update config.toml with folder id");

    files
        .iter()
        .for_each(|file| println!("folder name: {}, id: {}", file.name, file.id));
}
