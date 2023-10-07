use crate::bit::*;
use crate::template::Template;
pub struct Rainbow {}

impl Template for Rainbow {
    fn render(&self) -> Message {
        let mut message = Message::new_filled(Bit::Color(Color::Red));
        for j in 0..22 {
            message.rows[1].bits[j] = Bit::Color(Color::Orange);
            message.rows[2].bits[j] = Bit::Color(Color::Yellow);
            message.rows[3].bits[j] = Bit::Color(Color::Green);
            message.rows[4].bits[j] = Bit::Color(Color::Blue);
            message.rows[5].bits[j] = Bit::Color(Color::Violet);
        }
        message
    }
}
