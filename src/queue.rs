use game_logic::Action;
use heapless::consts::*;
use heapless::spsc::{Producer, Queue};

#[derive(Debug, PartialEq, Eq)]
pub enum ClientMessage {
    ActionPerformed(Action),
}
pub type ClientMessageQueue = Queue<ClientMessage, U4>;
pub type ClientMessageProducer<'a> = Producer<'a, ClientMessage, U4>;
