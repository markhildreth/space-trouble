#![no_std]
mod game_screen;
pub mod states;
mod strings;
mod timing;

use core::fmt::Write;
pub use game_screen::GameScreen;
use st_data::time::*;
use st_data::ClientMessageProducer;

pub trait Panel {
    fn update(&mut self, producer: &mut ClientMessageProducer, now: Instant);
}

pub trait LCD: Sized + Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}

mod client;
pub use client::Client;
