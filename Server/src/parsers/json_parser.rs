use crate::common_traits::*;
use crate::data::*;
use crate::error::*;
use serde_json::from_reader;
use serde_json::value::Value;
use std::io::{BufRead, BufReader, Read};

///This parser reads until `\n` and parses input as json and serializes results into json.  
///
///This parser is mainly used for easy implementation and to enable a rest api

pub struct JsonParser;

impl JsonParser {
    fn read_json_object<S>(stream: &mut S) -> Result<Value, MyError>
    where
        S: Read,
    {
        let reader = BufReader::new(stream);
        let obj: Value = from_reader(reader).map_err(|_| MyError::MalformedCommand)?;
        Ok(obj)
    }
}

impl<T, EC> Parser<JsonCommand<T>, EC, T> for JsonParser
where
    EC: EventCommand,
    T: Table,
{
    fn read_intent<S>(stream: &mut S) -> Result<StreamIntent, MyError>
    where
        S: Read,
    {
        let obj = Self::read_json_object(stream)?;
        let s = obj.get("intent");
        unimplemented!()
    }

    fn read_command<S>(stream: &mut S) -> Result<JsonCommand<T>, MyError>
    where
        S: Read,
    {
        unimplemented!()
    }

    fn read_ev_command<S>(stream: &mut S) -> Result<EC, MyError>
    where
        S: Read,
    {
        unimplemented!()
    }

    fn parse_command(data: &[u8]) -> Result<JsonCommand<T>, MyError> {
        unimplemented!()
    }

    fn parse_ev_command(data: &[u8]) -> Result<EC, MyError> {
        unimplemented!()
    }

    fn serialize_ev_content<CO>(con: CO) -> Result<Vec<u8>, MyError>
    where
        CO: EventContent,
    {
        unimplemented!()
    }

    fn serialize_command_result<CR>(comm: CR) -> Result<Vec<u8>, MyError>
    where
        CR: CommandResult,
    {
        unimplemented!()
    }
}

pub struct JsonCommand<T>
where
    T: Table,
{
    arge: Vec<Data<T>>,
}

impl<T> Command for JsonCommand<T>
where
    T: Table,
{
    type Table = T;

    fn get_path<'a>(&'a self) -> Option<&'a str> {
        unimplemented!()
    }

    fn is_mutator(&self) -> bool {
        unimplemented!()
    }

    fn is_terminate(&self) -> bool {
        unimplemented!()
    }

    fn get_operation(&self) -> Operation {
        unimplemented!()
    }

    fn get_args_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a CommandArg<Self::Table>> + 'a> {
        unimplemented!()
    }
}
