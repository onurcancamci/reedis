use std::sync::{Arc, Mutex};

use crate::common_traits::*;
use crate::error::MyError;

pub trait Database<E>
where
    E: Event,
{
    type CommandResult: CommandResult<Table = Self::Table>;
    type Command: Command<Table = Self::Table>;
    type Table: Table
        + TableMethods<E, Command = Self::Command, CommandResult = Self::CommandResult>;

    fn run(
        &self,
        context: Arc<Mutex<impl ExecutionContext<E>>>,
        command: Self::Command,
    ) -> Result<Self::CommandResult, MyError> {
        self.table().run(context, command)
    }

    fn run_mutable(
        &mut self,
        context: Arc<Mutex<impl ExecutionContext<E>>>,
        command: Self::Command,
    ) -> Result<Self::CommandResult, MyError> {
        self.table_mut().run_mutable(context, command)
    }

    fn table(&self) -> &Self::Table;
    fn table_mut(&mut self) -> &mut Self::Table;
}
