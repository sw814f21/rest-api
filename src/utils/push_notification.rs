use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct PushNotification {
    pub to: Vec<String>,
    pub title: String,
    pub msg: String,
}

pub async fn send_notification(push_notificaton: PushNotification) {
    let client = actix_web::client::Client::new();

    let response = client
        .post("https://exp.host/--/api/v2/push/send")
        .send_json(&push_notificaton)
        .await;

    response.expect("Failed to send push notification");
}
