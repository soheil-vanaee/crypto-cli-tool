use clap::{Parser, Subcommand};
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

/// A simple CLI to fetch cryptocurrency data
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get the list of all coins
    ListCoins,
    /// Get details for a specific coin
    CoinDetails {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
    },
    /// Get the price of a specific coin in a target currency
    CoinPrice {
        /// Coin ID (e.g., btc-bitcoin)
        coin_id: String,
        /// Target currency (e.g., usd, usdt, eth, doge)
        target_currency: String,
    },
    /// Compare the prices of two coins in a target currency
    CompareCoins {
        /// First coin ID (e.g., btc-bitcoin)
        coin1_id: String,
        /// Second coin ID (e.g., eth-ethereum)
        coin2_id: String,
        /// Target currency (e.g., usd, usdt)
        target_currency: String,
    },
}

#[derive(Deserialize, Debug)]
struct Coin {
    id: String,
    name: String,
    symbol: String,
    rank: u32,
}

#[derive(Deserialize, Debug)]
struct CoinDetail {
    id: String,
    name: String,
    symbol: String,
    description: String,
    rank: u32,
}

#[derive(Deserialize, Debug)]
struct TickerResponse {
    id: String,
    name: String,
    symbol: String,
    rank: u32,
    quotes: HashMap<String, MarketQuote>,
}

#[derive(Deserialize, Debug)]
struct MarketQuote {
    price: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Display banner with options
    println!("=========================================================");
    println!("█▀█ █▀█ █▀▀ █▀█ ▀█▀ █ █▀▀ ▀█▀ █ █▄ █ █▀▀ █▀█ ▀█▀ █▄ █ ");
    println!("█▄█ █▀▄ █▄▄ █▀▄  █  █ █▄▄  █  █ █ ▀█ ██▄ █▀▄  █  █ ▀█");
    println!("=========================================================");
    println!("             Welcome to the Crypto CLI Tool              ");
    println!("=========================================================");
    println!("Fetch real-time cryptocurrency data like prices, details,\n");
    println!("compare coins, and more!\n");
    println!("Available Commands:");
    println!("  - ListCoins              -> Show a list of all coins");
    println!("  - CoinDetails <coin_id>   -> Show details for a specific coin");
    println!("  - CoinPrice <coin_id> <target_currency> -> Get the price of a coin in a target currency");
    println!("  - CompareCoins <coin1_id> <coin2_id> <target_currency> -> Compare two coins");
    println!("=========================================================\n");

    let cli = Cli::parse();

    match &cli.command {
        Commands::ListCoins => {
            list_coins().await?;
        }
        Commands::CoinDetails { coin_id } => {
            get_coin_details(coin_id).await?;
        }
        Commands::CoinPrice {
            coin_id,
            target_currency,
        } => {
            get_coin_price(coin_id, target_currency).await?;
        }
        Commands::CompareCoins {
            coin1_id,
            coin2_id,
            target_currency,
        } => {
            compare_coin_prices(coin1_id, coin2_id, target_currency).await?;
        }
    }

    Ok(())
}

async fn list_coins() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.coinpaprika.com/v1/coins";
    let response = reqwest::get(url).await?.json::<Vec<Coin>>().await?;

    println!("=======================================");
    println!("          Listing All Coins            ");
    println!("=======================================\n");

    for coin in response {
        println!("{} ({}) - Rank: {}", coin.name, coin.symbol, coin.rank);
    }

    Ok(())
}

async fn get_coin_details(coin_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.coinpaprika.com/v1/coins/{}", coin_id);
    let response = reqwest::get(&url).await?.json::<CoinDetail>().await?;

    println!("\n=======================================");
    println!("        Coin Details for {} ({})        ", response.name, response.symbol);
    println!("=======================================\n");

    println!(
        "Name: {}\nSymbol: {}\nDescription: {}\nRank: {}",
        response.name, response.symbol, response.description, response.rank
    );

    Ok(())
}

async fn get_coin_price(coin_id: &str, target_currency: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.coinpaprika.com/v1/tickers/{}", coin_id);
    let response = reqwest::get(&url).await?.json::<TickerResponse>().await?;

    let target_currency_upper = target_currency.to_uppercase();
    println!("\n=======================================");
    println!("       Price for {} ({}) in {}        ", response.name, response.symbol, target_currency_upper);
    println!("=======================================\n");

    if let Some(quote) = response.quotes.get(&target_currency_upper) {
        println!(
            "1 {} ({}) = {} {}",
            response.name, response.symbol, quote.price, target_currency_upper
        );
    } else {
        println!(
            "Could not find price information for {} in {}",
            response.name, target_currency_upper
        );
    }

    Ok(())
}

async fn compare_coin_prices(
    coin1_id: &str,
    coin2_id: &str,
    target_currency: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let coin1_price = get_coin_price_value(coin1_id, target_currency).await?;
    let coin2_price = get_coin_price_value(coin2_id, target_currency).await?;

    println!("\n=======================================");
    println!("  Comparing {} and {} in {} Currency  ", coin1_id, coin2_id, target_currency.to_uppercase());
    println!("=======================================\n");

    println!("{} price: {} {}", coin1_id, coin1_price, target_currency.to_uppercase());
    println!("{} price: {} {}", coin2_id, coin2_price, target_currency.to_uppercase());

    if coin1_price > coin2_price {
        println!(
            "{} is more valuable than {} in {}",
            coin1_id, coin2_id, target_currency.to_uppercase()
        );
    } else if coin1_price < coin2_price {
        println!(
            "{} is more valuable than {} in {}",
            coin2_id, coin1_id, target_currency.to_uppercase()
        );
    } else {
        println!(
            "Both {} and {} have the same value in {}",
            coin1_id, coin2_id, target_currency.to_uppercase()
        );
    }

    Ok(())
}

async fn get_coin_price_value(coin_id: &str, target_currency: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("https://api.coinpaprika.com/v1/tickers/{}", coin_id);
    let response = reqwest::get(&url).await?.json::<TickerResponse>().await?;

    let target_currency_upper = target_currency.to_uppercase();
    if let Some(quote) = response.quotes.get(&target_currency_upper) {
        Ok(quote.price)
    } else {
        Err(format!(
            "Could not find price information for {} in {}",
            coin_id, target_currency_upper
        )
        .into())
    }
}
