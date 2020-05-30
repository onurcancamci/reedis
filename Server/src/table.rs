use crate::*;
use std::collections::hash_map::HashMap;
use std::mem::size_of;
#[derive(Debug, Clone)]
pub struct Table {
    path: TPath,
    table: HashMap<String, Value>,
}

impl Table {
    pub fn new(path: TPath) -> Self {
        Table {
            table: HashMap::new(),
            path,
        }
    }
    pub fn with_capacity(path: TPath, capacity: usize) -> Self {
        Table {
            table: HashMap::with_capacity(capacity),
            path,
        }
    }
    pub fn with_hashmap(path: TPath, table: HashMap<String, Value>) -> Self {
        Table { table, path }
    }
    pub fn get(&self, mut key: TPath) -> Option<&Value> {
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
    pub fn get_mut(&mut self, mut key: TPath) -> Option<&mut Value> {
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
    pub fn set(&mut self, mut key: TPath, val: &Value) {
        if key.len() > 0 {
            let curr = key.pop_front().unwrap();
            if key.len() == 0 {
                let mut to_insert = val.clone();
                match &mut to_insert {
                    Value::Array(arr) => {
                        arr.regen_path(self.path.clone());
                    }
                    Value::Table(t) => {
                        let mut np = self.path.clone();
                        np.push_back(curr.clone());
                        t.regen_path(Some(np));
                    }
                    _ => {}
                }
                self.table.insert(curr, to_insert);
            } else {
                let existing_table = self.table.get_mut(&curr);
                match existing_table {
                    Some(Value::Table(existing_table)) => {
                        existing_table.set(key, val);
                    }
                    None => {
                        let mut new_path = self.path.clone();
                        new_path.push_back(curr.clone());
                        let mut ntable = Table::new(new_path);
                        ntable.set(key, val);
                        ntable.regen_path(None);
                        self.table.insert(curr, Value::Table(Box::from(ntable)));
                    }
                    Some(_) => {
                        // ignored
                    }
                }
            }
        }
    }

    pub fn regen_path(&mut self, path: Option<TPath>) {
        if let Some(p) = path {
            self.path = p.clone();
        }
        let p = self.path.clone();
        for (key, val) in self.pairs_mut() {
            match val {
                Value::Array(v) => {
                    let mut scope = p.clone();
                    scope.push_back(key.clone());
                    v.regen_path(scope)
                }
                Value::Table(v) => {
                    let mut scope = p.clone();
                    scope.push_back(key.clone());
                    v.regen_path(Some(scope))
                }
                _ => {}
            }
        }
    }

    pub fn key_count(&self) -> usize {
        self.table.len()
    }
    pub fn pairs(&self) -> std::collections::hash_map::Iter<String, Value> {
        self.table.iter()
    }
    pub fn pairs_mut(&mut self) -> std::collections::hash_map::IterMut<String, Value> {
        self.table.iter_mut()
    }

    pub fn byte_size(&self) -> usize {
        size_of::<u32>()
            + self
                .pairs()
                .map(|(key, val)| SIZE_USIZE * 2 + key.as_bytes().len() + val.size())
                .fold(0, |acc, e| acc + e)
    }
}
