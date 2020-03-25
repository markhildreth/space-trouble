#![no_std]
mod device;

pub mod controls;
pub mod lcd;

pub use device::Device;
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
    fn read(&self, device: &Device) -> PinValue {
        let result = match self {
            Pin::A0 => device.pin_a0.is_high(),
            Pin::A1 => device.pin_a1.is_high(),
            Pin::A2 => device.pin_a2.is_high(),
            Pin::A3 => device.pin_a3.is_high(),
            Pin::A4 => device.pin_a4.is_high(),
            Pin::A5 => device.pin_a5.is_high(),

            Pin::D5 => device.pin_d5.is_high(),
            Pin::D6 => device.pin_d6.is_high(),
            Pin::D9 => device.pin_d9.is_high(),
            Pin::D10 => device.pin_d10.is_high(),
            Pin::D11 => device.pin_d11.is_high(),
            Pin::D12 => device.pin_d12.is_high(),
        };

        match result.ok().unwrap() {
            false => PinValue::Low,
            true => PinValue::High,
        }
    }
}
