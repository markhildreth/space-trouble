use crate::common::*;

pub struct TimeSpan {
    finish: Instant,
    duration: Duration,
}

impl TimeSpan {
    pub fn new(finish: Instant, duration: Duration) -> Self {
        TimeSpan { finish, duration }
    }

    pub fn status(&self, now: Instant) -> SpanStatus {
        if now >= self.finish {
            return SpanStatus::Completed;
        }

        SpanStatus::Ongoing {
            remaining: self.finish - now,
            total: self.duration,
        }
    }
}

pub enum SpanStatus {
    Ongoing {
        remaining: Duration,
        total: Duration,
    },
    Completed,
}
