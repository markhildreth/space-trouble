use st_data::time::*;

const SHIP_DISTANCE_CALC_DELAY: Duration = Duration::from_secs(2);
const SHIP_DISTANCE_PER_DELAY: u32 = 275;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum ShipDistanceResult {
    Noop,
    DistanceUpdated(u32),
}

pub(crate) struct ShipDistance {
    distance: u32,
    next_update_at: Instant,
}

impl ShipDistance {
    pub(crate) fn new() -> ShipDistance {
        ShipDistance {
            distance: 0,
            // TODO: A bit funky
            next_update_at: Instant::ZERO + SHIP_DISTANCE_CALC_DELAY,
        }
    }

    pub(crate) fn update(&mut self, now: Instant) -> ShipDistanceResult {
        // Note that we will assume that we won't be stalling for more
        // than the delay time. There are much bigger problems if it's
        // taking us 2 seconds to run this update.
        if now >= self.next_update_at {
            self.distance += SHIP_DISTANCE_PER_DELAY;
            self.next_update_at += SHIP_DISTANCE_CALC_DELAY;
            return ShipDistanceResult::DistanceUpdated(self.distance);
        }
        ShipDistanceResult::Noop
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn ms(ms: u32) -> Instant {
        Instant::from_millis(ms)
    }

    #[test]
    fn gives_noop_when_no_update_needed() {
        let mut distance = ShipDistance::new();
        let tenth_distance_ms: u32 = SHIP_DISTANCE_CALC_DELAY.as_millis() / 10;
        assert_eq!(
            distance.update(ms(tenth_distance_ms)),
            ShipDistanceResult::Noop
        );
        assert_eq!(
            distance.update(ms(2 * tenth_distance_ms)),
            ShipDistanceResult::Noop
        );
        assert_eq!(
            distance.update(ms(3 * tenth_distance_ms)),
            ShipDistanceResult::Noop
        );
        assert_eq!(
            distance.update(ms(4 * tenth_distance_ms)),
            ShipDistanceResult::Noop
        );
    }

    #[test]
    fn eventually_gives_distance_update() {
        let mut distance = ShipDistance::new();
        assert_eq!(distance.update(ms(1900)), ShipDistanceResult::Noop);
        assert_eq!(
            distance.update(ms(2_000)),
            ShipDistanceResult::DistanceUpdated(SHIP_DISTANCE_PER_DELAY)
        );
    }

    #[test]
    fn provides_multiple_distance_updates() {
        let mut distance = ShipDistance::new();
        assert_eq!(distance.update(ms(1900)), ShipDistanceResult::Noop);
        assert_eq!(
            distance.update(ms(3000)),
            ShipDistanceResult::DistanceUpdated(SHIP_DISTANCE_PER_DELAY)
        );
        assert_eq!(distance.update(ms(3500)), ShipDistanceResult::Noop);
        assert_eq!(
            distance.update(ms(4100)),
            ShipDistanceResult::DistanceUpdated(2 * SHIP_DISTANCE_PER_DELAY)
        );
    }
}
