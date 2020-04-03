use crate::common::*;
use core::fmt::Write;

pub trait LCD: Write {
    fn set_cursor_pos(&mut self, row: u8, col: u8);
}

pub trait Panel {
    type Iter: Iterator<Item = Action>;

    // Return any values that have changed since the last poll.
    fn poll(&mut self, now: Instant) -> Self::Iter;
}
