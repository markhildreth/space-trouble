#![no_std]
mod game_screen;
pub mod states;
mod strings;
mod timing;

pub use game_screen::GameScreen;
use st_data::ClientMessageProducer;

pub trait Panel {
    fn update(&mut self, producer: &mut ClientMessageProducer, ms: u32);
}
