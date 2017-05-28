use std::error::Error;
use std::fmt;
use std::sync::mpsc::SendError;
use libradium::{Frontend, Entry, EntryId, Command};
use radium_protocol::Message;
use radium_protocol::messages::{SetWatchMode, AddEntry, EntryAdded, RemoveEntry, ErrorCode, ErrorMessage};
use super::connection::Connection;
use super::entry::EntryData;

#[derive(Debug)]
pub enum ActionError {
    NotACommand,
    Unimplemented,
    FrontendError,
}

pub type ActionResult = Result<Message, ActionError>;

pub trait Action {
    fn process(self, conn: &mut Connection, frontend: &mut Frontend<EntryData>) -> ActionResult;
}

impl From<SendError<Command<EntryData>>> for ActionError {
    fn from(_: SendError<Command<EntryData>>) -> Self {
        ActionError::FrontendError
    }
}

impl Into<ErrorCode> for ActionError {
    fn into(self) -> ErrorCode {
        match self {
            ActionError::NotACommand => ErrorCode::InvalidAction,
            ActionError::Unimplemented => ErrorCode::ActionNotImplemented,
            ActionError::FrontendError => ErrorCode::ActionProcessingError,
        }
    }
}

impl Into<Message> for ActionError {
    fn into(self) -> Message {
        Message::Error(ErrorMessage::new(self.into()))
    }
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
            &ActionError::FrontendError => "Unable to communicate with frontend"
        }
    }
}

impl Action for SetWatchMode {
    fn process(self, conn: &mut Connection, _: &mut Frontend<EntryData>) -> ActionResult {
        conn.set_watch_mode(self.mode());
        Ok(Message::Ok)
    }
}

impl Action for AddEntry {
    fn process(self, _: &mut Connection, frontend: &mut Frontend<EntryData>) -> ActionResult {
        let id = EntryId::gen(self.timestamp());
        let entry = Entry::new(id, self.consume_data());

        frontend.add_entry(entry)?;

        Ok(Message::EntryAdded(EntryAdded::new(id.timestamp().sec, id.id())))
    }
}

impl Action for RemoveEntry {
    fn process(self, _: &mut Connection, frontend: &mut Frontend<EntryData>) -> ActionResult {
        let id = EntryId::new(self.timestamp(), self.id());

        frontend.remove_entry(id)?;

        Ok(Message::Ok)
    }
}

impl Action for Message {
    fn process(self, conn: &mut Connection, frontend: &mut Frontend<EntryData>) -> ActionResult {
        if !self.is_command() {
            return Err(ActionError::NotACommand);
        }

        match self {
            Message::Ping => Ok(Message::Pong),
            Message::SetWatchMode(msg) => msg.process(conn, frontend),
            Message::AddEntry(msg) => msg.process(conn, frontend),
            Message::RemoveEntry(msg) => msg.process(conn, frontend),
            _ => Err(ActionError::Unimplemented)
        }
    }
}