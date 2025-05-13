use anyhow::Result;
use auth::{minecraft, oauth, xbox_live, xsts};
mod auth;

fn main() -> Result<()> {
    // TODO: wait for aprouval, handle errors...
    dotenvy::dotenv().unwrap();

    let client_id = std::env::var("CLIENT_ID").expect("Expected `CLIENT_ID` in .env");

    let access_token = oauth(client_id)?;

    let (token, xl_user_hash) = xbox_live(access_token)?;
    let (token, xsts_user_hash) = xsts(token)?;
    assert_eq!(xl_user_hash, xsts_user_hash);

    let (token, uuid) = minecraft(xsts_user_hash, token)?;
    println!("{token}\n{uuid}");
    Ok(())
}
