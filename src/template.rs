use crate::bit::Message;

pub mod rainbow;
pub mod heart;

pub trait Template {
    fn render(&self) -> Message;
}