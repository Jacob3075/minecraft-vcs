use crate::configuration::Credentials;
use yup_oauth2::{AccessToken, Error, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

pub async fn get_token(credentials: &Credentials) -> Result<AccessToken, Error> {
    let secret = yup_oauth2::read_application_secret(&credentials.secrets_location).await?;

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(&credentials.cache_location)
        .build()
        .await
        .unwrap();

    let scopes = &[
        "https://www.googleapis.com/auth/drive",
        "https://www.googleapis.com/auth/drive.file",
        "https://www.googleapis.com/auth/drive.appdata",
    ];

    return auth.token(scopes).await;
}
