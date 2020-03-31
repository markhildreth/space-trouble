use super::Context;
use st_common::messaging::*;
use st_common::*;

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
    fn handle(&mut self, msg: TickEvent, ctx: &mut Context) {
        ctx.panel.update(msg.now, &mut ctx.queue);
    }
}
