use crate::config::Config;
use crate::error::ServiceError;
use crate::services::bybit_service::{BybitService, BybitServiceImpl};
use crate::services::telegram_service::{TelegramService, TelegramServiceImpl};
use tokio_cron_scheduler::{Job, JobScheduler};
use std::time::Duration;
use chrono::Local;
use std::sync::Arc;

pub struct SchedulerService {
    scheduler: JobScheduler,
    config: Config,
    bybit_service: Arc<BybitServiceImpl>,
    telegram_service: Arc<TelegramServiceImpl>,
}

impl SchedulerService {
    pub async fn new(
        config: Config,
        bybit_service: BybitServiceImpl,
        telegram_service: TelegramServiceImpl,
    ) -> Result<Self, ServiceError> {
        let scheduler = JobScheduler::new()
            .await
            .map_err(|e| ServiceError::RequestError(e.to_string()))?;

        Ok(Self {
            scheduler,
            config,
            bybit_service: Arc::new(bybit_service),
            telegram_service: Arc::new(telegram_service),
        })
    }

    pub async fn start(&self) -> Result<(), ServiceError> {
        println!("Adding job to scheduler...");
        
        let bybit_service = self.bybit_service.clone();
        let telegram_service = self.telegram_service.clone();
        let account_type = self.config.account_type.clone();
        let check_interval = self.config.check_interval;

        self.scheduler.add(Job::new_repeated_async(
            Duration::from_secs(check_interval),
            move |_uuid, _l| {
                let bybit_svc = bybit_service.clone();
                let telegram_svc = telegram_service.clone();
                let account_type = account_type.clone();
                
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

                    let last_pnl: f64 = closed_pnl.iter()
                        .filter_map(|p| p["closedPnl"].as_str())
                        .filter_map(|s| s.parse::<f64>().ok())
                        .sum();

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

        self.scheduler.start().await
            .map_err(|e| ServiceError::RequestError(e.to_string()))?;

        Ok(())
    }
} 