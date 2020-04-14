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

fn change_health(old: u8, delta: i8) -> u8 {
    let r = i16::from(old) + i16::from(delta);
    if r < 0 {
        0
    } else {
        r.try_into().unwrap_or(255)
    }
}

impl Handles<GameStartedEvent> for HullHealthActor {
    fn handle(&mut self, _: GameStartedEvent, _: &mut Context) {
        self.health = DEFAULT_HEALTH;
    }
}

impl Handles<UpdateHullHealthEvent> for HullHealthActor {
    fn handle(&mut self, ev: UpdateHullHealthEvent, ctx: &mut Context) {
        assert!(self.health <= DEFAULT_HEALTH);
        self.health = change_health(self.health, ev.delta);
        ctx.send(HullHealthUpdatedEvent {
            health: self.health,
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_change_health() {
        assert_eq!(10, change_health(10, 0));
        assert_eq!(5, change_health(10, -5));
        assert_eq!(0, change_health(10, -10));
        assert_eq!(0, change_health(10, -15));
        assert_eq!(255, change_health(255, 10));
    }
}
