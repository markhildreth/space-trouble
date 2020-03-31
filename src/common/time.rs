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

impl core::ops::Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        Instant::from_millis(self.ms + other.ms)
    }
}

impl core::ops::Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        Duration::from_millis(self.ms - other.ms)
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

impl core::ops::AddAssign<Duration> for Instant {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl core::ops::Sub<Duration> for Duration {
    type Output = Duration;

    fn sub(self, other: Duration) -> Duration {
        Duration::from_millis(self.ms - other.ms)
    }
}
