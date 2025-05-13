use std::sync::mpsc::channel;

use anyhow::Result;
use oauth2::{
    basic::BasicClient, reqwest, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    PkceCodeChallenge, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use regex::Regex;
use rouille::Response;

pub fn oauth(client_id: String) -> Result<String> {
    let client_secret = std::env::var("CLIENT_SECRET").expect("Expected `CLIENT_SECRET` in .env");

    let client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new(
            "https://login.live.com/oauth20_authorize.srf".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://login.live.com/oauth20_token.srf".to_string(),
        )?)
        .set_redirect_uri(RedirectUrl::new("http://localhost:80".to_string())?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("Xboxlive.signin".to_string()))
        .add_scope(Scope::new("Xboxlive.offline_access".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    println!("Browse to: {}", auth_url);

    let http_client = reqwest::blocking::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let (sender, receiver) = channel::<(String, String)>();
    let server = rouille::Server::new("localhost:80", move |req| {
        let regex = Regex::new(r"code=([^&]+)&state=([^&]+)").unwrap();
        let capture = regex
            .captures(req.raw_query_string())
            .map(|capture| capture.extract::<2>())
            .unwrap()
            .1;
        let _ = sender.send(capture.map(|s| s.to_string()).into());
        Response::html(format!(
            "{}<br>You can close this tab",
            req.raw_query_string()
        ))
        .with_status_code(200)
    })
    .unwrap_or_else(|e| panic!("Error: {}", e));
    let (handle, server_sender) = server.stoppable();

    let (auth_code, state) = receiver.recv()?;

    assert_eq!(csrf_token.into_secret(), state);

    let _ = server_sender.send(());
    let _ = handle.join();

    let client = client.set_client_secret(ClientSecret::new("".to_string()));

    let token_result = client
        .exchange_code(AuthorizationCode::new(auth_code))
        .set_pkce_verifier(pkce_verifier)
        .request(&http_client)?;

    Ok(token_result.access_token().secret().clone())
}
