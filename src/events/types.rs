use crate::*;

#[derive(Debug, Clone)]
pub enum Event {
    Empty, //for testing only TODO: remove
    Change(Value),
}
#[derive(Debug, Clone)]
pub struct EventObj {
    path: TPath,
    event: Event,
}

impl EventObj {
    pub fn new(path: TPath, event: Event) -> Self {
        EventObj { path, event }
    }
}
