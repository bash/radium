use super::connection::Connection;
use radium_protocol::Message;
use radium_protocol::messages::SetWatchMode;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ActionError {
    NotACommand,
    Unimplemented
}

pub trait Action {
    fn process(&self, conn: &mut Connection) -> Result<Message, ActionError>;
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ActionError {
    fn description(&self) -> &str {
        match self {
            &ActionError::NotACommand => "Action is not a command",
            &ActionError::Unimplemented => "Action is not implemented"
        }
    }
}

impl Action for SetWatchMode {
    fn process(&self, conn: &mut Connection) -> Result<Message, ActionError> {
        conn.set_watch_mode(self.mode());
        Ok(Message::Ok)
    }
}

impl Action for Message {
    fn process(&self, conn: &mut Connection) -> Result<Message, ActionError> {
        if !self.is_command() {
            return Err(ActionError::NotACommand);
        }

        match self {
            &Message::Ping => Ok(Message::Pong),
            &Message::SetWatchMode(ref msg) => msg.process(conn),
            _ => Err(ActionError::Unimplemented)
        }
    }
}