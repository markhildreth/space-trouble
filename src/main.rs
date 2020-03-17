#![no_std]
#![no_main]

extern crate panic_halt;

mod device;
mod game;
mod game_screen;
mod lcd;
mod messages;
mod queue;
mod states;
mod timing;

use crate::device::Device;
use crate::game::Game;
use crate::queue::OutgoingQueue;
use crate::states::GameState;
use feather_m0::entry;

#[entry]
fn main() -> ! {
    // Messages sent from the game server (the game server)
    let mut outgoing_queue = OutgoingQueue::new();
    let (out_producer, mut out_consumer) = outgoing_queue.split();

    // The game "server".
    let mut game = Game::new(out_producer);

    // The game "client"
    let mut device = Device::new();
    let mut state = GameState::new(&mut device);

    loop {
        device.update();

        state.update(&mut device);
        game.update(device.ms());

        loop {
            match out_consumer.dequeue() {
                Some(msg) => state.handle(device.ms(), msg),
                None => break,
            }
        }
    }
}
