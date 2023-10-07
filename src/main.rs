pub mod bit;
pub mod template;

use crate::template::Template;
use crate::template::rainbow::Rainbow;
use crate::template::heart::Heart;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    loop {
        let template = Heart{};
        let message = template.render();
        let string_message = serde_json::to_string(&message)?;
        let address = "http://127.0.01:8000/".to_string() + &string_message;

        let _resp = client.post(&address)
            .send()
            .await?;

        // wait 1 s
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let template = Rainbow{};
        let message = template.render();
        let string_message = serde_json::to_string(&message)?;
        let address = "http://127.0.01:8000/".to_string() + &string_message;

        let _resp = client.post(&address)
            .send()
            .await?;

        // wait 1 s
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    Ok(())
}
