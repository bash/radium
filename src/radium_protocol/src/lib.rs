#![feature(try_from)]
// TODO: only temporary until old parsing is removed
#![allow(deprecated)]

extern crate byteorder;

#[macro_use]
mod macros;

#[cfg(test)]
#[macro_use]
mod test_helpers;

mod message;
mod message_type;

#[deprecated(note = "Use new Reader/Writer api instead")]
mod io;

mod watch_mode;
mod error_code;

pub mod messages;
pub mod errors;
pub mod reader;
pub mod writer;

pub use self::message_type::*;
pub use self::message::*;
pub use self::io::*;
pub use self::watch_mode::*;
pub use self::error_code::*;
