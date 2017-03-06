use std::io::{Read, Write, Result as IoResult};
use std::fmt::Debug;
use byteorder::{NetworkEndian, WriteBytesExt};
use ::io::{Readable, Writable, Error};
use ::message_type::MessageType;
use ::backend::SharedBackend;

pub trait Action: Sized + Debug {
    type Resp: Writable;

    fn perform(&self, backend: SharedBackend) -> Self::Resp;
}

#[derive(Debug)]
pub struct Ping {}

#[derive(Debug)]
pub struct Pong {}

impl Ping {
    pub fn new() -> Self {
        Ping {}
    }
}

impl Pong {
    pub fn new() -> Self {
        Pong {}
    }
}

impl Action for Ping {
    type Resp = Pong;

    fn perform(&self, backend: SharedBackend) -> Self::Resp {
        println!("Pinging");

        Pong::new()
    }
}

impl Readable for Ping {
    fn read_from<R: Read>(read: &mut R) -> Result<Self, Error> {
        Ok(Ping::new())
    }
}

impl Writable for Pong {
    fn write_to<W: Write>(&self, write: &mut W) -> IoResult<()> {
        write.write_u16::<NetworkEndian>(1)
    }
}

pub fn action<R: Read>(read: &mut R) -> Result<impl Action, Error> {
    let msg_type = MessageType::read_from(read)?;

    match msg_type {
        MessageType::Ping => Ok(Ping::read_from(read)?)
    }
}