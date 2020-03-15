#![no_std]
#![no_main]

extern crate panic_halt;

use feather_m0::entry;

mod device;
// mod game;
// mod game_clock;
mod game_screen;
mod lcd;
mod states;
// mod transport;

use device::Device;
use states::GameState;

#[entry]
fn main() -> ! {
    let device = Device::new();
    let mut state = GameState::new(device);

    loop {
        state.tick();
    }
}
