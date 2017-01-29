#[derive(Debug)]
pub enum MessageType {
    Ping,
    Pong,
    Close,
    Add,
    Remove,
    Has,
    Subscribe,
    Push,
    Ok,
    Error
}

impl MessageType {
    pub fn from_value(value: u16) -> Option<Self> {
        match value {
            0 => Some(MessageType::Ping),
            1 => Some(MessageType::Pong),
            2 => Some(MessageType::Close),
            3 => Some(MessageType::Add),
            4 => Some(MessageType::Remove),
            5 => Some(MessageType::Has),
            6 => Some(MessageType::Subscribe),
            7 => Some(MessageType::Push),
            8 => Some(MessageType::Ok),
            9 => Some(MessageType::Error),
            _ => None
        }
    }
}