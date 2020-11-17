use crate::error::MyError;
use crate::message_types::{Command, CommandResult, Event};

pub trait Database {
    fn run<T, C, E>(command: T) -> Result<(C, Vec<E>), MyError>
    where
        T: Command,
        C: CommandResult,
        E: Event;
}
