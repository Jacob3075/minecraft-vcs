use reqwest::Url;
use yup_oauth2::AccessToken;

use crate::configuration::Config;
use crate::drive::models::{CreateFolderResponse, FileCreationData, FileDetails, FileList};

pub async fn find_vcs_folder_in_remote(token: &AccessToken) -> Result<Vec<FileDetails>, String> {
    let client = reqwest::Client::new();
    let access_token = token.token().expect("cannot get access token").to_string();

    let url = Url::parse_with_params(
        "https://www.googleapis.com/drive/v3/files/",
        &[(
            "q",
            "name = 'minecraft-vcs' and mimeType = 'application/vnd.google-apps.folder' and trashed = false",
        )],
    )
    .expect("could not parse url");

    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("supportsAllDrives", "true")
        .send()
        .await;

    return match response {
        Ok(file_list_json) => Ok(file_list_json
            .json::<FileList>()
            .await
            .expect("could not decode json response from folder search")
            .files),
        Err(error) => Err(format!("could not find minecraft-vcs folder: {}", error)),
    };
}

pub async fn create_folder_in_drive(token: &AccessToken) -> CreateFolderResponse {
    let client = reqwest::Client::new();
    let access_token = token.token().unwrap().to_string();

    let data = FileCreationData {
        mime_type: "application/vnd.google-apps.folder".to_string(),
        name: "minecraft-vcs".to_string(),
        parents: vec![],
    };

    let request_body = serde_json::to_string(&data).expect("failed to serialize json data");

    let response = client
        .post("https://www.googleapis.com/drive/v3/files/")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("supportsAllDrives", "true")
        .body(request_body)
        .send()
        .await
        .expect("failed to create folder")
        .json::<CreateFolderResponse>()
        .await
        .expect("failed to decode json response");

    return response;
    // dbg!(response);
}

pub async fn check_if_folder_exists(configs: &Config, token: &AccessToken) -> bool {
    if configs.remote_root_id.is_empty() {
        return false;
    }

    let client = reqwest::Client::new();
    let access_token = token.token().unwrap().to_string();

    let response = client
        .get(format!(
            "https://www.googleapis.com/drive/v3/files/{}",
            configs.remote_root_id
        ))
        .header("Authorization", format!("Bearer {}", access_token))
        .header("supportsAllDrives", "true")
        .send()
        .await;

    let status_code = response.unwrap().status();
    return if status_code.is_success() {
        true
    } else {
        false
    };
}
