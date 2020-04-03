use crate::common::*;
use core::fmt::Write;
use heapless::Vec;
use heapless::consts::*;

pub trait LCD: Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}

pub trait Panel {
    // Poll all of the panel's devices, and return all values for any
    // stateful devices (e.g., toggle switch as opposed to momentary buttons).
    fn poll_all(&mut self, now: Instant) -> Vec<Action, U8>;

    // Poll all of the panel's devices, and return all values for any
    // devices which have changed since the last poll.
    fn poll_changed(&mut self, now: Instant) -> Vec<Action, U8>;
}
