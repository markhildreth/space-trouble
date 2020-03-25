use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::gpio::*;
use hal::pac::{CorePeripherals, Peripherals, TC3};
use hal::prelude::*;
use hal::timer::TimerCounter;

use crate::lcd::LCD;

const LCD_I2C_ADDRESS: u8 = 0x27;

pub struct Device {
    pub led_pin: Pa17<Output<OpenDrain>>,

    pub pin_d5: Pa15<Input<PullDown>>,
    pub pin_d6: Pa20<Input<PullDown>>,
    pub pin_d9: Pa7<Input<PullDown>>,
    pub pin_d10: Pa18<Input<PullDown>>,
    pub pin_d11: Pa16<Input<PullDown>>,
    pub pin_d12: Pa19<Input<PullDown>>,

    pub pin_a0: Pa2<Input<PullDown>>,
    pub pin_a1: Pb8<Input<PullDown>>,
    pub pin_a2: Pb9<Input<PullDown>>,
    pub pin_a3: Pa4<Input<PullDown>>,
    pub pin_a4: Pa5<Input<PullDown>>,
    pub pin_a5: Pb2<Input<PullDown>>,
    pub lcd: LCD,
    pub timer: TimerCounter<TC3>,
    ms: u32,
}

impl Device {
    pub fn new() -> Device {
        let core = CorePeripherals::take().unwrap();
        let mut peripherals = Peripherals::take().unwrap();
        let mut clocks = GenericClockController::with_internal_32kosc(
            peripherals.GCLK,
            &mut peripherals.PM,
            &mut peripherals.SYSCTRL,
            &mut peripherals.NVMCTRL,
        );
        let mut pins = hal::Pins::new(peripherals.PORT);

        let mut led_pin = pins.d13.into_open_drain_output(&mut pins.port);
        led_pin.set_low().unwrap();

        let pin_d5 = pins.d5.into_pull_down_input(&mut pins.port);
        let pin_d6 = pins.d6.into_pull_down_input(&mut pins.port);
        let pin_d9 = pins.d9.into_pull_down_input(&mut pins.port);
        let pin_d10 = pins.d10.into_pull_down_input(&mut pins.port);
        let pin_d11 = pins.d11.into_pull_down_input(&mut pins.port);
        let pin_d12 = pins.d12.into_pull_down_input(&mut pins.port);

        let pin_a0 = pins.a0.into_pull_down_input(&mut pins.port);
        let pin_a1 = pins.a1.into_pull_down_input(&mut pins.port);
        let pin_a2 = pins.a2.into_pull_down_input(&mut pins.port);
        let pin_a3 = pins.a3.into_pull_down_input(&mut pins.port);
        let pin_a4 = pins.a4.into_pull_down_input(&mut pins.port);
        let pin_a5 = pins.a5.into_pull_down_input(&mut pins.port);

        let i2c = hal::i2c_master(
            &mut clocks,
            270.khz(),
            peripherals.SERCOM3,
            &mut peripherals.PM,
            pins.sda,
            pins.scl,
            &mut pins.port,
        );

        let lcd_delay = Delay::new(core.SYST, &mut clocks);
        let lcd = LCD::new_i2c(i2c, LCD_I2C_ADDRESS, lcd_delay);

        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
        timer.start(1.khz());

        Device {
            led_pin,
            pin_d5,
            pin_d6,
            pin_d9,
            pin_d10,
            pin_d11,
            pin_d12,
            pin_a0,
            pin_a1,
            pin_a2,
            pin_a3,
            pin_a4,
            pin_a5,
            lcd,
            timer,
            ms: 0,
        }
    }

    pub fn ms(&self) -> u32 {
        return self.ms;
    }

    pub fn update(&mut self) {
        if let Ok(_) = self.timer.wait() {
            self.ms += 1;
        }
    }
}
