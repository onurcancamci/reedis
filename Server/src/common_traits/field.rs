use crate::common_traits::*;
use crate::data::{Data, DataType};

pub trait Field {
    type Table: Table;

    fn create_with_data(data: Data<Self::Table>) -> Self;

    fn child_listener_ct(&self) -> usize;

    /// Returns new value
    fn set_child_listener_ct(&mut self, val: usize) -> usize;

    fn own_listener_ct(&self) -> usize;
    fn add_listener(&mut self, listener: usize);
    fn remove_listener(&mut self, listener: usize);
    fn own_listeners<'a>(&'a self) -> Box<dyn Iterator<Item = usize> + 'a>;

    fn get_data(&self) -> &Data<Self::Table>;
    fn get_data_mut(&mut self) -> &mut Data<Self::Table>;

    fn replace_data(&mut self, data: Data<Self::Table>) -> Data<Self::Table> {
        std::mem::replace(self.get_data_mut(), data)
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
