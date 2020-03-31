use super::Context;
use crate::common::*;

pub struct PanelActor {}

impl PanelActor {
    pub fn new() -> PanelActor {
        PanelActor {}
    }
}

impl Handler for PanelActor {
    type Context = Context;
}

impl Handles<TickEvent> for PanelActor {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        ctx.panel.update(ctx.now, &mut ctx.queue);
    }
}
