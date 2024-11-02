use super::ApiClient;
use crate::error::ServiceError;
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;
use reqwest::Client;


#[derive(Clone)]
pub struct BybitApiClient {
    api_key: String,
    api_secret: String,
    client: Client,
    base_url: String,
}

impl BybitApiClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
            client: Client::new(),
            base_url: "https://api.bybit.com/v5".to_string(),
        }
    }

    fn generate_signature(&self, timestamp: &str, recv_window: &str, params: &str) -> String {
        let str_to_sign = format!("{}{}{}{}", timestamp, &self.api_key, recv_window, params);
        let mut mac = Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(str_to_sign.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }
}

#[async_trait]
impl ApiClient for BybitApiClient {
    async fn make_request(&self, endpoint: &str, params: &str) -> Result<Value, ServiceError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string();
        let recv_window = "20000";
        let signature = self.generate_signature(&timestamp, recv_window, params);
        let url = format!("{}/{}?{}", self.base_url, endpoint, params);

        let response = self.client
            .get(&url)
            .header("X-BAPI-API-KEY", &self.api_key)
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .header("X-BAPI-SIGN", signature)
            .send()
            .await?;

        let status = response.status();
        let response_text = response.text().await?;

        if !status.is_success() {
            return Err(ServiceError::ApiError(format!(
                "API returned error status: {}. Body: {}",
                status,
                response_text
            )));
        }

        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| ServiceError::ParseError(format!("Failed to parse JSON: {}. Response: {}", e, response_text)))?;

        // Check API error codes
        if let Some(ret_code) = response_json["retCode"].as_i64() {
            if ret_code != 0 {
                let ret_msg = response_json["retMsg"].as_str().unwrap_or("Unknown error");
                return Err(ServiceError::ApiError(format!(
                    "API returned error code {}: {}",
                    ret_code, ret_msg
                )));
            }
        }

        Ok(response_json)
    }
} 