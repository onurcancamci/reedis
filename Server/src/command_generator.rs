use crate::*;
pub struct CommGen;

impl CommGen {
    pub fn result(val: Value) -> CommandInto {
        CommandInto {
            command: CommandTypes::Result,
            args: vec![DataInto {
                data_type: val.data_type(),
                content: val,
            }],
        }
    }
}
