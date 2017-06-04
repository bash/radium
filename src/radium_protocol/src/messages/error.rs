use std::io;
use super::super::{WriteTo, ErrorCode, WriteResult, MessageInner, Message};
use super::super::reader::{Reader, ReaderStatus, HasReader};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ErrorMessage {
    code: ErrorCode,
}

#[derive(Debug)]
pub struct ErrorMessageReader;

impl ErrorMessage {
    pub fn new(code: ErrorCode) -> Self {
        ErrorMessage { code }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }
}

impl MessageInner for ErrorMessage {
    fn wrap(self) -> Message {
        Message::Error(self)
    }
}

impl HasReader for ErrorMessage {
    type Reader = ErrorMessageReader;

    fn reader() -> Self::Reader {
        ErrorMessageReader {}
    }
}

impl WriteTo for ErrorMessage {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        self.code.write_to(target)
    }
}

impl Reader<ErrorMessage> for ErrorMessageReader {
    fn resume<I>(&mut self, input: &mut I) -> io::Result<ReaderStatus<ErrorMessage>> where I: io::Read {
        let mut reader = ErrorCode::reader();
        let status = reader.resume(input)?;

        Ok(status.map(|code| ErrorMessage::new(code)))
    }

    fn rewind(&mut self) {}
}