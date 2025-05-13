use anyhow::Result;
use json::object;

pub fn xsts(token: String) -> Result<(String, String)> {
    let client = reqwest::blocking::Client::new();

    let request = client
        .post("https://xsts.auth.xboxlive.com/xsts/authorize")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(
            object! {
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": [
                    token
                ]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
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
