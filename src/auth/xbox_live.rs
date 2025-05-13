use anyhow::Result;
use json::object;

pub fn xbox_live(access_token: String) -> Result<(String, String)> {
    let client = reqwest::blocking::Client::new();
    let access_token = format!("d={}", access_token);
    let request = client
        .post("https://user.auth.xboxlive.com/user/authenticate")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(
            object! {
                "Properties": {
                    "AuthMethod": "RPS",
                    "SiteName": "user.auth.xboxlive.com",
                    "RpsTicket": access_token // your access token from the previous step here, make sure that it is prefixed with `d=`
                },
                "RelyingParty": "http://auth.xboxlive.com",
                "TokenType": "JWT"
            }
            .to_string(),
        )
        .build()?;

    let result = client.execute(request)?;
    let mut json_result = json::parse(result.text()?.as_str())?;

    let token = json_result["Token"].take_string().unwrap_or_default();
    let uhs = json_result["DisplayClaims"]["xui"][0]["uhs"]
        .take_string()
        .unwrap_or_default();

    Ok((token, uhs))
}
