use crate::bit::Message;

pub mod crypto_price;
pub mod heart;
pub mod rainbow;
pub mod text;
pub mod happy_text;

pub trait Template {
    fn render(&self) -> Message;
}
