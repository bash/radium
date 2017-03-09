use super::{Ping, Close};
use super::{Action, ActionResponse};
use super::super::backend::SharedBackend;

#[derive(Debug)]
pub enum WrappedAction {
    Ping(Ping),
    Close(Close),
}

impl Action for WrappedAction {
    fn perform(&self, backend: SharedBackend) -> ActionResponse {
        match self {
            &WrappedAction::Ping(ref action) => action.perform(backend),
            &WrappedAction::Close(ref action) => action.perform(backend),
        }
    }
}
