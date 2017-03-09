use super::{Action, ActionResponse};
use super::super::backend::SharedBackend;

#[derive(Debug)]
pub struct Close {}

impl Close {
    pub fn new() -> Self {
        Close {}
    }
}

impl Action for Close {
    fn perform(&self, _: SharedBackend) -> ActionResponse {
        ActionResponse::None
    }
}
