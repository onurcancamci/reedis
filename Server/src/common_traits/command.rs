use crate::common_traits::*;
use crate::data::*;

pub trait Command: Sized {
    type Table: Table;

    //TODO: for parser, new functions are required

    //fn new(op: Operation, path: Option<String>)

    fn is_terminate(&self) -> bool;

    fn is_mutator(&self) -> bool;

    fn get_path<'a>(&'a self) -> Option<&'a str>;

    fn get_operation(&self) -> Operation;

    fn get_args_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a CommandArg<Self::Table, Self>> + 'a>;
}

#[derive(Debug)]
pub enum CommandArg<T, C>
where
    T: Table,
    C: Command<Table = T>,
{
    Data(Data<T>),
    Command(C),
}

impl<T, C> CommandArg<T, C>
where
    T: Table,
    C: Command<Table = T>,
{
    pub fn data(&self) -> Option<&Data<T>> {
        if let CommandArg::Data(d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn command(&self) -> Option<&C> {
        if let CommandArg::Command(d) = self {
            Some(d)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Get,
    Set,
    Terminate,
}

impl Operation {
    pub fn is_mutator(&self) -> bool {
        match self {
            Operation::Get => false,
            Operation::Set => true,
            Operation::Terminate => false,
        }
    }
}
