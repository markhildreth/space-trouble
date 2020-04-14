use crate::common::*;
use crate::device::Panel;

pub struct PanelActor<P>
where
    P: Panel,
{
    panel: P,
}

impl<P> PanelActor<P>
where
    P: Panel,
{
    pub fn new(panel: P) -> PanelActor<P> {
        PanelActor { panel }
    }
}

impl<P> Handles<InitializeGameEvent> for PanelActor<P>
where
    P: Panel,
{
    fn handle(&mut self, _: InitializeGameEvent, ctx: &mut Context) {
        self.panel
            .poll_all(ctx.now())
            .iter()
            .copied()
            .for_each(|action| ctx.send(ControlInitReportedEvent { action }));
    }
}

impl<P> Handles<TickEvent> for PanelActor<P>
where
    P: Panel,
{
    fn handle(&mut self, _: TickEvent, ctx: &mut Context) {
        self.panel
            .poll_changed(ctx.now())
            .iter()
            .copied()
            .for_each(|action| ctx.send(ActionPerformedEvent { action }));
    }
}
