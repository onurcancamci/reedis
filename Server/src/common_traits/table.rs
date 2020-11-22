use crate::common_traits::*;
use crate::data::{Data, DataType};
use crate::error::MyError;
use crate::util::*;

pub trait Table: Sized + Clone {
    type Field: Field<Table = Self>;
    type Event: Event;

    fn new() -> Box<Self>;

    fn create_delete_events(
        path: &str,
        op: Operation,
        table: &Box<Self>,
        events: &mut Vec<Self::Event>,
    ) {
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
                    unreachable!();
                }
            }
        }
    }

    fn create_events(
        path: &str,
        op: Operation,
        field: &Self::Field,
        events: &mut Vec<Self::Event>,
    ) {
        for target in field.own_listeners() {
            let event = Self::Event::new(path, op.clone(), target);
            events.push(event);
        }
    }

    fn child_listener_ct(&self) -> usize;

    fn set_child_listener_ct(&mut self, val: usize) -> usize;

    fn get_field(&self, key: &str) -> Option<&Self::Field>;

    fn get_field_mut(&mut self, key: &str) -> Option<&mut Self::Field>;

    ///Sets field  
    ///
    ///Returns error if key exists
    fn set_field(&mut self, key: &str, field: Self::Field) -> Result<(), MyError>;

    fn keys_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a str> + 'a>;

    fn dec_child_listener_ct(&mut self) -> usize {
        self.set_child_listener_ct(self.child_listener_ct() - 1)
    }

    fn inc_child_listener_ct(&mut self) -> usize {
        self.set_child_listener_ct(self.child_listener_ct() + 1)
    }

    fn insert_data(&mut self, key: &str, data: Data<Self>) -> Result<(), MyError> {
        let field = Self::Field::create_with_data(data);
        self.set_field(key, field)
    }

    fn table_exists(&self, key: &str) -> bool {
        if let Some(val) = self.get_field(key) {
            val.data_type() == DataType::Table
        } else {
            false
        }
    }

    /// Set operation for table
    /// Propagates to nested tables when necessary and constructs events
    /// TODO: Find a better way to clear unreachable()! and expects
    fn set(
        &mut self,
        path: (&str, usize),
        data: Data<Self>,
        events: &mut Vec<Self::Event>,
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
}
