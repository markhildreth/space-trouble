#![no_std]
mod device;

pub mod controls;
pub mod lcd;

pub use device::Device;

pub enum Pin {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,

    SCK,
    MOSI,
    MISO,

    RX0,
    RX1,

    SDA,
    SCL,

    D5,
    D6,
    D9,
    D10,
    D11,
    D12,
    D13,
}

enum PinValue {
    Low,
    High,
}

impl Pin {
    fn read(&self, _device: &Device) -> PinValue {
        // TODO
        PinValue::Low
    }
}
