#![no_std]
mod game_screen;
pub mod states;
mod strings;
mod timing;

pub use game_screen::GameScreen;
use st_data::ClientMessageProducer;
use st_device::Device;

pub trait Panel {
    fn update(&mut self, producer: &mut ClientMessageProducer, device: &Device);
}
