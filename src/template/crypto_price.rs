use futures::executor::block_on;

use crate::bit::*;
use crate::template::Template;
use crate::CryptoTickers;

pub struct CryptoPrice {
    pub tickers: Vec<CryptoTickers>,
}

impl CryptoPrice {
    pub fn new(tickers: Vec<CryptoTickers>) -> CryptoPrice {
        CryptoPrice { tickers }
    }
}

impl Template for CryptoPrice {
    fn render(&self) -> Message {
        let mut text = vec!["".to_string(); 6];

        let evol = get_evolution(&self.tickers);

        let prices = block_on(get_prices(&self.tickers));
        let evolution = block_on(evol);

        for (i, ticker) in self.tickers.iter().enumerate() {
            text[i] = format!(
                "{} ${:>7.1}  {:>+6.1}%",
                ticker,
                prices[i],
                evolution[i] * 100.0
            );
        }
        let mut message = Message::from_text(&text);

        for (i, e) in evolution.iter().enumerate() {
            if *e < 0.0 {
                message.rows[i].bits[15] = Bit::Color(Color::Red);
                message.rows[i].bits[21] = Bit::Color(Color::Red);
            } else {
                message.rows[i].bits[15] = Bit::Color(Color::Green);
                message.rows[i].bits[21] = Bit::Color(Color::Green);
            }
        }

        message
    }
}

async fn get_evolution(tickers: &Vec<CryptoTickers>) -> Vec<f64> {
    let mut evolution = vec![0.0; 0];
    for ticker in tickers {
        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency=usd&days=1",
            get_id_from_ticker(ticker)
        );
        let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();

        let json: serde_json::Value = serde_json::from_str(&resp).unwrap();
        let prices = json["prices"].as_array().unwrap();
        let last_price = prices[prices.len() - 1][1].as_f64().unwrap();
        let first_price = prices[0][1].as_f64().unwrap();
        evolution.push((last_price - first_price) / first_price);
    }

    evolution
}

async fn get_prices(tickers: &Vec<CryptoTickers>) -> Vec<f64> {
    let mut ids = String::new();
    for ticker in tickers {
        ids.push_str(get_id_from_ticker(ticker));
        ids.push(',');
    }

    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        ids
    );
    let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();

    let json: serde_json::Value = serde_json::from_str(&resp).unwrap();

    let mut prices = vec![0.0; 0];
    for ticker in tickers {
        prices.push(
            json[get_id_from_ticker(ticker)]["usd"]
                .as_f64()
                .unwrap_or(0.0),
        );
    }

    prices
}

fn get_id_from_ticker(ticker: &CryptoTickers) -> &'static str {
    match ticker {
        CryptoTickers::Eth => "ethereum",
        CryptoTickers::Btc => "bitcoin",
        CryptoTickers::Sol => "solana",
        CryptoTickers::Bnb => "binancecoin",
    }
}
