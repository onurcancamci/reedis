use crate::error::MyError;
use crate::message_types::{Command, CommandResult, Event, EventCommand};

pub trait Database {
    fn run<T, C, E>(&mut self, command: T) -> Result<(C, Vec<E>), MyError>
    where
        T: Command,
        C: CommandResult,
        E: Event;

    fn run_ev_command<EC>(&mut self, command: EC)
    where
        EC: EventCommand;
}
