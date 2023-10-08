use crate::bit::*;
use crate::template::Template;

pub struct Text {
    pub text: Vec<String>,
}

impl Text {
    pub fn new(mut text: Vec<String>) -> Text {
        while text.len() > 6 {
            text.pop();
        }
        for line in &mut text {
            line.make_ascii_uppercase();
            *line = line.chars().take(22).collect();
        }
        Text { text }
    }
}

impl Template for Text {
    fn render(&self) -> Message {
        Message::from_text(&self.text)
    }
}
