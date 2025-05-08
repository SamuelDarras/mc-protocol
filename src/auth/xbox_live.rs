use json::object;

pub fn xbox_live(access_token: String) -> (String, String) {
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
        .build()
        .unwrap();

    let result = client.execute(request).unwrap();
    let json_result = json::parse(result.text().unwrap().as_str()).unwrap();

    let mut token = json_result["Token"].clone();
    let mut uhs = json_result["DisplayClaims"]["xui"][0]["uhs"].clone();

    (token.take_string().unwrap(), uhs.take_string().unwrap())
}
