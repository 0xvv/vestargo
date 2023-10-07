use crate::bit::Message;

pub mod heart;
pub mod rainbow;
pub mod text;

// TODO generic template for text

pub trait Template {
    fn render(&self) -> Message;
}
