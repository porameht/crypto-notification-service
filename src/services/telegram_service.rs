use async_trait::async_trait;
use crate::error::ServiceError;
use crate::services::api::telegram::TelegramApiClient;
use crate::services::api::ApiClient;
use serde_json::json;

#[async_trait]
pub trait TelegramService {
    async fn send_notification(&self, message: &str) -> Result<(), ServiceError>;
}

#[derive(Clone)]
pub struct TelegramServiceImpl {
    api_client: TelegramApiClient,
    group_id: String,
}

impl TelegramServiceImpl {
    pub fn new(bot_token: String, group_id: String) -> Self {
        Self {
            api_client: TelegramApiClient::new(bot_token),
            group_id,
        }
    }
}

#[async_trait]
impl TelegramService for TelegramServiceImpl {
    async fn send_notification(&self, message: &str) -> Result<(), ServiceError> {
        println!("Sending notification to Telegram...");
        
        let params = json!({
            "chat_id": self.group_id,
            "text": message,
            "parse_mode": "HTML"
        }).to_string();

        self.api_client.make_request("sendMessage", &params).await?;
        
        println!("Notification sent successfully");
        Ok(())
    }
}