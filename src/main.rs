mod config;
mod error;
mod services;

use config::Config;
use error::ServiceError;
use services::bybit_service::BybitServiceImpl;
use services::telegram_service::TelegramServiceImpl;
use services::scheduler_service::SchedulerService;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let config = Config::new();
    
    let bybit_service = BybitServiceImpl::new(
        config.bybit_api_key.clone(),
        config.bybit_api_secret.clone(),
        config.account_type.clone(),
    );
    
    let telegram_service = TelegramServiceImpl::new(
        config.telegram_bot_token.clone(),
        config.telegram_group_id.clone(),
    );

    println!("Starting notification service...");
    let check_interval = config.check_interval;
    let scheduler_service = SchedulerService::new(
        config,
        bybit_service,
        telegram_service,
    ).await?;

    scheduler_service.start().await?;
    
    loop {
        tokio::time::sleep(Duration::from_secs(check_interval)).await;
    }
}
