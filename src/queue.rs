use crate::messages::Messages;
use heapless::consts::*;
use heapless::spsc::{Producer, Queue};

pub type OutgoingQueue = Queue<Messages, U4>;
pub type OutgoingProducer<'a> = Producer<'a, Messages, U4>;
