use json::object;

pub fn minecraft(uhs: String, token: String) -> (String, String) {
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
        .build()
        .unwrap();

    let result = client.execute(request).unwrap();
    println!("{result:?}");
    let json_result = json::parse(result.text().unwrap().as_str()).unwrap();

    let mut token = json_result["access_token"].clone();
    let mut uuid = json_result["username"].clone();

    (token.take_string().unwrap(), uuid.take_string().unwrap())
}
