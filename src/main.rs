#![no_std]
#![no_main]

extern crate panic_halt;

use feather_m0::entry;

mod device;
// mod game_clock;
mod game_screen;
mod lcd;
mod messages;
mod states;

use crate::device::Device;
use crate::messages::{Action, Directive, Interface, Messages, Value};
use crate::states::GameState;
use core::convert::TryInto;
use embedded_hal::timer::CountDown;

#[entry]
fn main() -> ! {
    // TODO: Need a better way to build a timer. This one is not very accurate,
    // and also we can't turn it on/off to preserve power.
    let (device, mut timer) = Device::new();
    let mut state = GameState::new(device);

    let mut distance = 0;
    let mut hull = 100;
    let mut ms = 0;
    let directive_time = 10_000;

    loop {
        if let Ok(_) = timer.wait() {
            ms += 1;
        }

        state.update();

        // Test stuff
        if ms == 1_000 {
            state.handle(Messages::NewDirective(Directive {
                action: Action {
                    interface: Interface::Eigenthrottle,
                    value: Value::Enable,
                },
                time_ms: 20,
            }));
        }

        if ms > 1_000 && ms % 500 == 0 {
            let blocks = ((directive_time as u32 - (ms - 1_000)) * 20) / directive_time;
            state.handle(Messages::UpdateDirectiveTimeRemaining(
                blocks.try_into().unwrap(),
            ));
        }

        if ms % 1_000 == 0 {
            distance += 204;
            state.handle(Messages::UpdateDistance(distance));
        }

        if ms % 2_000 == 0 {
            hull -= 4;
            state.handle(Messages::UpdateHullHealth(hull));
        }
    }
}
