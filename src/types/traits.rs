use crate::*;

pub trait Command<'a> {
    fn command_type(&self) -> &CommandTypes;
    fn args(&'a self) -> Box<dyn ArgIter<'a> + 'a>;
    fn read_into(&self, buf: &mut [u8]) -> AppResult<()>;
    fn size(&self) -> usize;
}

pub trait Data<'a> {
    fn data(&self) -> AppResult<Value>;
    fn data_type(&self) -> &DataTypes;
    fn read_into(&self, buf: &mut [u8]) -> AppResult<()>;
    fn size(&self) -> usize;
}

pub trait ArgIter<'a> {
    fn get(&mut self) -> Option<&'a dyn Data<'a>>;
}

impl<'a> Iterator for Box<dyn ArgIter<'a>> {
    type Item = &'a dyn Data<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.get()
    }
}
