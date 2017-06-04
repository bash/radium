extern crate radium_protocol;

use std::net::TcpStream;
use radium_protocol::{Message, Reader, WriteValueExt, SyncReaderController, WatchMode};
use radium_protocol::messages::{AddEntry, SetWatchMode};
use std::env;
use std::io;
use std::io::Write;

fn main() {
    let mode = std::env::var("RADIUM_MODE").unwrap_or(String::from(""));

    let mut stream = TcpStream::connect("127.0.0.1:3126").unwrap();
    let mut reader = SyncReaderController::new(Message::reader());

    stream.write_value(&Message::Ping).unwrap();
    let message = reader.resume(&mut stream).unwrap();
    println!("{:?}", message);
    io::stdout().flush().unwrap();

    stream.write_value(&Message::AddEntry(AddEntry::new(0, 123, vec![1, 2, 3])));
    let message = reader.resume(&mut stream).unwrap();
    println!("{:?}", message);

    if mode == "w" {
        stream.write_value(&Message::SetWatchMode(SetWatchMode::new(WatchMode::All))).unwrap();

        loop {
            let message = reader.resume(&mut stream).unwrap();

            println!("{:?}", message);
        }
    }
}
