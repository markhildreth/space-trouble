use crate::controls::Control;
use crate::Pin;

pub struct ToggleSwitch {
    pin: Pin,
}

impl ToggleSwitch {
    pub fn new(pin: Pin) -> ToggleSwitch {
        ToggleSwitch { pin }
    }
}

impl Control for ToggleSwitch {}
