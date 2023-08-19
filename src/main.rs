mod configuration;

use std::default::Default;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use oauth2::{AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl, RevocationUrl, Scope, StandardRevocableToken, TokenResponse, TokenUrl};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::url::Url;
use crate::configuration::get_config_values;

fn main() {
    sadfg();
}

fn sadfg() {
    let configs = get_config_values();

    let client_id = configs.api_keys.client_id;
    let client_secret = configs.api_keys.client_secret;
    let redirect_uri = configs.api_keys.redirect_uri;
    let auth_url = configs.api_keys.auth_url;
    let token_url = configs.api_keys.token_url;
    let revocation_uri = configs.api_keys.revocation_uri;


    let google_client_id = ClientId::new(client_id);
    let google_client_secret = ClientSecret::new(client_secret);
    let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(token_url).expect("Invalid token endpoint URL");

    // Set up the config for the Google OAuth2 process.
    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_uri(RedirectUrl::new(redirect_uri).expect("Invalid redirect URL"))
        // Google supports OAuth 2.0 Token Revocation (RFC-7009)
        .set_revocation_uri(RevocationUrl::new(revocation_uri).expect("Invalid revocation endpoint URL"));

    // Google supports Proof Key for Code Exchange (PKCE - https://oauth.net/2/pkce/).
    // Create a PKCE code verifier and SHA-256 encode it as a code challenge.
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the "calendar" features and the user's profile.
        .add_scope(Scope::new("https://www.googleapis.com/auth/drive.file".to_string()))
        .add_scope(Scope::new("https://www.googleapis.com/auth/drive.appdata".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Google returned the following code:\n{}\n", code.secret());
            println!(
                "Google returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_state.secret()
            );

            // Exchange the code with a token.
            let token_response = client
                .exchange_code(code)
                .set_pkce_verifier(pkce_code_verifier)
                .request(http_client);

            println!(
                "Google returned the following token:\n{:?}\n",
                token_response
            );

            // Revoke the obtained token
            let token_response = token_response.unwrap();
            let token_to_revoke: StandardRevocableToken = match token_response.refresh_token() {
                Some(token) => token.into(),
                None => token_response.access_token().into(),
            };

            client
                .revoke_token(token_to_revoke)
                .unwrap()
                .request(http_client)
                .expect("Failed to revoke token");

            // The server will terminate itself after revoking the token.
            break;
        }
    }
}
