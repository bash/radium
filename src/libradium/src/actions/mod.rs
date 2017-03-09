use std::io::{Read, Write};
use std::fmt::Debug;
use super::io::{Error, ReadFrom, WriteTo, WriteToResult};
use super::action_type::ActionType;
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
    None,
}

impl WriteTo for ActionResponse {
    fn write_to<W: Write + Sized>(&self, write: &mut W) -> WriteToResult {
        match self {
            &ActionResponse::Pong(ref resp) => resp.write_to(write),
            &ActionResponse::None => Ok(()),
        }
    }
}

pub fn action<R: Read>(read: &mut R) -> Result<WrappedAction, Error> {
    let msg_type = ActionType::read_from(read)?;

    match msg_type {
        ActionType::Ping => Ok(WrappedAction::Ping(Ping::new())),
        ActionType::Close => Ok(WrappedAction::Close(Close::new())),
    }
}
