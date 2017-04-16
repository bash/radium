extern crate byteorder;

mod command;

pub enum ConnectionType {
    Command,
    Listen,
}