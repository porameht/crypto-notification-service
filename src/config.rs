use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub bybit_api_key: String,
    pub bybit_api_secret: String,
    pub telegram_bot_token: String,
    pub telegram_group_id: String,
    pub check_interval: u64,
    pub account_type: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        
        Self {
            bybit_api_key: env::var("BYBIT_API_KEY").expect("BYBIT_API_KEY must be set"),
            bybit_api_secret: env::var("BYBIT_API_SECRET").expect("BYBIT_API_SECRET must be set"),
            account_type: env::var("ACCOUNT_TYPE").expect("ACCOUNT_TYPE must be set"),
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set"),
            telegram_group_id: env::var("TELEGRAM_GROUP_ID").expect("TELEGRAM_GROUP_ID must be set"),
            check_interval: env::var("CHECK_INTERVAL").expect("CHECK_INTERVAL must be set").parse().unwrap_or(3600),
        }
    }
} 