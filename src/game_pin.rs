use embedded_hal::digital::v2::InputPin;

const DEBOUNCE_TIME: u32 = 5;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum PinResult {
    NoChange,
    Change(PinValue),
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum PinValue {
    Low,
    High,
}

pub struct GamePin {
    current_value: PinValue,
    debounce_finishes: Option<u32>,
}

impl GamePin {
    pub fn new(pin: &impl InputPin) -> GamePin {
        GamePin {
            current_value: pin.is_high().ok().unwrap().into(),
            debounce_finishes: None,
        }
    }

    pub fn update(&mut self, ms: u32, pin: &impl InputPin) -> PinResult {
        let pin_value = pin.is_high().ok().unwrap().into();

        if pin_value == self.current_value {
            return PinResult::NoChange;
        }

        match self.debounce_finishes {
            None => {
                self.debounce_finishes = Some(ms + DEBOUNCE_TIME);
                PinResult::NoChange
            }
            Some(finish) => {
                if ms > finish {
                    self.current_value = pin_value;
                    self.debounce_finishes = None;
                    PinResult::Change(self.current_value)
                } else {
                    PinResult::NoChange
                }
            }
        }
    }
}

impl From<bool> for PinValue {
    fn from(v: bool) -> PinValue {
        match v {
            false => PinValue::Low,
            true => PinValue::High,
        }
    }
}
