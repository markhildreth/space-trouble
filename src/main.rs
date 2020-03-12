#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::exception;
use feather_m0 as hal;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

static mut COUNTER: u32 = 0;

#[entry]
fn main() -> ! {
    let mut core = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    red_led.set_low().unwrap();

    unsafe {
        core.SYST.csr.write(0);
        core.SYST.set_reload(999);
        core.SYST.clear_current();
        core.SYST.csr.write(0b011);
    }

    loop {
        if unsafe { COUNTER == 1000 } {
            red_led.set_high().unwrap();
        }
        if unsafe { COUNTER == 2000 } {
            red_led.set_low().unwrap();
        }
        if unsafe { COUNTER == 3000 } {
            red_led.set_high().unwrap();
        }
        if unsafe { COUNTER == 4000 } {
            red_led.set_low().unwrap();
        }
        if unsafe { COUNTER == 5000 } {
            red_led.set_high().unwrap();
        }
        if unsafe { COUNTER == 6000 } {
            red_led.set_low().unwrap();
        }
    }
}

#[exception]
fn SysTick() {
    unsafe {
        COUNTER += 1;
    }
}
