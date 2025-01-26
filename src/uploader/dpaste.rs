pub async fn upload_to_dpaste(content: &String) -> String {
    let expiry_days = "1".to_string();
    let syntax = "lua".to_string();

    let params = [
        ("content", content),
        ("expiry_days", &expiry_days),
        ("syntax", &syntax),
    ];

    let client = reqwest::Client::new();

    let res = client
        .post("https://dpaste.com/api/")
        .form(&params)
        .send()
        .await;

    let mut url = res.unwrap().headers()["location"]
        .to_str()
        .unwrap()
        .to_string();

    url.push_str(".txt"); // we need the raw version

    url
}
