use crate::controls::Control;
use crate::{Pin, PinValue};
use st_data::control_values::ToggleSwitchValue;

pub struct ToggleSwitch {
    pin: Pin,
}

impl ToggleSwitch {
    pub fn new(pin: Pin) -> ToggleSwitch {
        ToggleSwitch { pin }
    }
}

impl Control<ToggleSwitchValue> for ToggleSwitch {
    fn read(&self) -> ToggleSwitchValue {
        /*
        match self.pin.read(device) {
            PinValue::Low => ToggleSwitchValue::Disabled,
            PinValue::High => ToggleSwitchValue::Enabled,
        }
        */
        ToggleSwitchValue::Disabled
    }
}
