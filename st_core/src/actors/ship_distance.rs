use crate::common::*;

const SHIP_DISTANCE_CALC_PERIOD: Duration = Duration::from_secs(2);
const SHIP_DISTANCE_PER_PERIOD: u32 = 278;
const DEFAULT_DISTANCE: u32 = 0;

pub struct ShipDistanceActor {
    distance: u32,
    next_update_at: Option<Instant>,
}

impl Default for ShipDistanceActor {
    fn default() -> ShipDistanceActor {
        ShipDistanceActor {
            distance: DEFAULT_DISTANCE,
            next_update_at: None,
        }
    }
}

impl Handles<GameStartedEvent> for ShipDistanceActor {
    fn handle(&mut self, _: GameStartedEvent, ctx: &mut Context) {
        self.distance = DEFAULT_DISTANCE;
        self.next_update_at = Some(ctx.now() + SHIP_DISTANCE_CALC_PERIOD);
    }
}

// Note that we will assume that we won't be stalling for more
// than the delay time. There are much bigger problems if it's
// taking us 2 seconds between tick events.
impl Handles<TickEvent> for ShipDistanceActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        if let Some(next_update_at) = self.next_update_at {
            if ctx.now() > next_update_at {
                self.distance += SHIP_DISTANCE_PER_PERIOD;
                ctx.send(ShipDistanceUpdatedEvent {
                    distance: self.distance,
                });
                self.next_update_at = Some(next_update_at + SHIP_DISTANCE_CALC_PERIOD);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn ms(ms: u32) -> Instant {
        Instant::from_millis(ms)
    }

    fn drain(ctx: &mut Context) -> Vec<Events> {
        let mut events = Vec::new();
        loop {
            if let Some(event) = ctx.dequeue() {
                events.push(event);
            } else {
                return events;
            }
        }
    }

    #[test]
    fn does_nothing_before_game_starts() {
        let tick = TickEvent {};

        let mut actor = ShipDistanceActor::default();
        let mut ctx = Context::new(ms(0));

        actor.handle(tick, &mut ctx);
        ctx.update_now(ms(2000));
        actor.handle(tick, &mut ctx);
        ctx.update_now(ms(4000));

        assert_eq!(drain(&mut ctx), vec![]);
    }

    #[test]
    fn offsets_first_update_after_game_start() {
        let game_started = GameStartedEvent { random_seed: 0 };
        let tick = TickEvent {};

        let mut actor = ShipDistanceActor::default();
        let mut ctx = Context::new(ms(50));

        actor.handle(game_started, &mut ctx);
        ctx.update_now(ms(2001));
        actor.handle(tick, &mut ctx);
        assert_eq!(drain(&mut ctx), vec![]);
    }

    #[test]
    fn sends_ship_update_as_necessary() {
        let tick = TickEvent {};

        let mut actor = ShipDistanceActor::default();
        let mut ctx = Context::new(ms(0));

        actor.handle(GameStartedEvent { random_seed: 0 }, &mut ctx);
        actor.handle(tick, &mut ctx);
        assert_eq!(ctx.dequeue(), None);

        ctx.update_now(ms(1900));
        actor.handle(tick, &mut ctx);
        assert_eq!(ctx.dequeue(), None);

        ctx.update_now(ms(2020));
        actor.handle(tick, &mut ctx);
        let update = ShipDistanceUpdatedEvent {
            distance: SHIP_DISTANCE_PER_PERIOD,
        };
        assert_eq!(drain(&mut ctx), vec![update.into()]);

        ctx.update_now(ms(4001));
        actor.handle(tick, &mut ctx);
        let update2 = ShipDistanceUpdatedEvent {
            distance: SHIP_DISTANCE_PER_PERIOD * 2,
        };
        assert_eq!(drain(&mut ctx), vec![update2.into()]);
    }
}
