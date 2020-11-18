use crate::error::MyError;
use crate::message_types::{Command, CommandResult, Event, EventCommand};

pub trait Database {
    type CommandResult;
    type Event;

    fn run<T>(&mut self, command: T) -> Result<(Self::CommandResult, Vec<Self::Event>), MyError>
    where
        T: Command;

    fn run_ev_command<EC>(&mut self, command: EC)
    where
        EC: EventCommand;
}
