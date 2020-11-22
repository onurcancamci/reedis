use crate::common_traits::*;
use crate::data::*;

pub trait CommandResult {
    type Table: Table;

    fn new_data_result(data: Data<Self::Table>, mod_count: usize) -> Self;

    fn new_empty_result(mod_count: usize) -> Self;

    fn modified_row_count(&self) -> usize;

    fn result(&self) -> &ResultTypes<Self::Table>;
}

#[derive(Clone, Debug)]
pub enum ResultTypes<T>
where
    T: Table,
{
    Data(Data<T>),
    None,
}

impl<T> ResultTypes<T>
where
    T: Table,
{
    pub fn kind(&self) -> ResultTypeVariant {
        match self {
            ResultTypes::Data(_) => ResultTypeVariant::Data,
            ResultTypes::None => ResultTypeVariant::None,
        }
    }

    pub fn data(&self) -> Option<&Data<T>> {
        if let ResultTypes::Data(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResultTypeVariant {
    Data,
    None,
}
