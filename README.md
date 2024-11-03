# Crypto Notification Service

A Rust-based service that monitors Bybit cryptocurrency trading accounts and sends periodic status updates via Telegram. Built with async Rust and supports multiple trading accounts.

## Features

- 🔄 Real-time monitoring of Bybit trading accounts
- 💰 Track account balance and positions
- 📊 Monitor PnL (Profit and Loss)
  - Last 100 trades PnL tracking
  - Current unrealized PnL monitoring
- 📱 Telegram notifications
  - HTML formatted messages
  - Secure bot integration
  - Group chat support
- ⚡ High Performance
  - Async operations with Tokio
  - Efficient scheduler implementation
  - Configurable update intervals
- 🔐 Security Features
  - HMAC SHA256 API authentication
  - Non-root Docker container
  - Environment-based configuration
- 🐳 Deployment Ready
  - Multi-stage Docker builds
  - SSL/TLS support included
  - Volume mounting for configuration

## Prerequisites

- Rust 1.76 or higher
- Docker (optional)
- Bybit API credentials
  - API Key
  - API Secret
  - Account Type (UNIFIED/CONTRACT)
- Telegram Bot
  - Bot Token
  - Group ID

## Configuration

Create a `.env` file in the project root:

## How to Run

### Local Development

1. Clone the repository:
```bash
git clone <repository-url>
cd crypto_notification_service
```

2. Create a `.env` file in the project root with the following variables:
```bash
BYBIT_API_KEY=your_api_key
BYBIT_API_SECRET=your_api_secret
ACCOUNT_TYPE=UNIFIED  # or CONTRACT
TELEGRAM_BOT_TOKEN=your_bot_token
TELEGRAM_GROUP_ID=your_group_id
CHECK_INTERVAL=3600  # Update interval in seconds
```

3. Build and run the project:
```bash
cargo build --release
cargo run --release
```
### Docker Deployment

1. Build the Docker image:
```bash
docker build -t crypto-notification-service .
```

2. Run the Docker container:
```bash
docker run -d \
--name crypto-notification \
-v $(pwd)/.env:/app/.env \
crypto-notification-service
```

## Verify Operation

Once running, the service will:
- Start monitoring your Bybit account
- Send periodic updates to your Telegram group
- Log operational status to the console

You can verify operation by checking:
- Docker logs: `docker logs crypto-notification`
- Telegram messages in your configured group
- Console output in local development mode

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch:
```bash
git checkout -b feature-name
```

3. Make your changes and submit a pull request
```bash
git commit -m "Add some feature"
```

4. Push to your fork:
```bash
git push origin feature-name
```

### Development Guidelines

- Follow Rust coding conventions and style guidelines
- Add tests for new functionality
- Update documentation for significant changes
- Use descriptive commit messages
- Keep pull requests focused on a single feature


### Code Style

Before submitting a PR, ensure your code follows the project's style:
```bash
cargo fmt
cargo clippy
```

## License

MIT License - See LICENSE file for details