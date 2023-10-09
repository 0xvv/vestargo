use crate::bit::Message;

pub mod crypto_price;
pub mod happy_text;
pub mod heart;
pub mod meteo;
pub mod rainbow;
pub mod text;

pub trait Template {
    fn render(&self) -> Message;
}
