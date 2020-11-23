use crate::common_traits::*;

macro_rules! option_field {
    ($name:ident, $type: path, $ret: ty) => {
        pub fn $name(&self) -> Option<&$ret> {
            if let $type(x) = self {
                Some(x)
            } else {
                None
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
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
    Array(Vec<Data<T>>),
    //Set
}

#[derive(Eq, PartialEq, Clone, Copy)]
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

    option_field!(table, Data::Table, Box<T>);
    option_field!(int, Data::Int, i32);
    option_field!(float, Data::Float, f32);
    option_field!(str, Data::Str, str);
    option_field!(bool, Data::Bool, bool);
    option_field!(array, Data::Array, Vec<Data<T>>);
}

// /table1/table2/field
// arrays sets etc does not count as table
