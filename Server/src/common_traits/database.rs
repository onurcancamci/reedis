use crate::common_traits::{Command, CommandResult, Event, EventCommand};
use crate::error::MyError;

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
