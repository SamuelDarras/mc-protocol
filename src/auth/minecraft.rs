use anyhow::Result;
use json::object;

pub fn minecraft(uhs: String, token: String) -> Result<(String, String)> {
    let client = reqwest::blocking::Client::new();

    let identity_token = format!("XBL3.0 x={uhs};{token}");

    let request = client
        .post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(
            object! {
                    "identityToken": identity_token
            }
            .to_string(),
        )
        .build()?;

    let result = client.execute(request)?;
    println!("{result:?}");
    let mut json_result = json::parse(result.text()?.as_str())?;

    let token = json_result["access_token"]
        .take_string()
        .unwrap_or_default();
    let uuid = json_result["username"].take_string().unwrap_or_default();

    Ok((token, uuid))
}
