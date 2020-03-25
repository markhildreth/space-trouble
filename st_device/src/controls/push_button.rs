use crate::controls::Control;
use crate::Device;
use crate::{Pin, PinValue};
use st_data::control_values::PushButtonValue;

pub struct PushButton {
    pin: Pin,
}

impl PushButton {
    pub fn new(pin: Pin) -> PushButton {
        PushButton { pin }
    }
}

impl Control<PushButtonValue> for PushButton {
    fn read(&self, device: &Device) -> PushButtonValue {
        match self.pin.read(device) {
            PinValue::Low => PushButtonValue::NotPushed,
            PinValue::High => PushButtonValue::Pushed,
        }
    }
}
