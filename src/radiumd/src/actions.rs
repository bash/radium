use std::error::Error;
use std::fmt;
use libradium::{Frontend, Entry, EntryId};
use radium_protocol::Message;
use radium_protocol::messages::{SetWatchMode, AddEntry, EntryAdded};
use super::connection::Connection;
use super::entry::EntryData;

#[derive(Debug)]
pub enum ActionError {
    NotACommand,
    Unimplemented
}

pub trait Action {
    fn process(self, conn: &mut Connection, frontend: &mut Frontend<EntryData>) -> Result<Message, ActionError>;
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
    fn process(self, conn: &mut Connection, _: &mut Frontend<EntryData>) -> Result<Message, ActionError> {
        conn.set_watch_mode(self.mode());
        Ok(Message::Ok)
    }
}

impl Action for AddEntry {
    fn process(self, _: &mut Connection, frontend: &mut Frontend<EntryData>) -> Result<Message, ActionError> {
        let id = EntryId::gen(self.timestamp());
        let entry = Entry::new(id, self.consume_data());

        // TODO: use result
        frontend.add_entry(entry);

        Ok(Message::EntryAdded(EntryAdded::new(id.timestamp().sec, id.id())))
    }
}

impl Action for Message {
    fn process(self, conn: &mut Connection, frontend: &mut Frontend<EntryData>) -> Result<Message, ActionError> {
        if !self.is_command() {
            return Err(ActionError::NotACommand);
        }

        match self {
            Message::Ping => Ok(Message::Pong),
            Message::SetWatchMode(msg) => msg.process(conn, frontend),
            Message::AddEntry(msg) => msg.process(conn, frontend),
            _ => Err(ActionError::Unimplemented)
        }
    }
}