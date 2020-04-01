use crate::common::*;

pub trait Event: core::fmt::Debug {}

pub struct Context {
    pub now: Instant,
    pub queue: EventsQueue,
}

pub trait Handles<E: Event>: Sized {
    fn handle(&mut self, e: E, ctx: &mut Context);
}
