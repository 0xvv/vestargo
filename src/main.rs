use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::template::crypto_price::CryptoPrice;
use crate::template::heart::Heart;
use crate::template::meteo::Meteo;
use crate::template::rainbow::Rainbow;
use crate::template::text::Text;
use crate::template::Template;

pub mod bit;
pub mod template;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let yaml = std::fs::read_to_string("sequence.yaml")?;
    let sequence: Sequence = serde_yaml::from_str(&yaml).unwrap();
    println!("{:?}", sequence);

    // No repeat is bugged for now
    while sequence.repeat {
        for step in &sequence.steps {
            let template: Box<dyn Template> = match step.template {
                Templates::Rainbow => Box::new(Rainbow {}),
                Templates::Hearts => Box::new(Heart {}),
                Templates::Text => Box::new(Text::new(step.text.clone().unwrap())),
                Templates::Crypto => Box::new(CryptoPrice::new(step.tickers.clone().unwrap())),
                Templates::HappyText => Box::new(crate::template::happy_text::HappyText::new(
                    step.text.clone().unwrap(),
                )),
                Templates::Meteo => Box::new(Meteo {}),
            };
            let message = template.render();
            let string_message = serde_json::to_string(&message)?;
            let address = "http://127.0.01:8000/".to_string() + &string_message;

            let _resp = client.post(&address).send().await?;

            tokio::time::sleep(tokio::time::Duration::from_secs(step.duration.into())).await;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Sequence {
    repeat: bool,
    steps: Vec<Step>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Step {
    template: Templates,
    duration: u16,
    text: Option<Vec<String>>,
    tickers: Option<Vec<CryptoTickers>>,
}

#[derive(Serialize, Deserialize, Debug)]
enum Templates {
    Rainbow,
    Hearts,
    Text,
    Crypto,
    HappyText,
    Meteo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CryptoTickers {
    Eth,
    Btc,
    Sol,
    Bnb,
}

impl Display for CryptoTickers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoTickers::Eth => write!(f, "ETH"),
            CryptoTickers::Btc => write!(f, "BTC"),
            CryptoTickers::Sol => write!(f, "SOL"),
            CryptoTickers::Bnb => write!(f, "BNB"),
        }
    }
}
