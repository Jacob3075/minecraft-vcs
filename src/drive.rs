use serde::{Deserialize, Serialize};
use yup_oauth2::AccessToken;

pub async fn create_folder_in_drive(token: AccessToken) {
    let client = reqwest::Client::new();
    let access_token = token.token().clone().unwrap().to_string();

    let data = FileCreationData {
        mime_type: "application/vnd.google-apps.folder".to_string(),
        name: "minecraft-vcs".to_string(),
        parents: vec![],
    };

    let response = client
        .post("https://www.googleapis.com/drive/v3/files/")
        .header("Authorization", format!("Bearer {}", access_token))
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
        .unwrap()
        .text()
        .await;

    dbg!(response);
}

#[derive(Serialize, Deserialize)]
struct FileCreationData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub name: String,
    pub parents: Vec<String>,
}
