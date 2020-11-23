use crate::common_traits::*;
use crate::error::MyError;

macro_rules! parser_type {
    ($($name: ident),*) => {
        $(
        pub trait $name<PC, PE, T, E>: Parser<PC, PE, T, E>
        where
            PC: Command<Table = T>,
            PE: EventCommand,
            T: Table,
        {
        }

        impl<PC, PE, T, E> $name<PC, PE, T, E> for InvalidParser
        where
            PC: Command<Table = T>,
            PE: EventCommand,
            T: Table,
        {
        }
        )*
    };
}

pub trait ParserCollection<PC, PE, T, E>
where
    PC: Command<Table = T>,
    PE: EventCommand,
    T: Table,
{
    type Readable: ReadableParser<PC, PE, T, E>;
    type Intermediate: IntermediateParser<PC, PE, T, E>;
    type MachineReadable: MachineReadableParser<PC, PE, T, E>;
    type Custom: CustomParser<PC, PE, T, E>;
}

parser_type!(
    ReadableParser,
    IntermediateParser,
    MachineReadableParser,
    CustomParser
);

pub struct InvalidParser;

impl<PC, PE, T, E> Parser<PC, PE, T, E> for InvalidParser
where
    PC: Command<Table = T>,
    PE: EventCommand,
    T: Table,
{
    const IS_IMPLEMENTED: bool = false;

    fn parse_command(_: &[u8]) -> Result<PC, MyError> {
        panic!("INVALID PARSER")
    }

    fn parse_ev_command(_: &[u8]) -> Result<PE, MyError> {
        panic!("INVALID PARSER")
    }

    fn serialize_ev_content<CO>(_: CO) -> Result<Vec<u8>, MyError>
    where
        CO: EventContent,
    {
        panic!("INVALID PARSER")
    }

    fn serialize_command_result<CR>(_: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult,
    {
        panic!("INVALID PARSER")
    }

    fn read_command<S>(_: S) -> PC {
        panic!("INVALID PARSER")
    }

    fn read_ev_command<S>(_: S) -> PE {
        panic!("INVALID PARSER")
    }

    fn is_correct_parser<S>(_: S) -> Option<usize> {
        None
    }
}
