use core::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq)]
pub struct Instant {
    ms: u32,
}

impl Instant {
    pub const ZERO: Instant = Instant::from_millis(0);

    pub const fn from_millis(ms: u32) -> Instant {
        Instant { ms }
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        Instant::from_millis(self.ms + other.ms)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        Instant::from_millis(self.ms - other.ms)
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        Duration::from_millis(self.ms - other.ms)
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

#[derive(PartialEq, Eq, PartialOrd, Copy, Clone, Debug)]
pub struct Duration {
    ms: u32,
}

impl Duration {
    pub const fn from_secs(s: u32) -> Duration {
        Duration { ms: s * 1000 }
    }

    pub const fn from_millis(ms: u32) -> Duration {
        Duration { ms }
    }

    pub fn as_millis(self) -> u32 {
        self.ms
    }
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, other: Duration) -> Duration {
        Duration::from_millis(self.ms + other.ms)
    }
}

impl Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        Duration::from_millis(self.ms - other.ms)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn i_ms(x: u32) -> Instant {
        Instant::from_millis(x)
    }

    fn d_ms(x: u32) -> Duration {
        Duration::from_millis(x)
    }

    #[test]
    fn test_instant_op_duration() {
        assert_eq!(i_ms(5) + d_ms(20), i_ms(25));
        assert_eq!(i_ms(25) - d_ms(5), i_ms(20));
    }

    #[test]
    fn test_instant_op_instant() {
        assert_eq!(i_ms(20) - i_ms(5), d_ms(15));
    }

    #[test]
    fn test_instant_add_assign() {
        let mut a = i_ms(20);
        a += d_ms(5);
        assert_eq!(a, i_ms(25));
    }

    #[test]
    fn test_duration_op_duration() {
        assert_eq!(d_ms(20) + d_ms(5), d_ms(25));
        assert_eq!(d_ms(25) - d_ms(5), d_ms(20));
    }

    #[test]
    fn test_duration_seconds() {
        assert_eq!(Duration::from_millis(5000), Duration::from_secs(5));
    }
}
