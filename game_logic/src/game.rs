use crate::{Directive, GameMessage, GameMessageProducer, ShipState};
use rand::Rng;

pub struct Game<'a> {
    producer: GameMessageProducer<'a>,
    ship_state: ShipState,
    ms: u32,
}

impl<'a> Game<'a> {
    pub fn new(producer: GameMessageProducer<'a>) -> Game {
        Game {
            producer,
            ms: 0,
            ship_state: ShipState::default(),
        }
    }

    pub fn update(&mut self, ms: u32, rng: &mut impl Rng) {
        if ms >= 2000 {
            self.generate_directive(ms, rng);
        }
    }

    fn generate_directive(&mut self, ms: u32, rng: &mut impl Rng) {
        let action = self.ship_state.generate_action(rng);
        match action {
            Ok(action) => {
                self.producer.enqueue(GameMessage::NewDirective(Directive {
                    action,
                    expiration: 0,
                }));
            }
            Err(_) => (),
        }
    }
}
