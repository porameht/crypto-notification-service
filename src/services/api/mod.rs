pub mod bybit;
pub mod telegram;

use async_trait::async_trait;
use serde_json::Value;
use crate::error::ServiceError;

#[async_trait]
pub trait ApiClient {
    async fn make_request(&self, endpoint: &str, params: &str) -> Result<Value, ServiceError>;
} 