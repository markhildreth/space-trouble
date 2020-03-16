#![no_std]
#![no_main]

extern crate panic_halt;

use feather_m0::entry;

mod device;
mod game_screen;
mod lcd;
mod messages;
mod states;
mod timing;

use crate::device::Device;
use crate::messages::{Action, Directive, Interface, Messages, Value};
use crate::states::GameState;

#[entry]
fn main() -> ! {
    // TODO: Need a better way to build a timer. This one is not very accurate,
    // and also we can't turn it on/off to preserve power.
    let mut device = Device::new();
    let mut state = GameState::new(&mut device);

    let mut distance = 0;
    let mut hull = 100;

    loop {
        device.update();
        state.update(&mut device);

        // Test stuff
        if device.ms() % 1_000 == 0 {
            distance += 204;
            state.handle(&mut device, Messages::UpdateDistance(distance));
        }

        if device.ms() % 18_000 == 0 {
            hull -= 4;
            state.handle(&mut device, Messages::UpdateHullHealth(hull));
        }

        if device.ms() == 1_000 {
            state.handle(
                &mut device,
                Messages::NewDirective(Directive {
                    action: Action {
                        interface: Interface::Eigenthrottle,
                        value: Value::Enable,
                    },
                    time_ms: 10_000,
                }),
            );
        }

        if device.ms() == 6_000 {
            state.handle(&mut device, Messages::CompleteDirective);
        }

        if device.ms() == 8_000 {
            state.handle(
                &mut device,
                Messages::NewDirective(Directive {
                    action: Action {
                        interface: Interface::Eigenthrottle,
                        value: Value::Enable,
                    },
                    time_ms: 10_000,
                }),
            );
        }
    }
}
