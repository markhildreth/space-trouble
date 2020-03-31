use super::{Context, Tick};
use st_client::Panel;
use st_common::messaging::*;

struct Panels {}

impl Handler for Panels {
    type Context = Context;
}

impl Handles<Tick> for Panels {
    type Error = ();

    fn handle(&mut self, msg: Tick, ctx: &Context) -> Result<(), Self::Error> {
        ctx.panel.update(msg.now);
        Ok(())
    }
}
