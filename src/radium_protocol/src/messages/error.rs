#[derive(Copy, Clone, Debug)]
pub enum ErrorCode {
    /// The client was rejected because
    /// the worker was unable to handle more clients
    ClientRejected,
    /// The action that was sent is not implemented
    ActionNotImplemented,
    /// The message that was sent is not an action
    InvalidAction
}

impl Into<u8> for ErrorCode {
    fn into(self) -> T {
        match self {
            ErrorCode::ClientRejected => 0,
            ErrorCode::ActionNotImplemented => 1,
            ErrorCode::InvalidAction => 2
        }
    }
}