use crate::bit::Bit;
use crate::bit::Color;
use crate::bit::Message;
use crate::template::Template;
use num_traits::FromPrimitive;
use rand::Rng;

pub struct HappyText {
    pub text: Vec<String>,
}

impl HappyText {
    pub fn new(mut text: Vec<String>) -> HappyText {
        while text.len() > 4 {
            text.pop();
        }
        for line in &mut text {
            line.make_ascii_uppercase();
            *line = line.chars().take(20).collect();
        }
        HappyText { text }
    }
}

impl Template for HappyText {
    fn render(&self) -> Message {
        let mut t = self.text.clone();
        t.insert(0, "".parse().unwrap());
        for line in &mut t {
            String::insert(line, 0, " ".parse().unwrap());
        }
        let mut message = Message::from_text(t.as_slice());
        let mut rng = rand::thread_rng();

        // set the borders to a random color
        for i in 0..22 {
            message.rows[0].bits[i] = Bit::Color(Color::from_u8(rng.gen_range(63..=69)).unwrap());
            message.rows[5].bits[i] = Bit::Color(Color::from_u8(rng.gen_range(63..=69)).unwrap());
        }
        for i in 0..6 {
            message.rows[i].bits[0] = Bit::Color(Color::from_u8(rng.gen_range(63..=69)).unwrap());
            message.rows[i].bits[21] = Bit::Color(Color::from_u8(rng.gen_range(63..=69)).unwrap());
        }

        message
    }
}
