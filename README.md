
# Crypto CLI Tool

A simple and fast command-line tool built with Rust to fetch real-time cryptocurrency data using the CoinPaprika API.

## Features

- **List Coins**: Display a list of available cryptocurrencies.
- **Coin Details**: Retrieve detailed information about a specific cryptocurrency.
- **Coin Price**: Get the current price of a cryptocurrency in a target currency (e.g., USD, ETH).
- **Compare Coins**: Compare the prices of two cryptocurrencies in a specified currency.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/soheil-vanaee/crypto-cli-tool.git
   cd crypto-cli-tool
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the tool**:
   ```bash
   cargo run -- <command>
   ```

## Usage

Here are some example commands you can use:

### 1. List all available coins
   ```bash
   cargo run -- list-coins
   ```

### 2. Get details of a specific coin (e.g., Bitcoin)
   ```bash
   cargo run -- coin-details btc-bitcoin
   ```

### 3. Get the price of a coin in a specific currency (e.g., USD)
   ```bash
   cargo run -- coin-price btc-bitcoin usd
   ```

### 4. Compare the prices of two coins (e.g., Bitcoin and Ethereum in USD)
   ```bash
   cargo run -- compare-coins btc-bitcoin eth-ethereum usd
   ```

## License

This project is licensed under the MIT License.
```

This markdown was created with artificial intelligence.
