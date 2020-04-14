use crate::common::*;

#[derive(PartialEq)]
enum State {
    // We are waiting for the user to press a button to start the game.
    AwaitingInput,
    // We are waiting for the game to finish any pre-game setup.
    Initializing { start: Instant },
    Playing { distance_traveled: u32 },
    GameEnded,
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
            self.state = State::Initializing { start: ctx.now() };
            ctx.send(InitializeGameEvent {});
        }
    }
}

impl Handles<ControlInitFinishedEvent> for GameStateActor {
    fn handle(&mut self, _: ControlInitFinishedEvent, ctx: &mut Context) {
        if let State::Initializing { start } = self.state {
            self.state = State::Playing {
                distance_traveled: 0,
            };
            let elapsed = ctx.now() - start;
            // Note that currently, the millisecond resolution is not accurate or high
            // resolution enough to get different random seeds. Hopefully this will change
            // when I make the move to having to initialize controls on different panels
            // wirelessly, as the wireless communication might not take sub-ms time. Otherwise
            // I'll have to try out higher resolution timers, or alternate methods of randomness
            // (e.g., a static number of different seeds that loop.
            ctx.send(GameStartedEvent {
                random_seed: elapsed.as_millis(),
            });
        }
    }
}

impl Handles<ShipDistanceUpdatedEvent> for GameStateActor {
    fn handle(&mut self, ev: ShipDistanceUpdatedEvent, _: &mut Context) {
        if let State::Playing { .. } = self.state {
            self.state = State::Playing {
                distance_traveled: ev.distance,
            };
        }
    }
}

impl Handles<HullHealthUpdatedEvent> for GameStateActor {
    fn handle(&mut self, ev: HullHealthUpdatedEvent, ctx: &mut Context) {
        if ev.health > 0 {
            return;
        }

        if let State::Playing { distance_traveled } = self.state {
            self.state = State::GameEnded;
            ctx.send(GameEndedEvent { distance_traveled });
        }
    }
}
