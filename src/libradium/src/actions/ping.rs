use std::io::{Read, Write, Result as IoResult};
use byteorder::{NetworkEndian, WriteBytesExt};
use super::{Action, ActionResponse};
use super::super::backend::SharedBackend;
use super::super::io::{ReadFrom, WriteTo, Error};

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