use crate::error::ServiceError;
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;

#[async_trait]
pub trait BalanceService {
    async fn get_balance(&self) -> Result<f64, ServiceError>;
}

#[derive(Clone)]
pub struct BybitService {
    api_key: String,
    api_secret: String,
    account_type: String,
}

impl BybitService {
    pub fn new(api_key: String, api_secret: String, account_type: String) -> Self {
        Self {
            api_key,
            api_secret,
            account_type,
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
impl BalanceService for BybitService {
    async fn get_balance(&self) -> Result<f64, ServiceError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .to_string();
        let recv_window = "20000";
        let params = format!("accountType={}", self.account_type);
        let signature = self.generate_signature(&timestamp, recv_window, &params);

        let client = reqwest::Client::new();
        let url = format!("https://api.bybit.com/v5/account/wallet-balance?{}", params);
        
        println!("Making request to URL: {}", url); // Debug log
        
        let response = client
            .get(&url)
            .header("X-BAPI-API-KEY", &self.api_key)
            .header("X-BAPI-TIMESTAMP", &timestamp)
            .header("X-BAPI-RECV-WINDOW", recv_window)
            .header("X-BAPI-SIGN", signature)
            .send()
            .await?;

        // Store status for error checking
        let status = response.status();
        
        // Get response text once and reuse it
        let response_text = response.text().await?;
        
        // Check if the response status is successful
        if !status.is_success() {
            return Err(ServiceError::ApiError(format!(
                "API returned error status: {}. Body: {}",
                status,
                response_text
            )));
        }

        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| ServiceError::ParseError(format!("Failed to parse JSON: {}. Response: {}", e, response_text)))?;

        // Check if the API returned an error
        if let Some(ret_code) = response_json["retCode"].as_i64() {
            if ret_code != 0 {
                let ret_msg = response_json["retMsg"].as_str().unwrap_or("Unknown error");
                return Err(ServiceError::ApiError(format!(
                    "API returned error code {}: {}",
                    ret_code, ret_msg
                )));
            }
        }

        // Try to get the balance with better error handling
        let balance = response_json["result"]["list"]
            .as_array()
            .ok_or_else(|| ServiceError::ParseError("'list' not found or not an array".to_string()))?
            .first()
            .ok_or_else(|| ServiceError::ParseError("'list' is empty".to_string()))?
            ["totalEquity"]
            .as_str()
            .ok_or_else(|| ServiceError::ParseError("'totalEquity' not found or not a string".to_string()))?
            .parse::<f64>()?;

        Ok(balance)
    }
}