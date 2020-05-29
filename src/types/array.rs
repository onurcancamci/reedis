use crate::*;

pub trait ValueArray {
    fn regen_path(&mut self, path: TPath);
}

impl ValueArray for Vec<Value> {
    fn regen_path(&mut self, path: TPath) {
        for (i, val) in self.iter_mut().enumerate() {
            match val {
                Value::Array(v) => {
                    let mut scope = path.clone();
                    scope.push_back(i.to_string());
                    v.regen_path(scope)
                }
                Value::Table(v) => {
                    let mut scope = path.clone();
                    scope.push_back(i.to_string());
                    v.regen_path(Some(scope))
                }
                _ => {}
            }
        }
    }
}
