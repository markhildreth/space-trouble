use crate::common::*;

pub trait Event: core::fmt::Debug {}

pub struct Context {
    now: Instant,
    queue: EventsQueue,
}

impl Context {
    pub fn new(queue: EventsQueue, now: Instant) -> Context {
        Context { now, queue }
    }

    pub fn now(&self) -> Instant {
        self.now
    }

    pub fn send<T: Into<Events>>(&mut self, ev: T) {
        self.queue.enqueue(ev.into()).unwrap();
    }

    pub fn dequeue(&mut self) -> Option<Events> {
        self.queue.dequeue()
    }

    pub fn update_now(&mut self, now: Instant) {
        self.now = now;
    }
}

pub trait Handles<E: Event>: Sized {
    fn handle(&mut self, e: E, ctx: &mut Context);
}
