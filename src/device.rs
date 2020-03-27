use crate::lcd::{LCDImpl, LCD};
use crate::panels::Panel;
use feather_m0 as hal;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals, TC3};
use hal::prelude::*;
use hal::timer::TimerCounter;
use st_data::time::*;
use st_panels::controls::{Control, FourSwitch, PushButton, ToggleSwitch};

const LCD_I2C_ADDRESS: u8 = 0x27;

pub struct DeviceComponents {
    pub panel: Panel,
    pub lcd: LCD,
    pub timer: TimerCounter<TC3>,
}

pub fn initialize_device() -> DeviceComponents {
    let core = CorePeripherals::take().unwrap();
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let lcd = {
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
        LCD::new(LCDImpl::new_i2c(i2c, LCD_I2C_ADDRESS, lcd_delay))
    };

    let timer = {
        let gclk0 = clocks.gclk0();
        let timer_clock = clocks.tcc2_tc3(&gclk0).unwrap();
        let mut timer = TimerCounter::tc3_(&timer_clock, peripherals.TC3, &mut peripherals.PM);
        timer.start(1.khz());
        timer
    };

    let panel = {
        let d5 = pins.d5.into_pull_down_input(&mut pins.port);
        let d6 = pins.d6.into_pull_down_input(&mut pins.port);
        let d10 = pins.d10.into_pull_down_input(&mut pins.port);
        let d11 = pins.d11.into_pull_down_input(&mut pins.port);
        let d12 = pins.d12.into_pull_down_input(&mut pins.port);

        let a2 = pins.a2.into_pull_down_input(&mut pins.port);
        let a3 = pins.a3.into_pull_down_input(&mut pins.port);
        let a4 = pins.a4.into_pull_down_input(&mut pins.port);
        let a5 = pins.a5.into_pull_down_input(&mut pins.port);

        Panel {
            eigenthrottle: ToggleSwitch::new(d5).stateful(),
            gelatinous_darkbucket: ToggleSwitch::new(d6).stateful(),
            vent_hydrogen: PushButton::new(a2).stateful(),
            vent_water_vapor: PushButton::new(a3).stateful(),
            vent_waste: PushButton::new(a4).stateful(),
            vent_frustrations: PushButton::new(a5).stateful(),
            newtonian_fibermist: FourSwitch::new(d10, d11, d12)
                .debounce(Duration::from_millis(600)),
        }
    };

    DeviceComponents { panel, lcd, timer }
}
