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
use crate::queue::{IncomingQueue, OutgoingQueue};
use crate::states::GameState;
use feather_m0::entry;

#[entry]
fn main() -> ! {
    // Messages sent from the game "client" -> "server"
    let mut outgoing_queue = OutgoingQueue::new();
    let (out_producer, mut out_consumer) = outgoing_queue.split();

    // Messages sent from the game "server" -> "client"
    let mut incoming_queue = IncomingQueue::new();
    let (in_producer, mut in_consumer) = incoming_queue.split();

    // The game "server".
    let mut game = Game::new(in_producer);

    // The game "client"
    let mut device = Device::new();
    let mut state = GameState::new(out_producer, &mut device);

    loop {
        device.update();

        state.update(&mut device);
        game.update(device.ms());

        loop {
            match in_consumer.dequeue() {
                Some(msg) => state.handle(device.ms(), msg),
                None => break,
            }
        }

        loop {
            match out_consumer.dequeue() {
                Some(msg) => game.handle(device.ms(), msg),
                None => break,
            }
        }
    }
}
