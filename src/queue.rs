use crate::messages::{ClientMessages, Messages};
use heapless::consts::*;
use heapless::spsc::{Producer, Queue};

pub type IncomingQueue = Queue<Messages, U4>;
pub type IncomingProducer<'a> = Producer<'a, Messages, U4>;

pub type OutgoingQueue = Queue<ClientMessages, U4>;
pub type OutgoingProducer<'a> = Producer<'a, ClientMessages, U4>;
