use futures::executor::block_on;
use serde_json::Value;

use crate::bit::*;
use crate::template::Template;

pub struct Meteo {}

impl Template for Meteo {
    fn render(&self) -> Message {
        let json = block_on(get_data());
        let times = json["daily"]["time"].as_array().unwrap();
        let temp_max = json["daily"]["temperature_2m_max"].as_array().unwrap();
        let temp_min = json["daily"]["temperature_2m_min"].as_array().unwrap();
        let precip = json["daily"]["precipitation_probability_max"]
            .as_array()
            .unwrap();

        let mut text = vec!["".to_string(); 6];
        text[0] = "DATE   MIN   MAX  RAIN".to_string();
        for i in 0..5 {
            let date = times[i].as_str().unwrap();
            let date = &date[5..10];
            let temp_max = temp_max[i].as_f64().unwrap();
            let temp_min = temp_min[i].as_f64().unwrap();
            let precip = precip[i].as_f64().unwrap();
            text[i + 1] = format!(
                "{}  {:>2.0}°C  {:>2.0}°C  {:>2.0}%",
                date, temp_min, temp_max, precip
            );
        }

        let mut message = Message::from_text(&text);

        message.rows[0].bits[10] = Bit::Color(Color::White);
        message.rows[0].bits[16] = Bit::Color(Color::Red);

        message
    }
}

async fn get_data() -> Value {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=48.8534&longitude=2.3488&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&timezone=Europe%2FBerlin";
    let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    serde_json::from_str(&resp).unwrap()
}
