mod config;
mod error;
mod services;

use config::Config;
use error::ServiceError;
use services::bybit_service::{BalanceService, BybitService};
use services::notification_service::{NotificationService, TelegramService};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let config = Config::new();
    
    let bybit_service = BybitService::new(
        config.bybit_api_key,
        config.bybit_api_secret,
        config.account_type,
    );
    
    let telegram_service = TelegramService::new(
        config.telegram_bot_token,
        config.telegram_group_id,
    );

    println!("Starting notification service...");

    let scheduler = JobScheduler::new().await.map_err(|e| ServiceError::RequestError(e.to_string()))?;
    
    println!("Adding job to scheduler...");
    
    scheduler.add(Job::new_repeated_async(
        Duration::from_secs(config.check_interval),
        move |_uuid, _l| {
            let bybit_svc = bybit_service.clone();
            let telegram_svc = telegram_service.clone();
            
            Box::pin(async move {
                println!("Getting balance...");
                match bybit_svc.get_balance().await {
                    Ok(balance) => {
                        let message = format!(
                            "💰 Current Balance Update 💰\nTotal Balance: ${:.2} USD",
                            balance
                        );
                        
                        if let Err(e) = telegram_svc.send_notification(&message).await {
                            eprintln!("Error sending notification: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error getting balance: {}", e);
                    }
                }
            })
        }
    ).map_err(|e| ServiceError::RequestError(e.to_string()))?).await
        .map_err(|e| ServiceError::RequestError(e.to_string()))?;

    scheduler.start().await.map_err(|e| ServiceError::RequestError(e.to_string()))?;
    
    // Keep the main thread running
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
