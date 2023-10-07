use std::str::FromStr;
use crate::bit::*;
use crate::template::Template;
pub struct Text {
    pub text: Vec<String>,
}

impl Text {
    pub fn new(mut text: Vec<String>) -> Text {
        for line in &mut text {
            line.make_ascii_uppercase();
            if line.len() > 22 {
                line.truncate(22);
            }
        }
        Text {
            text
        }
    }
}

impl Template for Text {
    fn render(&self) -> Message {
        let mut message = Message::new();
        for (i, mut line) in self.text.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                message.rows[i].bits[j] = Bit::from_str(&c.to_string()).unwrap()
            }
        }

        message
    }
}
