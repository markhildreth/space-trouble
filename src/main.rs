#![no_std]
#![no_main]

extern crate panic_halt;

use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::TimerCounter;

#[entry]
fn main() -> ! {
    let mut core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();

    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    red_led.set_low().unwrap();

    let gclk0 = clocks.gclk0();
    let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
    let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
    timer.start(20.khz());

    let mut ms = 0;
    loop {
        if let Ok(_) = timer.wait() {
            ms += 1;
        }

        // 1 sec
        if ms == 20_000 {
            red_led.set_high().unwrap();
        }
    }
}
