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

#[entry]
fn main() -> ! {
    // TODO: Need a better way to build a timer. This one is not very accurate,
    // and also we can't turn it on/off to preserve power.
    let mut device = Device::new();
    let mut state = GameState::new(&mut device);

    let mut distance = 0;
    let mut hull = 100;
    let directive_time = 10_000;

    loop {
        device.update();
        state.update(&mut device);

        // Test stuff
        if device.ms() == 1_000 {
            state.handle(Messages::NewDirective(Directive {
                action: Action {
                    interface: Interface::Eigenthrottle,
                    value: Value::Enable,
                },
                time_ms: 20,
            }));
        }

        if device.ms() > 1_000 && device.ms() % 500 == 0 {
            let blocks = ((directive_time as u32 - (device.ms() - 1_000)) * 20) / directive_time;
            state.handle(Messages::UpdateDirectiveTimeRemaining(
                blocks.try_into().unwrap(),
            ));
        }

        if device.ms() % 1_000 == 0 {
            distance += 204;
            state.handle(Messages::UpdateDistance(distance));
        }

        if device.ms() % 2_000 == 0 {
            hull -= 4;
            state.handle(Messages::UpdateHullHealth(hull));
        }
    }
}
