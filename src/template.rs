use crate::bit::Message;

pub mod heart;
pub mod rainbow;

// TODO generic template for text

pub trait Template {
    fn render(&self) -> Message;
}
