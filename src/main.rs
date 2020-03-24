#![no_std]
#![no_main]

mod data;
mod game_screen;
mod queue;
mod states;
mod timing;

use crate::queue::{ClientMessage, ClientMessageQueue};
use crate::states::GameState;
use core::panic::PanicInfo;
use feather_m0::entry;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use st_device::Device;
use st_logic::{Game, GameMessageQueue};

#[entry]
fn main() -> ! {
    // Messages coming from the game server
    let mut game_msg_queue = GameMessageQueue::new();
    let (game_msg_producer, mut game_msg_consumer) = game_msg_queue.split();

    // Messages coming from the client
    let mut client_msg_queue = ClientMessageQueue::new();
    let (client_msg_producer, mut client_msg_consumer) = client_msg_queue.split();

    // The game "server".
    let mut game = Game::new(game_msg_producer);

    // The game "client"
    let mut device = Device::new();
    let mut state = GameState::new(client_msg_producer, &mut device);

    let mut rng = SmallRng::seed_from_u64(0x12345678);

    loop {
        device.update();

        state.update(&mut device);
        game.update(device.ms(), &mut rng);

        loop {
            match game_msg_consumer.dequeue() {
                Some(msg) => state.handle(device.ms(), msg),
                None => break,
            }
        }

        loop {
            match client_msg_consumer.dequeue() {
                Some(msg) => match msg {
                    ClientMessage::ActionPerformed(action) => {
                        game.perform(device.ms(), action);
                    }
                },
                None => break,
            }
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
