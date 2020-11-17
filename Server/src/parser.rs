use crate::*;

pub trait Parser {
    fn parse_command<T>(data: &[u8]) -> Result<T, MyError>
    where
        T: Command;

    fn serialize_command_result<CR>(comm: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult;
}
