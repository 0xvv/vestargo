use crate::bit::{Bit, Color};

mod bit;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut start_bit = 63_u8;

    loop {
        let message = bit::Message::new_filled(start_bit.into());

        let string_message = serde_json::to_string(&message)?;
        let address = "http://127.0.01:8000/".to_string() + &string_message;

        let resp = client.post(&address)
            .send()
            .await?;

        // wait 1 s
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        match start_bit {
            70 => start_bit = 63,
            _ => start_bit += 1,
        }
    }
    Ok(())
}
