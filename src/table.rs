use crate::*;
use std::collections::hash_map::HashMap;
use std::mem::size_of;
#[derive(Debug, Clone)]
pub struct Table {
    path: Vec<String>,
    table: HashMap<String, Value>,
}

impl Table {
    pub fn new(path: Vec<String>) -> Self {
        Table {
            table: HashMap::new(),
            path,
        }
    }
    pub fn with_capacity(path: Vec<String>, capacity: usize) -> Self {
        Table {
            table: HashMap::with_capacity(capacity),
            path,
        }
    }
    pub fn with_hashmap(path: Vec<String>, table: HashMap<String, Value>) -> Self {
        Table { table, path }
    }
    pub fn get(&self, mut key: VecDeque<String>) -> Option<&Value> {
        if key.len() > 0 {
            let curr = key.pop_front().unwrap();
            let val = self.table.get(curr.as_str());
            if key.len() == 0 {
                val
            } else {
                match val {
                    Some(Value::Table(t2)) => t2.get(key),
                    _ => None,
                }
            }
        } else {
            None
        }
    }
    pub fn get_mut(&mut self, mut key: VecDeque<String>) -> Option<&mut Value> {
        if key.len() > 0 {
            let curr = key.pop_front().unwrap();
            let val = self.table.get_mut(curr.as_str());
            if key.len() == 1 {
                val
            } else {
                match val {
                    Some(Value::Table(t2)) => t2.get_mut(key),
                    _ => None,
                }
            }
        } else {
            None
        }
    }
    pub fn set(&mut self, mut key: VecDeque<String>, val: &Value) {
        if key.len() > 0 {
            let curr = key.pop_front().unwrap();
            if key.len() == 0 {
                self.table.insert(curr, val.clone());
            } else {
                let existing_table = self.table.get_mut(&curr);
                match existing_table {
                    Some(Value::Table(existing_table)) => {
                        existing_table.set(key, val);
                    }
                    None => {
                        let mut new_path = self.path.clone();
                        new_path.push(curr.clone());
                        let mut ntable = Table::new(new_path);
                        ntable.set(key, val);
                        self.table.insert(curr, Value::Table(Box::from(ntable)));
                    }
                    Some(_) => {
                        // ignored
                    }
                }
            }
        }
    }

    pub fn key_count(&self) -> usize {
        self.table.len()
    }
    pub fn pairs(&self) -> std::collections::hash_map::Iter<String, Value> {
        self.table.iter()
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u32>()
            + self
                .pairs()
                .map(|(key, val)| SIZE_USIZE * 2 + key.as_bytes().len() + val.size())
                .fold(0, |acc, e| acc + e)
    }
}
