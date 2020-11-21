use crate::common_traits::*;
use crate::error::MyError;

pub trait Database {
    type CommandResult: CommandResult;
    type Event: Event;
    type Table: Table;

    fn run<T>(&self, command: T) -> Result<(Self::CommandResult, Vec<Self::Event>), MyError>
    where
        T: Command;

    fn run_mutable<T>(
        &mut self,
        command: T,
    ) -> Result<(Self::CommandResult, Vec<Self::Event>), MyError>
    where
        T: Command;

    fn run_ev_command<EC>(&mut self, command: EC)
    where
        EC: EventCommand;

    fn table(&self) -> &Self::Table;
    fn table_mut(&mut self) -> &mut Self::Table;
}
