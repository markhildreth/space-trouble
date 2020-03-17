#![no_std]
#![no_main]

extern crate panic_halt;

use feather_m0::entry;

mod device;
mod game;
mod game_screen;
mod lcd;
mod messages;
mod states;
mod timing;

use crate::device::Device;
use crate::game::Game;
use crate::states::GameState;

#[entry]
fn main() -> ! {
    // TODO: Need a better way to build a timer. This one is not very accurate,
    // and also we can't turn it on/off to preserve power.
    let mut device = Device::new();
    let mut state = GameState::new(&mut device);

    let mut game = Game::new();

    loop {
        device.update();
        game.update(device.ms(), |msg| state.handle(&mut device, msg));
        state.update(&mut device);
    }
}
