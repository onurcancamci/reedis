use crate::common_traits::*;
use crate::error::MyError;

pub trait Parser<PC, PE, T, E>
where
    PC: Command<Table = T>,
    PE: EventCommand,
    T: Table,
{
    fn parse_command(data: &[u8]) -> Result<PC, MyError>;

    fn parse_ev_command(data: &[u8]) -> Result<PE, MyError>;

    fn serialize_command_result<CR>(comm: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult;

    fn serialize_ev_content<CO>(con: CO) -> Result<Vec<u8>, MyError>
    where
        CO: EventContent;
}
