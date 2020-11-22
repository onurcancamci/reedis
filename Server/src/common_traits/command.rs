use crate::common_traits::*;
use crate::data::*;

pub trait Command {
    type Table: Table;

    fn is_terminate(&self) -> bool;

    fn is_mutator(&self) -> bool;

    fn get_path<'a>(&'a self) -> Option<&'a str>;

    fn get_operation(&self) -> Operation;

    fn get_args_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a CommandArg<Self::Table>> + 'a>;
}

pub enum CommandArg<T>
where
    T: Table,
{
    Data(Data<T>),
}

#[derive(Clone, Debug)]
pub enum Operation {
    Get,
    Set,
    Terminate,
}
