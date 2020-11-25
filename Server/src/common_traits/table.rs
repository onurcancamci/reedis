use crate::common_traits::*;
use crate::data::{Data, DataType};
use crate::error::MyError;

pub trait Table: Sized + Clone {
    type Field: Field<Table = Self>;

    fn new() -> Box<Self>;

    fn get_field(&self, key: &str) -> Option<&Self::Field>;

    fn get_field_mut(&mut self, key: &str) -> Option<&mut Self::Field>;

    ///Sets field  
    ///
    ///Returns error if key exists
    fn set_field(&mut self, key: &str, field: Self::Field) -> Result<(), MyError>;

    fn keys_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a str> + 'a>;

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
}
