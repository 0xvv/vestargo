use crate::template::Template;
use crate::bit::*;
pub struct Heart {}

impl Template for Heart {
    fn render(&self) -> Message {
        let mut message = Message::new_filled(Bit::Color(Color::White));
        message.rows[0].bits[2] = Bit::Color(Color::Red);
        message.rows[0].bits[6] = Bit::Color(Color::Red);

        message.rows[1].bits[1] = Bit::Color(Color::Red);
        message.rows[1].bits[2] = Bit::Color(Color::Red);
        message.rows[1].bits[3] = Bit::Color(Color::Red);
        message.rows[1].bits[5] = Bit::Color(Color::Red);
        message.rows[1].bits[6] = Bit::Color(Color::Red);
        message.rows[1].bits[7] = Bit::Color(Color::Red);

        message.rows[2].bits[1] = Bit::Color(Color::Red);
        message.rows[2].bits[2] = Bit::Color(Color::Red);
        message.rows[2].bits[3] = Bit::Color(Color::Red);
        message.rows[2].bits[4] = Bit::Color(Color::Red);
        message.rows[2].bits[5] = Bit::Color(Color::Red);
        message.rows[2].bits[6] = Bit::Color(Color::Red);
        message.rows[2].bits[7] = Bit::Color(Color::Red);

        message.rows[3].bits[2] = Bit::Color(Color::Red);
        message.rows[3].bits[3] = Bit::Color(Color::Red);
        message.rows[3].bits[4] = Bit::Color(Color::Red);
        message.rows[3].bits[5] = Bit::Color(Color::Red);
        message.rows[3].bits[6] = Bit::Color(Color::Red);

        message.rows[4].bits[3] = Bit::Color(Color::Red);
        message.rows[4].bits[4] = Bit::Color(Color::Red);
        message.rows[4].bits[5] = Bit::Color(Color::Red);

        message.rows[5].bits[4] = Bit::Color(Color::Red);


        message.rows[0].bits[15] = Bit::Color(Color::Red);
        message.rows[0].bits[19] = Bit::Color(Color::Red);

        message.rows[1].bits[14] = Bit::Color(Color::Red);
        message.rows[1].bits[15] = Bit::Color(Color::Red);
        message.rows[1].bits[16] = Bit::Color(Color::Red);
        message.rows[1].bits[18] = Bit::Color(Color::Red);
        message.rows[1].bits[19] = Bit::Color(Color::Red);
        message.rows[1].bits[20] = Bit::Color(Color::Red);

        message.rows[2].bits[14] = Bit::Color(Color::Red);
        message.rows[2].bits[15] = Bit::Color(Color::Red);
        message.rows[2].bits[16] = Bit::Color(Color::Red);
        message.rows[2].bits[17] = Bit::Color(Color::Red);
        message.rows[2].bits[18] = Bit::Color(Color::Red);
        message.rows[2].bits[19] = Bit::Color(Color::Red);
        message.rows[2].bits[20] = Bit::Color(Color::Red);

        message.rows[3].bits[15] = Bit::Color(Color::Red);
        message.rows[3].bits[16] = Bit::Color(Color::Red);
        message.rows[3].bits[17] = Bit::Color(Color::Red);
        message.rows[3].bits[18] = Bit::Color(Color::Red);
        message.rows[3].bits[19] = Bit::Color(Color::Red);

        message.rows[4].bits[16] = Bit::Color(Color::Red);
        message.rows[4].bits[17] = Bit::Color(Color::Red);
        message.rows[4].bits[18] = Bit::Color(Color::Red);

        message.rows[5].bits[17] = Bit::Color(Color::Red);

        message
    }
}