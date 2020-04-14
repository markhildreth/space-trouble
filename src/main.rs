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
    let mut game_state = GameStateActor::default();
    let mut panel = PanelActor::new(device.panel);
    let mut display = DisplayActor::new(device.lcd);
    let mut directives = DirectivesActor::default();
    let mut hull_health = HullHealthActor::default();
    let mut ship_distance = ShipDistanceActor::default();

    // Context for Actors
    let mut now = Instant::from_millis(0);
    let mut ctx = Context::new(now);

    loop {
        ctx.send(TickEvent {});

        // I'm currently hand-crafting this routing of events. It would be nice
        // if this could be less manual.
        while let Some(event) = ctx.dequeue() {
            if device.timer.wait().is_ok() {
                now += TICK;
                ctx.update_now(now);
            }

            match event {
                Events::Tick(ev) => {
                    directives.handle(ev, &mut ctx);
                    panel.handle(ev, &mut ctx);
                    display.handle(ev, &mut ctx);
                    ship_distance.handle(ev, &mut ctx);
                }
                Events::InitializeGame(ev) => {
                    panel.handle(ev, &mut ctx);
                    display.handle(ev, &mut ctx);
                }
                Events::ControlInitReported(ev) => directives.handle(ev, &mut ctx),
                Events::ControlInitFinished(ev) => game_state.handle(ev, &mut ctx),
                Events::GameStarted(ev) => {
                    ship_distance.handle(ev, &mut ctx);
                    directives.handle(ev, &mut ctx);
                    display.handle(ev, &mut ctx);
                }
                Events::ActionPerformed(ev) => {
                    directives.handle(ev, &mut ctx);
                    game_state.handle(ev, &mut ctx);
                }
                Events::NewDirective(ev) => display.handle(ev, &mut ctx),
                Events::UpdateHullHealth(ev) => hull_health.handle(ev, &mut ctx),
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
