use crate::common_traits::*;
use crate::data::*;
use crate::error::*;
use crate::util::*;

pub trait TableMethods<E>: Table
where
    E: Event,
{
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
}
