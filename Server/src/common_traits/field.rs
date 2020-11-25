use crate::common_traits::*;
use crate::data::{Data, DataType};

pub trait Field {
    type Table: Table;

    fn create_with_data(data: Data<Self::Table>) -> Self;

    fn get_data(&self) -> &Data<Self::Table>;
    fn get_data_mut(&mut self) -> &mut Data<Self::Table>;

    fn replace_data(&mut self, data: Data<Self::Table>) -> Data<Self::Table> {
        std::mem::replace(self.get_data_mut(), data)
    }

    fn data_type(&self) -> DataType {
        self.get_data().data_type()
    }
}
