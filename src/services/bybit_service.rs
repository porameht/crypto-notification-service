use crate::error::ServiceError;
use crate::services::api::bybit::BybitApiClient;
use async_trait::async_trait;
use serde_json::Value;
use crate::services::api::ApiClient;

#[async_trait]
pub trait BybitService {
    async fn get_balance(&self) -> Result<f64, ServiceError>;
    async fn get_positions(&self, limit: u32) -> Result<Vec<Value>, ServiceError>;
    async fn get_closed_pnl(&self, limit: u32) -> Result<Vec<Value>, ServiceError>;
}

#[derive(Clone)]
pub struct BybitServiceImpl {
    api_client: BybitApiClient,
    account_type: String,
}

impl BybitServiceImpl {
    pub fn new(api_key: String, api_secret: String, account_type: String) -> Self {
        Self {
            api_client: BybitApiClient::new(api_key, api_secret),
            account_type,
        }
    }

    // Helper to extract array from response
    fn extract_list(response: Value) -> Result<Vec<Value>, ServiceError> {
        response["result"]["list"]
            .as_array()
            .ok_or_else(|| ServiceError::ParseError("'list' not found or not an array".to_string()))
            .map(|arr| arr.to_vec())
    }
}

#[async_trait]
impl BybitService for BybitServiceImpl {
    async fn get_balance(&self) -> Result<f64, ServiceError> {
        let params = format!("accountType={}", self.account_type);
        let response = self.api_client.make_request("account/wallet-balance", &params).await?;
        
        let balance = Self::extract_list(response)?
            .first()
            .ok_or_else(|| ServiceError::ParseError("'list' is empty".to_string()))?
            ["totalEquity"]
            .as_str()
            .ok_or_else(|| ServiceError::ParseError("'totalEquity' not found or not a string".to_string()))?
            .parse::<f64>()?;

        Ok(balance)
    }

    async fn get_positions(&self, limit: u32) -> Result<Vec<Value>, ServiceError> {
        let params = format!("category=linear&settleCoin=USDT&limit={}", limit);
        let response = self.api_client.make_request("position/list", &params).await?;
        Self::extract_list(response)
    }

    async fn get_closed_pnl(&self, limit: u32) -> Result<Vec<Value>, ServiceError> {
        let params = format!("category=linear&limit={}", limit);
        let response = self.api_client.make_request("position/closed-pnl", &params).await?;
        Self::extract_list(response)
    }
}