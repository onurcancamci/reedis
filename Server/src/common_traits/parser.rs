use crate::common_traits::*;
use crate::error::MyError;
use std::io::Read;

pub trait Parser<PC, PE, T, E>
where
    PC: Command<Table = T>,
    PE: EventCommand,
    T: Table,
{
    const IS_IMPLEMENTED: bool = true;

    fn parse_command(data: &[u8]) -> Result<PC, MyError>;

    fn parse_ev_command(data: &[u8]) -> Result<PE, MyError>;

    fn serialize_command_result<CR>(comm: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult;

    fn serialize_ev_content<CO>(con: CO) -> Result<Vec<u8>, MyError>
    where
        CO: EventContent;

    //TODO: define read_message and read_intent here

    fn read_command<S>(socket: S) -> PC
    where
        S: Read;

    fn read_ev_command<S>(socket: S) -> PE
    where
        S: Read;

    ///This function peeks at incoming message and decides if message is meant for this parser.
    ///If correct, returns peeked size in order to drain those bytes. If false returns `None`
    fn is_correct_parser<S>(socket: S) -> Option<usize>
    where
        S: Read + Peek;
}
