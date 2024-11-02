use async_trait::async_trait;
use reqwest::Client;
use crate::error::ServiceError;
#[async_trait]
pub trait NotificationService {
    async fn send_notification(&self, message: &str) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct TelegramService {
    bot_token: String,
    group_id: String,
    client: Client,
}

impl TelegramService {
    pub fn new(bot_token: String, group_id: String) -> Self {
        Self {
            bot_token,
            group_id,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl NotificationService for TelegramService {
    async fn send_notification(&self, message: &str) -> Result<(), ServiceError> {
        println!("Sending notification to Telegram...");
        
        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let response = self.client
            .post(&url)
            .json(&serde_json::json!({
                "chat_id": self.group_id,
                "text": message,
                "parse_mode": "HTML"
            }))
            .send()
            .await?;

        println!("Notification sent with status: {}", response.status());

        Ok(())
    }
} 