#![allow(dead_code)]

use anyhow::{anyhow, Result};

mod message;
pub use crate::fcm::message::*;
mod notification;
pub use crate::fcm::notification::*;

#[macro_export]
macro_rules! build_notification {
    (
        $($attr_name:ident = $attr_value:expr;)*
    ) => {{
        let mut noti = crate::fcm::NotificationBuilder::new();
        $(
            noti.$attr_name($attr_value);
        )*

        noti.build()
    }};
}

pub struct Client {
    api_key: String,
    to: String,
}

impl Client {
    pub fn new(api_key: impl Into<String>, to: impl Into<String>) -> Self {
        Client {
            api_key: api_key.into(),
            to: to.into(),
        }
    }

    pub async fn send_notification(&self, notification: Notification) -> Result<surf::Response> {
        let mut message_builder = MessageBuilder::new(&self.api_key, &self.to);
        message_builder.notification(notification);

        self.send(message_builder.build()).await
    }

    pub async fn send(&self, message: Message) -> Result<surf::Response> {
        surf::post("https://fcm.googleapis.com/fcm/send")
            .set_header("Authorization", &format!("key={}", message.api_key))
            .body_json(&message.body)?
            .await
            .map_err(|e| anyhow!(e))
    }
}
