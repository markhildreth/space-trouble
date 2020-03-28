#![no_std]
#![no_main]

mod device;
mod lcd;
mod panels;

use core::panic::PanicInfo;
use embedded_hal::timer::CountDown;
use feather_m0::entry;
use st_client::{Client, ClientComponents};
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
    let components = ClientComponents {
        producer: client_msg_producer,
        panel: device.panel,
        lcd: device.lcd,
    };
    let mut client = Client::new(components);

    let mut now = Instant::from_millis(0);
    loop {
        if let Ok(_) = device.timer.wait() {
            now += TICK;
        }

        client.update(now);
        game.update(now);

        loop {
            match game_msg_consumer.dequeue() {
                Some(msg) => client.handle(now, msg),
                None => break,
            }
        }

        loop {
            match client_msg_consumer.dequeue() {
                Some(msg) => match msg {
                    ClientMessage::ActionPerformed(action) => {
                        game.perform(now, action);
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
