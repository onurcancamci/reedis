use crate::common_traits::*;
use crate::error::MyError;
use std::io::Read;

pub trait Parser<PC, PE, T>
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

    fn read_command<S>(stream: &mut S) -> Result<PC, MyError>
    where
        S: Read;

    fn read_ev_command<S>(stream: &mut S) -> Result<PE, MyError>
    where
        S: Read;

    fn read_intent<S>(stream: &mut S) -> Result<StreamIntent, MyError>
    where
        S: Read;
}

pub enum StreamIntent {
    Data,
    Event,
}
