use crate::common_traits::*;
use crate::data::*;
use crate::error::*;

pub trait CommandResult {
    type Table: Table;

    fn new_data_result(data: impl Iterator<Item = Data<Self::Table>>, mod_count: usize) -> Self;

    fn new_error_result(err: MyError, mod_count: usize) -> Self;

    fn modified_row_count(&self) -> usize;

    fn results<'a>(
        &'a self,
    ) -> Result<Box<dyn Iterator<Item = &'a Data<Self::Table>> + 'a>, MyError>;
}
