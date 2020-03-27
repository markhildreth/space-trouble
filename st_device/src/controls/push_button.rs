use crate::controls::Control;
use crate::{Pin, PinValue};
use st_data::control_values::PushButtonValue;

pub struct PushButton<P: Pin> {
    pin: P,
}

impl<P: Pin> PushButton<P> {
    pub fn new(pin: P) -> PushButton<P> {
        PushButton { pin }
    }
}

impl<P: Pin> Control for PushButton<P> {
    type Value = PushButtonValue;

    fn read(&self) -> Self::Value {
        match self.pin.read() {
            PinValue::Low => PushButtonValue::NotPushed,
            PinValue::High => PushButtonValue::Pushed,
        }
    }
}
