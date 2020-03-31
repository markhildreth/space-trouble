#![no_std]
mod client;
mod game_screen;
mod states;
mod strings;
mod timing;

use core::fmt::Write;
pub use game_screen::GameScreen;
use st_common::time::*;
use st_common::EventQueueProducer;

pub use client::Client;

pub trait Panel {
    fn update(&mut self, producer: &mut EventQueueProducer, now: Instant);
}

pub trait LCD: Sized + Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}
