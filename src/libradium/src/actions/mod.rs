use std::io::{Read, Write, Result as IoResult};
use std::fmt::Debug;
use super::io::{ReadFrom, WriteTo, Error};
use super::action_type::MessageType;
use super::backend::SharedBackend;

mod close;
mod ping;
mod wrapped;

pub use self::close::*;
pub use self::ping::*;
pub use self::wrapped::*;

pub trait Action: Debug {
    fn perform(&self, backend: SharedBackend) -> ActionResponse;
}

#[derive(Debug)]
pub enum ActionResponse {
    Pong(Pong),
    None
}

impl WriteTo for ActionResponse {
    fn write_to<W: Write + Sized>(&self, write: &mut W) -> IoResult<()> {
        match self {
            &ActionResponse::Pong(ref resp) => resp.write_to(write),
            &ActionResponse::None => Ok(())
        }
    }
}

pub fn action<R: Read>(read: &mut R) -> Result<WrappedAction, Error> {
    let msg_type = MessageType::read_from(read)?;

    match msg_type {
        MessageType::Ping => Ok(WrappedAction::Ping(Ping::new())),
        MessageType::Close => Ok(WrappedAction::Close(Close::new()))
    }
}