#![no_std]
pub mod controls;
pub mod lcd;

use embedded_hal::digital::v2::InputPin;

pub enum Pin {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,

    D5,
    D6,
    D9,
    D10,
    D11,
    D12,
}

enum PinValue {
    Low,
    High,
}

impl Pin {
    fn read(&self) -> PinValue {
        PinValue::Low
    }
}
