use crate::common::*;

enum State {
    // The device is running, but we have not yet received the "InitGame"
    // message to start the process of starting up the game.
    Initializing,
    Playing,
}

pub struct GameStateActor {
    state: State,
}

impl Default for GameStateActor {
    fn default() -> GameStateActor {
        GameStateActor {
            state: State::Initializing,
        }
    }
}

impl Handles<ControlInitFinishedEvent> for GameStateActor {
    fn handle(&mut self, _: ControlInitFinishedEvent, ctx: &mut Context) {
        self.state = State::Playing;
        ctx.send(GameStartedEvent {});
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn ms(x: u32) -> Instant {
        Instant::from_millis(x)
    }

    #[test]
    fn responds_as_directed() {
        let mut actor = GameStateActor::default();
        let mut ctx = Context::new(ms(0));
        actor.handle(ControlInitFinishedEvent {}, &mut ctx);
        assert_eq!(ctx.dequeue().unwrap(), GameStartedEvent {}.into());
    }
}
