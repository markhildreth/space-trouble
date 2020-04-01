#![deny(warnings)]
#![no_std]
#![no_main]

mod controls;
mod device;
mod lcd;
mod panels;

use core::panic::PanicInfo;
use embedded_hal::timer::CountDown;
use feather_m0::entry;
use st_core::actors::*;
use st_core::common::*;

const TICK: Duration = Duration::from_millis(1);

#[entry]
fn main() -> ! {
    let mut device = device::initialize_device();

    // Actors
    let mut panel = PanelActor::new(device.panel);
    let mut display = DisplayActor::new(device.lcd);
    let mut game_logic = GameLogicActor::new();

    // Context for Actors
    let mut ctx = Context {
        queue: EventsQueue::new(),
        now: Instant::from_millis(0),
    };

    ctx.queue.enqueue(StartGameEvent {}.into()).unwrap();
    loop {
        if device.timer.wait().is_ok() {
            ctx.now += TICK;
        }

        ctx.queue.enqueue(TickEvent {}.into()).unwrap();

        while let Some(event) = ctx.queue.dequeue() {
            match event {
                Events::Tick(ev) => {
                    game_logic.handle(ev, &mut ctx);
                    panel.handle(ev, &mut ctx);
                    display.handle(ev, &mut ctx);
                }
                Events::StartGame(ev) => game_logic.handle(ev, &mut ctx),
                Events::ActionPerformed(ev) => game_logic.handle(ev, &mut ctx),
                Events::NewDirective(ev) => display.handle(ev, &mut ctx),
                Events::HullHealthUpdated(ev) => display.handle(ev, &mut ctx),
                Events::ShipDistanceUpdated(ev) => display.handle(ev, &mut ctx),
                Events::DirectiveCompleted(ev) => display.handle(ev, &mut ctx),
            }
        }
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
