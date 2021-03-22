extern crate reqwest;
use std::collections::HashMap;

pub async fn send_notification(token: &String, msg: String) {
    let mut json = HashMap::new();
    json.insert("to", format!("ExponentPushToken[{}]", &token));
    json.insert("title", String::from("Hello, World!"));
    json.insert("body", msg);

    let client = reqwest::Client::new();
    let x = client.post("https://exp.host/--/api/v2/push/send")
        .json(&json)
        .send()
        .await;
    
    x.expect("Failed to send push notification");
}