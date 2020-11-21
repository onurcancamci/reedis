use crate::common_traits::*;
use crate::data::{Data, DataType};
use crate::error::MyError;

pub trait Field {
    type Table: Table;

    fn create_with_data(data: Data<Self::Table>) -> Self;

    fn child_listener_ct(&self) -> usize;
    fn set_child_listener_ct(&mut self, val: usize) -> usize;

    fn own_listener_ct(&self) -> usize;
    fn add_listener(&mut self) -> Result<(), MyError>;
    fn remove_listener(&mut self, listener: usize) -> Result<(), MyError>;
    fn own_listeners(&self) -> Box<dyn Iterator<Item = usize>>;

    fn get_data(&self) -> &Data<Self::Table>;
    fn get_mut_data(&mut self) -> &mut Data<Self::Table>;

    fn replace_data(&mut self, data: Data<Self::Table>) -> Data<Self::Table> {
        std::mem::replace(self.get_mut_data(), data)
    }

    fn inc_child_listener_ct(&mut self) -> usize {
        self.set_child_listener_ct(self.child_listener_ct() + 1)
    }

    fn dec_child_listener_ct(&mut self) -> usize {
        self.set_child_listener_ct(self.child_listener_ct() - 1)
    }

    fn data_type(&self) -> DataType {
        self.get_data().data_type()
    }
}
