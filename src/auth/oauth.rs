use std::sync::mpsc::channel;

use oauth2::{
    basic::BasicClient, reqwest, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use rouille::Response;

pub fn oauth(client_id: String) -> String {
    let client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new("secret".to_string()))
        .set_auth_uri(
            AuthUrl::new("https://login.live.com/oauth20_authorize.srf".to_string()).unwrap(),
        )
        .set_token_uri(
            TokenUrl::new("https://login.live.com/oauth20_token.srf".to_string()).unwrap(),
        )
        .set_redirect_uri(RedirectUrl::new("http://localhost:80".to_string()).unwrap());

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("Xboxlive.signin".to_string()))
        .add_scope(Scope::new("Xboxlive.offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    let http_client = reqwest::blocking::ClientBuilder::new()
        // Following redirects opens the client up to SSRF vulnerabilities.
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let (sender, receiver) = channel::<String>();
    let server = rouille::Server::new("localhost:80", move |req| {
        let code = req
            .raw_query_string()
            .to_string()
            .split("&")
            .next()
            .unwrap()
            .split("=")
            .collect::<Vec<_>>()[1]
            .to_string();
        let _ = sender.send(code.clone());
        Response::html(format!("{}<br>You can close this tab", code)).with_status_code(200)
    })
    .unwrap();
    let (handle, server_sender) = server.stoppable();

    let auth_code = receiver.recv().unwrap();

    let _ = server_sender.send(());
    let _ = handle.join();

    let client = client.set_client_secret(ClientSecret::new("".to_string()));

    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request(&http_client)
        .unwrap();

    token_result.access_token().clone().into_secret()
}
