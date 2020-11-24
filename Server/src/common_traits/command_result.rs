use crate::common_traits::*;
use crate::data::*;
use crate::error::*;

pub trait CommandResult {
    type Table: Table;

    fn new_data_result(data: Data<Self::Table>, mod_count: usize) -> Self;

    fn new_empty_result(mod_count: usize) -> Self;

    fn new_error_result(err: MyError) -> Self;

    fn modified_row_count(&self) -> usize;

    fn result(&self) -> &ResultTypes<Self::Table>;
}

#[derive(Clone, Debug)]
pub enum ResultTypes<T>
where
    T: Table,
{
    Data(Data<T>),
    Error(MyError),
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
            ResultTypes::Error(_) => ResultTypeVariant::Error,
        }
    }

    pub fn data(&self) -> Option<&Data<T>> {
        if let ResultTypes::Data(data) = self {
            Some(data)
        } else {
            None
        }
    }

    pub fn error(&self) -> Option<&MyError> {
        if let ResultTypes::Error(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResultTypeVariant {
    Data,
    Error,
    None,
}
