use crate::common_traits::*;
use crate::data::*;
use crate::error::*;
use crate::util::*;
use std::{
    iter::once,
    sync::{Arc, Mutex},
};

pub trait TableMethods<E>: Table
where
    E: Event,
{
    type Command: Command<Table = Self>;
    type CommandResult: CommandResult<Table = Self>;

    fn create_delete_events(
        context: Arc<Mutex<impl ExecutionContext<E>>>,
        path: &str,
        op: Operation,
        table: &Box<Self>,
    ) {
        //check event table
        {
            let ctx = context.lock().unwrap();
            let res = ctx.event_table().lookup(path);

            for id in res {
                ctx.tx_event().send(E::new(path, op.clone(), id)).unwrap();
            }
        }

        for key in table.keys_iter() {
            let field = table.get_field(key).unwrap();
            if let Data::Table(inner) = field.get_data() {
                Self::create_delete_events(
                    Arc::clone(&context),
                    format!("{}/{}", path, key).as_str(),
                    op.clone(),
                    inner,
                );
            }
        }
    }

    fn run(
        &self,
        context: Arc<Mutex<impl ExecutionContext<E>>>,
        command: Self::Command,
    ) -> Result<Self::CommandResult, MyError> {
        let op = command.get_operation();
        match op {
            Operation::Get => {
                let data = self.get((command.get_path().ok_or(MyError::MalformedCommand)?, 0))?;
                let cres = Self::CommandResult::new_data_result(once(data.clone()), 0);
                Ok(cres)
            }
            Operation::Set => panic!("Wrong run variant"),
            Operation::Terminate => panic!("Terminate cant run on table"),
        }
    }

    fn run_mutable(
        &mut self,
        context: Arc<Mutex<impl ExecutionContext<E>>>,
        command: Self::Command,
    ) -> Result<Self::CommandResult, MyError> {
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
                let mod_count = self.set(
                    context,
                    (command.get_path().ok_or(MyError::MalformedCommand)?, 0),
                    data.clone(),
                );
                let cres = Self::CommandResult::new_data_result(vec![].into_iter(), mod_count);
                Ok(cres)
            }
            Operation::Terminate => panic!("Terminate cant run on table"),
        }
    }

    /// Set operation for table
    /// Propagates to nested tables when necessary and constructs events
    /// TODO: Find a better way to clear unreachable()! and expects
    fn set(
        &mut self,
        context: Arc<Mutex<impl ExecutionContext<E>>>,
        path: (&str, usize),
        data: Data<Self>,
    ) -> usize {
        let mut mod_count = 0;
        let key_ind = next_path_key(path);
        match key_ind {
            (key, Some(new_ind)) => {
                // index is found, find that table and propagate command
                if !self.table_exists(key) {
                    self.insert_data(key, Data::Table(Table::new()))
                        .expect("TODO, Cant insert table");
                    mod_count += 1;
                }
                let table_field = self.get_field_mut(key).unwrap_or_else(|| unreachable!());
                let table = table_field.get_data_mut();
                if let Data::Table(table) = table {
                    let inner_mod_count = table.set(context, (path.0, new_ind), data);
                    mod_count += inner_mod_count;
                //TODO: if recursed set modified anything, trigger event for this table
                //
                } else {
                    unreachable!();
                }
            }
            (key, None) => {
                // field belongs to this table
                let field = self.get_field_mut(key);
                match field {
                    Some(field) => {
                        // remove field
                        let old_data = field.replace_data(data);
                        if let Data::Table(old_table) = old_data {
                            //Self::create_delete_events(context, path.0, Operation::Set, &old_table);
                        }
                        // TODO: update event for this table
                    }
                    None => {
                        self.insert_data(key, data)
                            .expect("TODO: Cant insert data, Memory issue?");
                        mod_count += 1;
                    }
                }
            }
        }
        mod_count
    }

    fn get(&self, path: (&str, usize)) -> Result<&Data<Self>, MyError> {
        let key_ind = next_path_key(path);
        match key_ind {
            (key, Some(new_ind)) => {
                if let Data::Table(table) = self
                    .get_field(key)
                    .map(|e| e.get_data())
                    .ok_or(MyError::KeyNotFound)?
                {
                    table.get((path.0, new_ind))
                } else {
                    Err(MyError::PathContainsPrimitiveValue)
                }
            }
            (key, None) => Ok(self.get_field(key).ok_or(MyError::KeyNotFound)?.get_data()),
        }
    }

    //
}
