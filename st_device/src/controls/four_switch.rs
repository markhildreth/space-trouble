use crate::controls::Control;
use crate::{Pin, PinValue};
use st_data::control_values::FourSwitchValue;

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

impl Control<FourSwitchValue> for FourSwitch {
    fn read(&self) -> FourSwitchValue {
        /*
        let one = self.pin_one.read(device);
        let two = self.pin_two.read(device);
        let three = self.pin_three.read(device);

        match (one, two, three) {
            (PinValue::Low, PinValue::Low, PinValue::Low) => FourSwitchValue::Zero,
            (PinValue::High, PinValue::Low, PinValue::Low) => FourSwitchValue::One,
            (PinValue::Low, PinValue::High, PinValue::Low) => FourSwitchValue::Two,
            (PinValue::Low, PinValue::Low, PinValue::High) => FourSwitchValue::Three,
            _ => unreachable!(),
        }
        */
        FourSwitchValue::Zero
    }
}
