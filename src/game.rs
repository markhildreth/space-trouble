use crate::game_clock::ElapsedMs;

pub enum MessageDestination {
    PlayerOne,
    PlayerTwo,
    PlayerThree,
}

pub enum Player {
    One,
    Two,
    Three,
}

pub struct Game {
    start_time: ElapsedMs,
    current_time: ElapsedMs,
    current_hull: u16,
    player_one_order: CurrentOrder,
    player_two_order: CurrentOrder,
    player_three_order: CurrentOrder,
}

struct Order {}

enum CurrentOrder {
    Order { order: Order },
    Awaiting { since: ElapsedMs },
}

pub struct MessagePayload {}

pub struct Message {
    destination: MessageDestination,
    payload: MessagePayload,
}

impl Message {
    pub fn new(_destination: MessageDestination, _payload: MessagePayload) {}
}

pub enum MessageGeneration {
    Message(Message),
    None,
}

impl Game {
    pub fn new(start_time: ElapsedMs) -> Game {
        Game {
            start_time,
            current_time: start_time,
            current_hull: 100,
            player_one_order: CurrentOrder::Awaiting { since: start_time },
            player_two_order: CurrentOrder::Awaiting { since: start_time },
            player_three_order: CurrentOrder::Awaiting { since: start_time },
        }
    }

    pub fn update(&mut self, current_time: ElapsedMs) {
        self.current_time = current_time;
    }

    // TODO: Seperate message generation and update
    pub fn generate_message(&mut self) -> MessageGeneration {
        if self.needs_new_order(&self.player_one_order) {
            let order = self.generate_order(Player::One);
            self.player_one_order = CurrentOrder::Order { order };
            let message = Message {
                destination: MessageDestination::PlayerOne,
                payload: MessagePayload {},
            };
            return MessageGeneration::Message(message);
        }

        MessageGeneration::None
    }

    fn needs_new_order(&self, current_order: &CurrentOrder) -> bool {
        match *current_order {
            CurrentOrder::Order { .. } => false,
            CurrentOrder::Awaiting { since } => self
                .current_time
                .has_elapsed_by(since, ElapsedMs::new(2000)),
        }
    }

    fn generate_order(&self, _player: Player) -> Order {
        Order {}
    }
}
