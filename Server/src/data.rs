use crate::common_traits::*;

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

    pub fn table(&self) -> Option<&Box<T>> {
        if let Data::Table(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn int(&self) -> Option<&i32> {
        if let Data::Int(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn float(&self) -> Option<&f32> {
        if let Data::Float(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn str(&self) -> Option<&str> {
        if let Data::Str(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn bool(&self) -> Option<&bool> {
        if let Data::Bool(x) = self {
            Some(x)
        } else {
            None
        }
    }

    pub fn array(&self) -> Option<&Vec<Data<T>>> {
        if let Data::Array(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

// /table1/table2/field
// arrays sets etc does not count as table
