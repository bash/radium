use std::error::Error;
use std::fmt;
use std::sync::mpsc::SendError;
use libradium::{Frontend, Entry, EntryId, Command};
use radium_protocol::Message;
use radium_protocol::messages::{SetWatchMode, AddEntry, EntryAdded, ErrorCode, ErrorMessage};
use super::connection::Connection;
use super::entry::EntryData;

#[derive(Debug)]
pub enum ActionError {
    NotACommand,
    Unimplemented,
    AddEntryError
}

impl From<SendError<Command<EntryData>>> for ActionError {
    fn from(_: SendError<Command<EntryData>>) -> Self {
        ActionError::AddEntryError
    }
}

impl Into<ErrorCode> for ActionError {
    fn into(self) -> ErrorCode {
        match self {
            ActionError::NotACommand => ErrorCode::InvalidAction,
            ActionError::Unimplemented => ErrorCode::ActionNotImplemented,
            ActionError::AddEntryError => ErrorCode::ActionProcessingError,
        }
    }
}

impl Into<Message> for ActionError {
    fn into(self) -> Message {
        Message::Error(ErrorMessage::new(self.into()))
    }
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
            &ActionError::Unimplemented => "Action is not implemented",
            &ActionError::AddEntryError => "Could not add entry"
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

        frontend.add_entry(entry)?;

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