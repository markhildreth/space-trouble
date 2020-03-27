#![no_std]
pub mod controls;
use embedded_hal::digital::v2::InputPin;

pub trait Pin {
    fn read(&self) -> PinValue;
}

pub enum PinValue {
    Low,
    High,
}

impl<T: InputPin> Pin for T {
    fn read(&self) -> PinValue {
        match self.is_high().ok().unwrap() {
            false => PinValue::Low,
            true => PinValue::High,
        }
    }
}