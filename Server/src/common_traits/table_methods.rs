use crate::common_traits::*;
use crate::data::*;
use crate::error::*;
use crate::util::*;
use std::iter::once;

pub trait TableMethods<E>: Table
where
    E: Event,
{
    type Command: Command<Table = Self>;
    type CommandResult: CommandResult<Table = Self>;

    fn create_delete_events(path: &str, op: Operation, table: &Box<Self>, events: &mut Vec<E>) {
        if table.child_listener_ct() <= 0 {
            return;
        }
        // iterate over keys and values
        for key in table.keys_iter() {
            let field = table.get_field(key).unwrap();
            if field.own_listener_ct() > 0 {
                // create events
                Self::create_events(path, op.clone(), &field, events);
            }
            if field.child_listener_ct() > 0 {
                if let Data::Table(nested) = field.get_data() {
                    Self::create_delete_events(path, op.clone(), nested, events);
                } else {
                    //TODO: arraysa icinde gezin ve bul
                    unreachable!();
                }
            }
        }
    }

    fn create_events(path: &str, op: Operation, field: &Self::Field, events: &mut Vec<E>) {
        for target in field.own_listeners() {
            let event = E::new(path, op.clone(), target);
            events.push(event);
        }
    }

    fn run(&self, command: Self::Command) -> Result<(Self::CommandResult, Vec<E>), MyError> {
        let op = command.get_operation();
        match op {
            Operation::Get => {
                let data = self.get((command.get_path().ok_or(MyError::MalformedCommand)?, 0))?;
                let cres = Self::CommandResult::new_data_result(once(data.clone()), 0);
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
                let mod_count = self.set(
                    (command.get_path().ok_or(MyError::MalformedCommand)?, 0),
                    data.clone(),
                    &mut events,
                );
                let cres = Self::CommandResult::new_data_result(vec![].into_iter(), mod_count);
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

    /// Set operation for table
    /// Propagates to nested tables when necessary and constructs events
    /// TODO: Find a better way to clear unreachable()! and expects
    fn set(&mut self, path: (&str, usize), data: Data<Self>, events: &mut Vec<E>) -> usize {
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
                    let inner_mod_count = table.set((path.0, new_ind), data, events);
                    mod_count += inner_mod_count;
                    if inner_mod_count > 0 && table_field.own_listener_ct() > 0 {
                        // create events
                        Self::create_events(path.0, Operation::Set, &table_field, events);
                    }
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
                            Self::create_delete_events(path.0, Operation::Set, &old_table, events);
                        }

                        if field.own_listener_ct() > 0 {
                            // create events
                            Self::create_events(path.0, Operation::Set, &field, events);
                        }
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

    ///Adds target as listener on specified field. Returns error if key not found or path is
    ///invalid. Returns true if listener is added. If same id is already listening, it returns
    ///false.
    fn add_listener(&mut self, path: (&str, usize), target: usize) -> Result<bool, MyError> {
        unimplemented!()
    }

    ///Internal function to update child_listener_ct fields if add_listener is successfull.
    ///Since this will only be called when add_listener is successfull and we still have the lock,
    ///operation will panic on error.
    fn mod_child_listener_ct(&mut self, path: (&str, usize), is_add: bool) {}
}
