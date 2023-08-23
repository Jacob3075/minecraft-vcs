use yup_oauth2::AccessToken;

use crate::configuration::Config;
use crate::drive::drive;

/// - check if minecraft-vcs folder exists
///   - create minecraft-vcs folder and update config with folder id
///   - prompt user to update config
/// - check if folder with world name exists in vcs folder
///   - create if not exists
pub async fn handle_init_command(configs: &Config, token: &AccessToken) {
    if drive::check_if_folder_exists(&configs, &token).await {
        println!(
            "folder id for vcs remote folder already set to valid id: {}",
            configs.remote_root_id
        );

        return;
    }

    println!("folder id for vcs remote folder not set/invalid");
    println!("searching for folder in drive with name `minecraft-vcs`");

    let files = drive::find_vcs_folder_in_remote(&token).await.unwrap();

    if files.is_empty() {
        return folder_not_present(token).await;
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

async fn folder_not_present(token: &AccessToken) {
    println!("no vcs folder found, creating one");
    let created_folder = drive::create_folder_in_drive(&token).await;

    println!(
        "created folder with name: {}, id: {}",
        created_folder.name, created_folder.id
    );

    println!("update config.toml with folder id");
    return;
}
