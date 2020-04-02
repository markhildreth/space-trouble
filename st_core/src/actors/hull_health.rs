use crate::common::*;
use core::convert::TryInto;

const DEFAULT_HEALTH: u8 = 100;

pub struct HullHealthActor {
    health: u8,
}

impl Default for HullHealthActor {
    fn default() -> HullHealthActor {
        HullHealthActor {
            health: DEFAULT_HEALTH,
        }
    }
}

impl Handles<UpdateHullHealthEvent> for HullHealthActor {
    fn handle(&mut self, ev: UpdateHullHealthEvent, ctx: &mut Context) {
        assert!(self.health <= DEFAULT_HEALTH);
        self.health = ((self.health as i8) + ev.delta).try_into().unwrap();
        ctx.send(HullHealthUpdatedEvent {
            health: self.health,
        });
    }
}
