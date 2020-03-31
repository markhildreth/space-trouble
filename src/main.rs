#![no_std]
#![no_main]

mod device;
mod lcd;
mod panels;

use core::panic::PanicInfo;
use embedded_hal::timer::CountDown;
use feather_m0::entry;
use st_client::Client;
use st_common::*;
use st_server::Game;

const TICK: Duration = Duration::from_millis(1);

#[entry]
fn main() -> ! {
    let mut device = device::initialize_device();

    let mut queue = EventQueue::new();
    let (mut producer, mut consumer) = queue.split();

    // The game "server".
    let mut game = Game::new();

    // The game "client"
    let mut client = Client::new();

    let mut now = Instant::from_millis(0);
    loop {
        if let Ok(_) = device.timer.wait() {
            now += TICK;
        }

        producer.enqueue(Event::Tick(TickEvent)).unwrap();

        while let Some(event) = consumer.dequeue() {
            match event {
                Event::ActionPerformed(action) => game.perform(now, action, &mut producer),
                Event::NewDirective(_)
                | Event::HullHealthUpdated(_)
                | Event::ShipDistanceUpdated(_)
                | Event::DirectiveCompleted => {
                    client.handle(
                        now,
                        event,
                        &mut producer,
                        &mut device.panel,
                        &mut device.lcd,
                    );
                }
                Event::Tick(_) => game.update(now, &mut producer),
            }
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
