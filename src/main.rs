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

#[entry]
fn main() -> ! {
    let device = Device::new();
    let mut state = GameState::new(device);

    let mut distance = 0;
    let mut hull = 100;
    let mut ticks = 0;

    loop {
        state.tick();
        ticks += 1;
        if ticks % 200_000 == 0 {
            distance += 204;
            state.handle(Messages::UpdateDistance(distance));
        }

        if ticks % 500_000 == 0 {
            hull -= 4;
            state.handle(Messages::UpdateHullHealth(hull));
        }

        if ticks == 100_000 {
            state.handle(Messages::NewDirective(Directive {
                action: Action {
                    interface: Interface::Eigenthrottle,
                    value: Value::Enable,
                },
                time_ms: 10_000,
            }));
        }
    }
}
