use crate::*;

pub trait Parser {
    fn parse_command<T>(data: &[u8]) -> Result<T, MyError>
    where
        T: Command;

    fn parse_ev_command<T>(data: &[u8]) -> Result<T, MyError>
    where
        T: EventCommand;

    fn serialize_command_result<CR>(comm: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult;

    fn serialize_ev_content<CO>(con: CO) -> Result<Vec<u8>, MyError>
    where
        CO: EventContent;
}
