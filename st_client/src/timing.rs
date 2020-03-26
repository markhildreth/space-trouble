pub struct TimeSpan {
    start_time_ms: u32,
    duration_ms: u32,
}

impl TimeSpan {
    pub fn new(start_time_ms: u32, duration_ms: u32) -> Self {
        TimeSpan {
            start_time_ms,
            duration_ms,
        }
    }

    pub fn status(&self, current_ms: u32) -> SpanStatus {
        let completed_ms = current_ms - self.start_time_ms;
        if completed_ms >= self.duration_ms {
            return SpanStatus::Completed;
        }

        SpanStatus::Ongoing {
            remaining_ms: self.duration_ms - completed_ms,
            total_ms: self.duration_ms,
        }
    }
}

pub enum SpanStatus {
    Ongoing { remaining_ms: u32, total_ms: u32 },
    Completed,
}
