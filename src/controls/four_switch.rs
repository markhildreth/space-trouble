use super::{Control, Pin, PinValue};
use st_core::common::*;

pub struct FourSwitch<P1, P2, P3>
where
    P1: Pin,
    P2: Pin,
    P3: Pin,
{
    pin_one: P1,
    pin_two: P2,
    pin_three: P3,
}

impl<P1, P2, P3> FourSwitch<P1, P2, P3>
where
    P1: Pin,
    P2: Pin,
    P3: Pin,
{
    pub fn new(pin_one: P1, pin_two: P2, pin_three: P3) -> FourSwitch<P1, P2, P3> {
        FourSwitch {
            pin_one,
            pin_two,
            pin_three,
        }
    }
}

impl<P1, P2, P3> Control for FourSwitch<P1, P2, P3>
where
    P1: Pin,
    P2: Pin,
    P3: Pin,
{
    type Value = FourSwitchValue;

    fn read(&self) -> Self::Value {
        let one = self.pin_one.read();
        let two = self.pin_two.read();
        let three = self.pin_three.read();

        match (one, two, three) {
            (PinValue::High, PinValue::Low, PinValue::Low) => FourSwitchValue::One,
            (PinValue::Low, PinValue::High, PinValue::Low) => FourSwitchValue::Two,
            (PinValue::Low, PinValue::Low, PinValue::High) => FourSwitchValue::Three,
            _ => FourSwitchValue::Zero,
        }
    }
}
