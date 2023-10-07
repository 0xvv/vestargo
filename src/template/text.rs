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
        Text { text }
    }
}

impl Template for Text {
    fn render(&self) -> Message {
        Message::from_text(&self.text)
    }
}
