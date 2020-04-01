use crate::common::*;

pub struct TimeSpan {
    start: Instant,
    duration: Duration,
}

impl TimeSpan {
    pub fn new(start: Instant, duration: Duration) -> Self {
        TimeSpan { start, duration }
    }

    pub fn status(&self, now: Instant) -> SpanStatus {
        let completed = now - self.start;
        if completed >= self.duration {
            return SpanStatus::Completed;
        }

        SpanStatus::Ongoing {
            remaining: self.duration - completed,
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
