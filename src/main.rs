#![deny(warnings)]
#![no_std]
#![no_main]

mod actors;
mod device;
mod lcd;
mod panels;

use crate::actors::{Context, PanelActor};
use core::panic::PanicInfo;
use embedded_hal::timer::CountDown;
use feather_m0::entry;
use st_client::Client;
use st_common::messaging::*;
use st_common::time::*;
use st_common::*;
use st_server::Game;

const TICK: Duration = Duration::from_millis(1);

#[entry]
fn main() -> ! {
    let mut device = device::initialize_device();

    // The game "server".
    let mut game = Game::new();

    // The game "client"
    let mut client = Client::new();

    // Actors
    let mut panel = PanelActor::new();

    // Context for Actors
    let mut ctx = Context {
        queue: EventQueue::new(),
        panel: device.panel,
    };

    // Event Loop
    let mut now = Instant::from_millis(0);
    loop {
        if let Ok(_) = device.timer.wait() {
            now += TICK;
        }

        ctx.queue.enqueue(Event::Tick(TickEvent { now })).unwrap();

        while let Some(event) = ctx.queue.dequeue() {
            match event {
                Event::ActionPerformed(action) => game.perform(now, action, &mut ctx.queue),
                Event::NewDirective(_)
                | Event::HullHealthUpdated(_)
                | Event::ShipDistanceUpdated(_)
                | Event::DirectiveCompleted => {
                    client.handle(now, event, &mut ctx.queue, &mut device.lcd);
                }
                Event::Tick(m) => {
                    game.update(now, &mut ctx.queue);
                    panel.handle(m, &mut ctx);
                }
            }
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
