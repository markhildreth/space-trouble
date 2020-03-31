#![deny(warnings)]
#![no_std]
#![no_main]

mod actors;
mod device;
mod lcd;
mod panels;

use crate::actors::*;
use core::panic::PanicInfo;
use embedded_hal::timer::CountDown;
use feather_m0::entry;
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

    // Actors
    let mut panel = PanelActor::new();
    let mut display = DisplayActor::new();

    // Context for Actors
    let mut ctx = Context {
        queue: EventQueue::new(),
        panel: device.panel,
        lcd: device.lcd,
        now: Instant::from_millis(0),
    };

    loop {
        if let Ok(_) = device.timer.wait() {
            ctx.now += TICK;
        }

        ctx.queue.enqueue(Event::Tick(TickEvent {})).unwrap();

        while let Some(event) = ctx.queue.dequeue() {
            match event {
                Event::ActionPerformed(ev) => game.handle(ctx.now, ev, &mut ctx.queue),
                Event::NewDirective(ev) => display.handle(ev, &mut ctx),
                Event::HullHealthUpdated(ev) => display.handle(ev, &mut ctx),
                Event::ShipDistanceUpdated(ev) => display.handle(ev, &mut ctx),
                Event::DirectiveCompleted(ev) => display.handle(ev, &mut ctx),
                Event::Tick(ev) => {
                    game.update(ctx.now, &mut ctx.queue);
                    panel.handle(ev, &mut ctx);
                    display.handle(ev, &mut ctx);
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
