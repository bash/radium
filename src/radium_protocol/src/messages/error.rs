use std::io;
use super::super::{ReadFrom, WriteTo, ErrorCode, ReadResult, WriteResult};

#[derive(Debug)]
pub struct ErrorMessage {
    code: ErrorCode
}

impl ErrorMessage {
    pub fn new(code: ErrorCode) -> Self {
        ErrorMessage { code }
    }

    pub fn code(&self) -> ErrorCode {
        self.code
    }
}

impl ReadFrom for ErrorMessage {
    fn read_from<R: io::Read>(source: &mut R) -> ReadResult<Self> {
        let code = ErrorCode::read_from(source)?;

        Ok(ErrorMessage::new(code))
    }
}

impl WriteTo for ErrorMessage {
    fn write_to<W: io::Write>(&self, target: &mut W) -> WriteResult {
        self.code.write_to(target)
    }
}