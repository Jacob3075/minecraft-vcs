use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileCreationData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub name: String,
    pub parents: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FileDetails {
    pub kind: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileList {
    pub kind: String,
    pub files: Vec<FileDetails>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateFolderResponse {
    pub kind: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}
