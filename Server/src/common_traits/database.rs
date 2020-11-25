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

    fn run(&self, command: Self::Command) -> Result<(Self::CommandResult, Vec<E>), MyError> {
        self.table().run(command)
    }

    fn run_mutable(
        &mut self,
        command: Self::Command,
    ) -> Result<(Self::CommandResult, Vec<E>), MyError> {
        self.table_mut().run_mutable(command)
    }

    fn run_ev_command<EC>(&mut self, command: EC)
    where
        EC: EventCommand,
    {
        self.table_mut().run_ev_command(command)
    }

    fn table(&self) -> &Self::Table;
    fn table_mut(&mut self) -> &mut Self::Table;
}
