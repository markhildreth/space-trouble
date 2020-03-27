const SHIP_DISTANCE_CALC_DELAY: u32 = 2_000;
const SHIP_DISTANCE_PER_DELAY: u32 = 275;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum ShipDistanceResult {
    Noop,
    DistanceUpdated(u32),
}

pub(crate) struct ShipDistance {
    distance: u32,
    next_update: u32,
}

impl ShipDistance {
    pub(crate) fn new() -> ShipDistance {
        ShipDistance {
            distance: 0,
            next_update: 0 + SHIP_DISTANCE_CALC_DELAY,
        }
    }

    pub(crate) fn update(&mut self, ms: u32) -> ShipDistanceResult {
        // Note that we will assume that we won't be stalling for more
        // than the delay time. There are much bigger problems if it's
        // taking us 2 seconds to run this update.
        if ms >= self.next_update {
            self.distance += SHIP_DISTANCE_PER_DELAY;
            self.next_update += SHIP_DISTANCE_CALC_DELAY;
            return ShipDistanceResult::DistanceUpdated(self.distance);
        }
        ShipDistanceResult::Noop
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gives_noop_when_no_update_needed() {
        let mut distance = ShipDistance::new();
        let tenth_distance = SHIP_DISTANCE_CALC_DELAY / 10;
        assert_eq!(distance.update(tenth_distance), ShipDistanceResult::Noop);
        assert_eq!(
            distance.update(2 * tenth_distance),
            ShipDistanceResult::Noop
        );
        assert_eq!(
            distance.update(3 * tenth_distance),
            ShipDistanceResult::Noop
        );
        assert_eq!(
            distance.update(4 * tenth_distance),
            ShipDistanceResult::Noop
        );
    }

    #[test]
    fn eventually_gives_distance_update() {
        let mut distance = ShipDistance::new();
        assert_eq!(distance.update(1900), ShipDistanceResult::Noop);
        assert_eq!(
            distance.update(2_000),
            ShipDistanceResult::DistanceUpdated(SHIP_DISTANCE_PER_DELAY)
        );
    }

    #[test]
    fn provides_multiple_distance_updates() {
        let mut distance = ShipDistance::new();
        assert_eq!(distance.update(1900), ShipDistanceResult::Noop);
        assert_eq!(
            distance.update(3000),
            ShipDistanceResult::DistanceUpdated(SHIP_DISTANCE_PER_DELAY)
        );
        assert_eq!(distance.update(3500), ShipDistanceResult::Noop);
        assert_eq!(
            distance.update(4100),
            ShipDistanceResult::DistanceUpdated(2 * SHIP_DISTANCE_PER_DELAY)
        );
    }
}
