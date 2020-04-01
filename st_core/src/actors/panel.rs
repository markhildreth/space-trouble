use crate::common::*;

pub trait Panel {
    fn update(&mut self, now: Instant, queue: &mut EventsQueue);
}

pub struct PanelActor<T: Panel> {
    panel: T,
}

impl<T: Panel> PanelActor<T> {
    pub fn new(panel: T) -> PanelActor<T> {
        PanelActor { panel }
    }
}

impl<T: Panel> Handles<TickEvent> for PanelActor<T> {
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        self.panel.update(ctx.now, &mut ctx.queue);
    }
}
