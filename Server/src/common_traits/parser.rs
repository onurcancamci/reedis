use crate::*;

pub trait Parser {
    type ParsedCommand;
    type ParsedEvCommand;

    fn parse_command(data: &[u8]) -> Result<Self::ParsedCommand, MyError>;

    fn parse_ev_command(data: &[u8]) -> Result<Self::ParsedEvCommand, MyError>;

    fn serialize_command_result<CR>(comm: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult;

    fn serialize_ev_content<CO>(con: CO) -> Result<Vec<u8>, MyError>
    where
        CO: EventContent;
}
