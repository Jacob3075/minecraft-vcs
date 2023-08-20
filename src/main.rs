use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

mod configuration;
#[tokio::main]
async fn main() {
    yup_oauth_example().await;
}

async fn yup_oauth_example() {
    // Read application secret from a file. Sometimes it's easier to compile it directly into
    // the binary. The clientsecret file contains JSON like `{"installed":{"client_id": ... }}`
    let secret = yup_oauth2::read_application_secret("configs/credentials.json")
        .await
        .expect("credentials.json");

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("configs/tokencache.json")
        .build()
        .await
        .unwrap();

    let scopes = &["https://www.googleapis.com/auth/drive.file"];

    // token(<scopes>) is the one important function of this crate; it does everything to
    // obtain a token that can be sent e.g. as Bearer token.
    match auth.token(scopes).await {
        Ok(token) => println!("The token is {:?}", token),
        Err(e) => println!("error: {:?}", e),
    }
}
