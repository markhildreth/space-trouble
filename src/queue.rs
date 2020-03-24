use heapless::consts::*;
use heapless::spsc::{Producer, Queue};
use st_data::Action;

#[derive(Debug, PartialEq, Eq)]
pub enum ClientMessage {
    ActionPerformed(Action),
}
pub type ClientMessageQueue = Queue<ClientMessage, U4>;
pub type ClientMessageProducer<'a> = Producer<'a, ClientMessage, U4>;
