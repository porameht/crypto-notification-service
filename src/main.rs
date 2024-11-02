mod config;
mod error;
mod services;

use config::Config;
use error::ServiceError;
use services::bybit_service::{BalanceService, BybitService};
use services::notification_service::{NotificationService, TelegramService};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::time::Duration;
use chrono::Local;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let config = Config::new();
    
    let bybit_service = BybitService::new(
        config.bybit_api_key,
        config.bybit_api_secret,
        config.account_type.clone(),
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
            let account_type = config.account_type.clone();
            
            Box::pin(async move {
                let balance = match bybit_svc.get_balance().await {
                    Ok(b) => format!("{:.2}", b),
                    Err(e) => {
                        eprintln!("Error getting balance: {}", e);
                        "Error".to_string()
                    }
                };

                let positions = match bybit_svc.get_positions(10).await {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Error getting positions: {}", e);
                        vec![]
                    }
                };

                let closed_pnl = match bybit_svc.get_closed_pnl(100).await {
                    Ok(pnl) => pnl,
                    Err(e) => {
                        eprintln!("Error getting closed PnL: {}", e);
                        vec![]
                    }
                };

                // Calculate total PnL from closed positions
                let last_pnl: f64 = closed_pnl.iter()
                    .filter_map(|p| p["closedPnl"].as_str())
                    .filter_map(|s| s.parse::<f64>().ok())
                    .sum();

                // Calculate current PnL from open positions
                let current_pnl: f64 = positions.iter()
                    .filter_map(|p| p["unrealisedPnl"].as_str())
                    .filter_map(|s| s.parse::<f64>().ok())
                    .sum();

                let message = format!(
                    "<b>✨ Account Status ({}) ✨</b>\n\
                    <b>💰 Balance:</b> <code>{} USDT</code>\n\
                    <b>⏱️ Timeframe:</b> <code>1m</code>\n\
                    <b>📂 Open Positions:</b> <code>{}</code>\n\
                    <b>💰 Last 100 P&L:</b> <code>{:.2} USDT</code>\n\
                    <b>💹 Current P&L:</b> <code>{:.2} USDT</code>\n\n\
                    <i>🔸 Generated at: <code>{}</code></i>",
                    account_type,
                    balance,
                    positions.len(),
                    last_pnl,
                    current_pnl,
                    Local::now().format("%Y-%m-%d %H:%M:%S")
                );
                
                if let Err(e) = telegram_svc.send_notification(&message).await {
                    eprintln!("Error sending notification: {}", e);
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
