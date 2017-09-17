#![feature(try_from)]

#[macro_use]
extern crate serde_derive;

extern crate serde;

#[cfg(test)]
extern crate serde_json;

mod message;
mod watch_mode;
mod error_code;

pub mod errors;
pub use self::message::*;
pub use self::watch_mode::*;
pub use self::error_code::*;
