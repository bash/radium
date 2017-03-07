use std::io::{Read, Write, Result as IoResult};
use std::fmt::Debug;
use byteorder::{NetworkEndian, WriteBytesExt};
use super::io::{ReadFrom, WriteTo, Error};
use super::action_type::MessageType;
use super::backend::SharedBackend;

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

#[derive(Debug)]
pub enum WrappedAction {
    Ping(Ping),
    Close(Close)
}

impl Action for WrappedAction {
    fn perform(&self, backend: SharedBackend) -> ActionResponse {
        match self {
            &WrappedAction::Ping(ref action) => action.perform(backend),
            &WrappedAction::Close(ref action) => action.perform(backend)
        }
    }
}

#[derive(Debug)]
pub struct Ping {}

impl Ping {
    pub fn new() -> Self {
        Ping {}
    }
}

impl Action for Ping {
    fn perform(&self, _: SharedBackend) -> ActionResponse {
        ActionResponse::Pong(Pong::new())
    }
}

impl ReadFrom for Ping {
    fn read_from<R: Read>(_: &mut R) -> Result<Self, Error> {
        Ok(Ping::new())
    }
}


#[derive(Debug)]
pub struct Pong {}

impl Pong {
    pub fn new() -> Self {
        Pong {}
    }
}

impl WriteTo for Pong {
    fn write_to<W: Write>(&self, write: &mut W) -> IoResult<()> {
        write.write_u16::<NetworkEndian>(1)
    }
}

#[derive(Debug)]
pub struct Close {}

impl Close {
    pub fn new() -> Self {
        Close {}
    }
}

impl Action for Close {
    fn perform(&self, _: SharedBackend) -> ActionResponse {
        ActionResponse::None
    }
}

pub fn action<R: Read>(read: &mut R) -> Result<WrappedAction, Error> {
    let msg_type = MessageType::read_from(read)?;

    match msg_type {
        MessageType::Ping => Ok(WrappedAction::Ping(Ping::new())),
        MessageType::Close => Ok(WrappedAction::Close(Close::new()))
    }
}