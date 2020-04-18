use crate::*;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::mem::size_of;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct CommandWrapper<'a> {
    content: &'a [u8],
    command: CommandTypes,
    args: Vec<DataWrapper<'a>>,
}

impl<'a> CommandWrapper<'a> {
    pub fn new(buf: &'a [u8]) -> AppResult<Self> {
        let command = CommandTypes::new(&buf[0..2])?;
        let content = &buf[2..];
        let mut args: Vec<DataWrapper<'a>> = Vec::with_capacity(command.arg_count() as usize);

        let mut ind = 0;
        while ind < content.len() {
            let len = usize_from(&content[ind..(ind + SIZE_USIZE)]);
            let arg_content = &content[(ind + SIZE_USIZE)..(ind + SIZE_USIZE + len)];
            args.push(DataWrapper::new(arg_content)?);
            ind += SIZE_USIZE + len;
        }
        Ok(CommandWrapper {
            args,
            command,
            content: &buf,
        })
    }
    pub fn raw_args(&self) -> &Vec<DataWrapper<'a>> {
        &self.args
    }
}

impl<'a> Command<'a> for CommandWrapper<'a> {
    fn args(&'a self) -> Box<dyn ArgIter<'a> + 'a> {
        let ai: ArgIterWrapper<'a> = ArgIterWrapper {
            index: 0,
            args: &self.args,
        };
        Box::new(ai)
    }
    fn command_type(&self) -> &CommandTypes {
        &self.command
    }
    fn size(&self) -> usize {
        self.content.len()
    }
    fn read_into(&self, buf: &mut [u8]) -> AppResult<()> {
        if buf.len() == self.content.len() {
            buf.copy_from_slice(self.content);
            Ok(())
        } else {
            Err(AppError::SizeCalculationIsInvalid)
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataWrapper<'a> {
    content: &'a [u8],
    data_type: DataTypes,
    content_cache: Option<Value>,
}

impl<'a> DataWrapper<'a> {
    pub fn new(buf: &'a [u8]) -> AppResult<Self> {
        let data_type = DataTypes::new(&buf[0..2])?;
        let content = &buf[0..];
        Ok(DataWrapper {
            data_type,
            content,
            content_cache: None,
        })
    }
}

impl<'a> Data<'a> for DataWrapper<'a> {
    fn data_type(&self) -> &DataTypes {
        &self.data_type
    }
    fn data(&self) -> AppResult<Value> {
        let content = &self.content[2..];
        match self.data_type {
            DataTypes::Null => Ok(Value::Null),
            DataTypes::Int64 => Ok(Value::Int64(i64_from(content))),
            DataTypes::Float64 => Ok(Value::Float64(f64_from(content))),
            DataTypes::Bool => Ok(Value::Bool(if content[0] == 1 { true } else { false })),
            DataTypes::String => Ok(Value::String(string_from(content)?)),
            DataTypes::Path => {
                let s_count = u16_from(content);
                let mut ind = size_of::<u16>();
                let mut segments = VecDeque::with_capacity(s_count as usize);
                for _ in 0..s_count {
                    let len = usize_from(&content[ind..]);
                    let val = string_from(&content[(ind + SIZE_USIZE)..(ind + SIZE_USIZE + len)])?;
                    ind += SIZE_USIZE + len;
                    segments.push_back(val);
                }
                Ok(Value::Path(segments))
            }
            DataTypes::Table => {
                let kv_count = u32_from(content);
                let mut ind = size_of::<u32>();
                let mut table: HashMap<String, Value> = HashMap::with_capacity(kv_count as usize);
                for _ in 0..kv_count {
                    let key_len = usize_from(&content[ind..(ind + SIZE_USIZE)]);
                    ind += SIZE_USIZE;
                    let key = string_from(&content[ind..(ind + key_len)])?;
                    ind += key_len;
                    let val_len = usize_from(&content[ind..(ind + SIZE_USIZE)]);
                    ind += SIZE_USIZE;
                    let val_wrapper = DataWrapper::new(&content[ind..(ind + val_len)])?;
                    ind += val_len;
                    let value = val_wrapper.data()?;
                    table.insert(key, value);
                }
                Ok(Value::Table(Box::from(Table::with_hashmap(table))))
            }
        }
    }
    fn size(&self) -> usize {
        //println!("dw {}", self.content.len());
        self.content.len()
    }
    fn read_into(&self, buf: &mut [u8]) -> AppResult<()> {
        if buf.len() == self.content.len() {
            buf.copy_from_slice(self.content);
            Ok(())
        } else {
            Err(AppError::SizeCalculationIsInvalid)
        }
    }
}

pub struct ArgIterWrapper<'a> {
    index: usize,
    args: &'a Vec<DataWrapper<'a>>,
}

impl<'a> ArgIter<'a> for ArgIterWrapper<'a> {
    fn get(&mut self) -> Option<&'a dyn Data<'a>> {
        let data = self.args.get(self.index);
        self.index += 1;
        data.map(|d| {
            let r: &dyn Data<'a> = d;
            r
        })
    }
}
