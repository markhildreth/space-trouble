#![no_std]
#![no_main]

mod device;
mod lcd;
mod panels;

use core::panic::PanicInfo;
use embedded_hal::timer::CountDown;
use feather_m0::entry;
use st_client::states::GameState;
use st_data::*;
use st_server::Game;

const TICK: Duration = Duration::from_millis(1);

#[entry]
fn main() -> ! {
    let mut device = device::initialize_device();

    // Messages coming from the game server
    let mut game_msg_queue = GameMessageQueue::new();
    let (game_msg_producer, mut game_msg_consumer) = game_msg_queue.split();

    // Messages coming from the client
    let mut client_msg_queue = ClientMessageQueue::new();
    let (client_msg_producer, mut client_msg_consumer) = client_msg_queue.split();

    // The game "server".
    let mut game = Game::new(game_msg_producer);

    // The game "client"
    let mut state = GameState::new(client_msg_producer, device.panel, device.lcd);

    let mut ms = Instant::from_millis(0);
    loop {
        if let Ok(_) = device.timer.wait() {
            ms += TICK;
        }

        state.update(ms);
        game.update(ms);

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
