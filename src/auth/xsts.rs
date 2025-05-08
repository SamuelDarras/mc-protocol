use json::object;

pub fn xsts(token: String) -> (String, String) {
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
        .build()
        .unwrap();

    let result = client.execute(request).unwrap();
    let json_result = json::parse(result.text().unwrap().as_str()).unwrap();

    let mut token = json_result["Token"].clone();
    let mut uhs = json_result["DisplayClaims"]["xui"][0]["uhs"].clone();

    (token.take_string().unwrap(), uhs.take_string().unwrap())
}
