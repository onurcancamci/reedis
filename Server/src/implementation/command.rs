use crate::common_traits::*;
use crate::{Command, Table};

#[derive(Clone)]
pub struct MyCommand<T>
where
    T: Table + Sized,
{
    op: Operation,
    args: Vec<CommandArg<T, Self>>,
    path: Option<String>,
}

impl<T> Command for MyCommand<T>
where
    T: Table,
{
    type Table = T;

    fn new_with_vec<'a>(
        op: Operation,
        path: Option<String>,
        args: Vec<CommandArg<Self::Table, Self>>,
    ) -> Self {
        MyCommand { op, path, args }
    }

    fn get_path<'a>(&'a self) -> Option<&'a str> {
        match &self.path {
            Some(s) => Some(s.as_str()),
            None => None,
        }
    }

    fn is_mutator(&self) -> bool {
        self.op.is_mutator()
    }

    fn is_terminate(&self) -> bool {
        self.op.is_terminate()
    }

    fn get_operation(&self) -> Operation {
        self.op.clone()
    }

    fn get_args_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a CommandArg<Self::Table, Self>> + 'a> {
        Box::new(self.args.iter())
    }
}
