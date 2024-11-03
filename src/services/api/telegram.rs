use super::ApiClient;
use crate::error::ServiceError;
use async_trait::async_trait;
use serde_json::Value;
use reqwest::Client;
use crate::constants::api::{
    TELEGRAM_BASE_URL,
    CONTENT_TYPE_HEADER,
    CONTENT_TYPE_JSON,
};

#[derive(Clone)]
pub struct TelegramApiClient {
    bot_token: String,
    client: Client,
    base_url: String,
}

impl TelegramApiClient {
    pub fn new(bot_token: String) -> Self {
        Self {
            bot_token,
            client: Client::new(),
            base_url: TELEGRAM_BASE_URL.to_string(),
        }
    }

    pub fn get_bot_url(&self) -> String {
        format!("{}/bot{}", self.base_url, self.bot_token)
    }
}

#[async_trait]
impl ApiClient for TelegramApiClient {
    async fn make_request(&self, endpoint: &str, params: &str) -> Result<Value, ServiceError> {
        let url = format!("{}/{}", self.get_bot_url(), endpoint);

        let response = self.client
            .post(&url)
            .header(CONTENT_TYPE_HEADER, CONTENT_TYPE_JSON)
            .body(params.to_string())
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(ServiceError::ApiError(format!(
                "Telegram API returned error status: {}. Body: {}",
                status,
                response_text
            )));
        }

        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| ServiceError::ParseError(format!("Failed to parse JSON: {}. Response: {}", e, response_text)))?;

        // Check Telegram API response
        if !response_json["ok"].as_bool().unwrap_or(false) {
            let error_code = response_json["error_code"].as_i64().unwrap_or(0);
            let description = response_json["description"].as_str().unwrap_or("Unknown error");
            return Err(ServiceError::ApiError(format!(
                "Telegram API error {}: {}",
                error_code,
                description
            )));
        }

        Ok(response_json)
    }
} 