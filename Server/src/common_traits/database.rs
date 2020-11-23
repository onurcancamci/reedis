use crate::common_traits::*;
use crate::error::MyError;

pub trait Database<E>
where
    E: Event,
{
    type CommandResult: CommandResult<Table = Self::Table>;
    type Command: Command<Table = Self::Table>;
    type Table: Table + TableMethods<E>;

    fn run(&self, command: Self::Command) -> Result<(Self::CommandResult, Vec<E>), MyError> {
        let op = command.get_operation();
        match op {
            Operation::Get => {
                let data = self
                    .table()
                    .get((command.get_path().ok_or(MyError::MalformedCommand)?, 0))?;
                let cres = Self::CommandResult::new_data_result(data.clone(), 0);
                Ok((cres, vec![]))
            }
            Operation::Set => panic!("Wrong run variant"),
            Operation::Terminate => panic!("Terminate cant run on table"),
        }
    }

    fn run_mutable(
        &mut self,
        command: Self::Command,
    ) -> Result<(Self::CommandResult, Vec<E>), MyError> {
        let op = command.get_operation();
        match op {
            Operation::Get => panic!("Wrong run variant"),
            Operation::Set => {
                let data = command
                    .get_args_iter()
                    .next()
                    .ok_or(MyError::MalformedCommand)?
                    .data()
                    .ok_or(MyError::MalformedCommand)?;
                let mut events: Vec<E> = vec![];
                let mod_count = self.table_mut().set(
                    (command.get_path().ok_or(MyError::MalformedCommand)?, 0),
                    data.clone(),
                    &mut events,
                );
                let cres = Self::CommandResult::new_empty_result(mod_count);
                Ok((cres, events))
            }
            Operation::Terminate => panic!("Terminate cant run on table"),
        }
    }

    fn run_ev_command<EC>(&mut self, command: EC)
    where
        EC: EventCommand,
    {
        unimplemented!()
    }

    fn table(&self) -> &Self::Table;
    fn table_mut(&mut self) -> &mut Self::Table;
}
