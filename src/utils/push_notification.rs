use std::collections::HashMap;

pub async fn send_notification(token: &str, title: &str, msg: &str) {
    let mut json = HashMap::new();
    json.insert("to", token);
    json.insert("title", title);
    json.insert("body", msg);

    let client = actix_web::client::Client::new();

    let response = client
        .post("https://exp.host/--/api/v2/push/send")
        .send_json(&json)
        .await;

    response.expect("Failed to send push notification");
}