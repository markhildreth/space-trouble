use crate::common::*;

#[derive(PartialEq)]
enum State {
    // We are waiting for the user to press a button to start the game.
    AwaitingInput,
    // We are waiting for the game to finish any pre-game setup.
    Initializing,
    Playing,
}

pub struct GameStateActor {
    state: State,
}

impl Default for GameStateActor {
    fn default() -> GameStateActor {
        GameStateActor {
            state: State::AwaitingInput,
        }
    }
}

impl Handles<ActionPerformedEvent> for GameStateActor {
    fn handle(&mut self, _: ActionPerformedEvent, ctx: &mut Context) {
        if self.state == State::AwaitingInput {
            self.state = State::Initializing;
            ctx.send(InitializeGameEvent {});
        }
    }
}

impl Handles<ControlInitFinishedEvent> for GameStateActor {
    fn handle(&mut self, _: ControlInitFinishedEvent, ctx: &mut Context) {
        if self.state == State::Initializing {
            self.state = State::Playing;
            ctx.send(GameStartedEvent {});
        }
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
