use serde::Serialize;

#[derive(Serialize)]
pub struct PushNotification {
    pub to: Vec<&'static str>,
    pub title: &'static str,
    pub body: &'static str,
}

pub async fn send_notification(push_notificaton: PushNotification) {
    let client = actix_web::client::Client::new();

    let response = client
        .post("https://exp.host/--/api/v2/push/send")
        .send_json(&push_notificaton)
        .await;

    response.expect("Failed to send push notification");
}
