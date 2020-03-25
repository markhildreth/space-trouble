use crate::controls::Control;
use crate::Pin;

pub struct PushButton {
    pin: Pin,
}

impl PushButton {
    pub fn new(pin: Pin) -> PushButton {
        PushButton { pin }
    }
}

impl Control for PushButton {}
