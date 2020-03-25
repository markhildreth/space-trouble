use crate::controls::Control;
use crate::Pin;

pub struct FourSwitch {
    pin_one: Pin,
    pin_two: Pin,
    pin_three: Pin,
}

impl FourSwitch {
    pub fn new(pin_one: Pin, pin_two: Pin, pin_three: Pin) -> FourSwitch {
        FourSwitch {
            pin_one,
            pin_two,
            pin_three,
        }
    }
}

impl Control for FourSwitch {}
