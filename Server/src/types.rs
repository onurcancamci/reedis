mod into;
mod traits;
mod wrapper;

use crate::*;
pub use into::*;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::mem::size_of;
pub use traits::*;
pub use wrapper::*;
#[derive(Debug, TryFromPrimitive, Clone)]
#[repr(u16)]
pub enum CommandTypes {
    Get = 0x0001,
    Set = 0x0002,
    Result = 0x0003,
    Terminate = 0xffff,
}
impl CommandTypes {
    pub fn new(buf: &[u8]) -> AppResult<CommandTypes> {
        let mut sl = [0u8; 2];
        sl.copy_from_slice(&buf[0..2]);
        CommandTypes::try_from(u16::from_le_bytes(sl)).map_err(|_| AppError::InvalidCommandType)
    }
    pub fn arg_count(&self) -> u32 {
        match self {
            CommandTypes::Get => 1,
            CommandTypes::Set => 2,
            CommandTypes::Result => 1,
            CommandTypes::Terminate => 0,
        }
    }
}

#[derive(Debug, TryFromPrimitive, Clone)]
#[repr(u16)]
pub enum DataTypes {
    Null = 0x0000,
    String = 0x0001,
    Int64 = 0x0002,
    Float64 = 0x0003,
    Path = 0x0004,
    Table = 0x0005,
    Bool = 0x0006,
    Array = 0x0007,
}

impl DataTypes {
    pub fn new(buf: &[u8]) -> AppResult<DataTypes> {
        let mut sl = [0u8; 2];
        sl.copy_from_slice(&buf[0..2]);
        DataTypes::try_from(u16::from_le_bytes(sl)).map_err(|_| AppError::InvalidDataType)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    String(String),
    Int64(i64),
    Float64(f64),
    Path(TPath),
    Bool(bool),
    Table(Box<Table>),
    Array(Box<Vec<Value>>),
}

impl Value {
    pub fn size(&self) -> usize {
        let type_size = size_of::<u16>();
        match self {
            Value::Null => type_size,
            Value::Bool(_) => 1 + type_size,
            Value::Float64(_) => 8 + type_size,
            Value::Int64(_) => 8 + type_size,
            Value::Path(key) => {
                let str_size_sum: usize = key
                    .iter()
                    .map(|s| s.as_bytes().len())
                    .fold(0, |acc, sl| acc + sl);
                SIZE_USIZE * key.len() + str_size_sum + type_size + 2
            }
            Value::String(s) => s.as_bytes().len() + type_size,
            Value::Table(t) => t.byte_size() + type_size,
            Value::Array(arr) => {
                type_size + 4 + SIZE_USIZE * arr.len() + arr.iter().fold(0, |acc, v| acc + v.size())
            }
        }
    }
    pub fn read_into(&self, buf: &mut [u8]) -> AppResult<()> {
        match &self {
            Value::Bool(v) => buf[0] = if *v { 1 } else { 0 },
            Value::Float64(v) => buf.copy_from_slice(&v.to_le_bytes()),
            Value::Int64(v) => buf.copy_from_slice(&v.to_le_bytes()),
            Value::Null => {}
            Value::Path(v) => {
                buf[0..2].copy_from_slice(&(v.len() as u16).to_le_bytes());
                let mut ind = 2;
                for segment in v {
                    let slen = segment.as_bytes().len();
                    buf[ind..(ind + SIZE_USIZE)].copy_from_slice(&slen.to_le_bytes());
                    ind += SIZE_USIZE;
                    buf[ind..(ind + slen)].copy_from_slice(segment.as_bytes());
                    ind += slen
                }
            }
            Value::String(v) => {
                buf.copy_from_slice(v.as_bytes());
            }
            Value::Table(v) => {
                let kv_count = v.key_count();
                buf[0..4].copy_from_slice(&(kv_count as u32).to_le_bytes());
                let mut ind = 4;
                for (key, value) in v.pairs() {
                    let klen = key.as_bytes().len();
                    buf[ind..(ind + SIZE_USIZE)].copy_from_slice(&klen.to_le_bytes());
                    ind += SIZE_USIZE;
                    buf[ind..(ind + klen)].copy_from_slice(key.as_bytes());
                    ind += klen;
                    let vlen = value.size();
                    buf[ind..(ind + SIZE_USIZE)].copy_from_slice(&vlen.to_le_bytes());
                    ind += SIZE_USIZE;
                    buf[ind..(ind + 2)].copy_from_slice(&(value.data_type() as u16).to_le_bytes()); //value type
                    ind += 2;
                    value.read_into(&mut buf[ind..(ind + vlen - 2)])?;
                    ind += vlen - 2;
                }
            }
            Value::Array(v) => {
                let count = v.len();
                buf[0..4].copy_from_slice(&(count as u32).to_le_bytes());
                let mut ind = 4;
                for value in v.iter() {
                    let vlen = value.size();
                    buf[ind..(ind + SIZE_USIZE)].copy_from_slice(&vlen.to_le_bytes());
                    ind += SIZE_USIZE;
                    buf[ind..(ind + 2)].copy_from_slice(&(value.data_type() as u16).to_le_bytes()); //value type
                    ind += 2;
                    value.read_into(&mut buf[ind..(ind + vlen - 2)])?;
                    ind += vlen - 2;
                }
            }
        };
        Ok(())
    }
    pub fn data_type(&self) -> DataTypes {
        match self {
            Value::Bool(_) => DataTypes::Bool,
            Value::Float64(_) => DataTypes::Float64,
            Value::Int64(_) => DataTypes::Int64,
            Value::Path(_) => DataTypes::Path,
            Value::String(_) => DataTypes::String,
            Value::Table(_) => DataTypes::Table,
            Value::Null => DataTypes::Null,
            Value::Array(_) => DataTypes::Array,
        }
    }
}

// ?
#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum PacketTypes {
    Command = 0x00,
    Data = 0x01,
}
