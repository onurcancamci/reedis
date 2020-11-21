use crate::common_traits::*;

pub enum Data<T>
where
    T: Table,
{
    Null,
    Table(Box<T>),
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
    Array(Vec<Box<Data<T>>>),
    //Set
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DataType {
    Null,
    Table,
    Int,
    Float,
    Str,
    Bool,
    Array,
}

impl<T> Data<T>
where
    T: Table,
{
    pub fn data_type(&self) -> DataType {
        use Data::*;
        match self {
            Null => DataType::Null,
            Table(_) => DataType::Table,
            Int(_) => DataType::Int,
            Float(_) => DataType::Float,
            Str(_) => DataType::Str,
            Bool(_) => DataType::Bool,
            Array(_) => DataType::Array,
        }
    }
}

// /table1/table2/field
// arrays sets etc does not count as table
