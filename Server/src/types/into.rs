use crate::*;

#[derive(Debug, Clone)]
pub struct CommandInto {
    pub command: CommandTypes,
    pub args: Vec<DataInto>,
}

impl CommandInto {
    pub fn new_raw(command: CommandTypes, args: Vec<DataInto>) -> Self {
        CommandInto { command, args }
    }
}

impl Command<'_> for CommandInto {
    fn command_type(&self) -> &CommandTypes {
        &self.command
    }
    fn args<'a>(&'a self) -> Box<dyn ArgIter<'a> + 'a> {
        let ai: ArgIterInto<'a> = ArgIterInto {
            index: 0,
            args: &self.args,
        };
        Box::new(ai)
    }
    fn size(&self) -> usize {
        2 + SIZE_USIZE * self.args.len() + self.args.iter().fold(0, |acc, a| acc + a.size())
    }
    fn read_into(&self, buf: &mut [u8]) -> AppResult<()> {
        buf[0..2].copy_from_slice(&(self.command.clone() as u16).to_le_bytes());
        let mut ind = 2;
        for arg in &self.args {
            let asize = arg.size();
            buf[ind..(ind + SIZE_USIZE)].copy_from_slice(&asize.to_le_bytes());
            ind += SIZE_USIZE;
            arg.read_into(&mut buf[ind..(ind + asize)])?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct DataInto {
    pub content: Value,
    pub data_type: DataTypes,
}

impl DataInto {
    pub fn new_raw(data_type: DataTypes, content: Value) -> Self {
        DataInto { content, data_type }
    }
}

impl Data<'_> for DataInto {
    fn data_type(&self) -> &DataTypes {
        &self.data_type
    }
    fn data(&self) -> AppResult<Value> {
        Ok(self.content.clone())
    }
    fn size(&self) -> usize {
        self.content.size()
    }
    fn read_into(&self, buf: &mut [u8]) -> AppResult<()> {
        buf[0..2].copy_from_slice(&(self.data_type.clone() as u16).to_le_bytes());
        let mut to_write = &mut buf[2..];
        self.content.read_into(&mut to_write)?;
        Ok(())
    }
}

pub struct ArgIterInto<'a> {
    index: usize,
    args: &'a Vec<DataInto>,
}

impl<'a> ArgIter<'a> for ArgIterInto<'a> {
    fn get(&mut self) -> Option<&'a dyn Data<'a>> {
        let data = self.args.get(self.index);
        self.index += 1;
        data.map(|d| {
            let r: &dyn Data<'a> = d;
            r
        })
    }
}
