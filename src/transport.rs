use crate::game::Message;

trait Transport {
    fn deliver(msg: Message);
}
