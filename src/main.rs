#![no_std]
#![no_main]

mod panels;

use core::panic::PanicInfo;
use feather_m0::entry;
use panels::Panel;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use st_client::states::GameState;
use st_data::{ClientMessage, ClientMessageQueue, GameMessageQueue};
use st_device::Device;
use st_logic::Game;

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
    let device = Device::new();
    let panel = Panel::default();
    let mut state = GameState::new(client_msg_producer, panel, device.lcd);

    let mut rng = SmallRng::seed_from_u64(0x12345678);

    let ms = 0;
    loop {
        state.update(ms);
        game.update(ms, &mut rng);

        loop {
            match game_msg_consumer.dequeue() {
                Some(msg) => state.handle(ms, msg),
                None => break,
            }
        }

        loop {
            match client_msg_consumer.dequeue() {
                Some(msg) => match msg {
                    ClientMessage::ActionPerformed(action) => {
                        game.perform(ms, action);
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
