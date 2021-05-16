use crate::data::*;
use crate::error::*;
use crate::{common_traits::*, implementation::MyCommand};
use serde_json::from_reader;
use serde_json::value::Value;
use std::io::{BufReader, Read};

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

    fn json_to_data<T: Table>(val: Value) -> Result<Data<T>, MyError> {
        Ok(match val {
            Value::Null => Data::Null,
            Value::Bool(n) => Data::Bool(n),
            Value::Number(n) => Data::Float(n.as_f64().ok_or(MyError::NumberCantBeParsed)? as f32),
            Value::String(n) => Data::Str(n),
            Value::Array(arr) => {
                let olen = arr.len();
                let results: Vec<Data<T>> = arr
                    .into_iter()
                    .map(|e| Self::json_to_data(e).ok())
                    .flatten()
                    .collect();
                if results.len() != olen {
                    Err(MyError::ArrayParseError)?;
                }
                Data::Array(results)
            }
            Value::Object(o) => {
                let mut t = T::new();
                for el in o {
                    t.insert_data(el.0.as_str(), Self::json_to_data(el.1)?)?;
                }
                Data::Table(t)
            }
        })
    }
}

impl<T> Parser<MyCommand<T>, T> for JsonParser
where
    T: Table,
{
    fn read_intent<S>(stream: &mut S) -> Result<StreamIntent, MyError>
    where
        S: Read,
    {
        let obj = Self::read_json_object(stream)?;
        let s = obj.get("intent");
        if let Some(Value::String(intent)) = s {
            if intent == "data" {
                Ok(StreamIntent::Data)
            } else {
                Ok(StreamIntent::Event)
            }
        } else {
            Err(MyError::MalformedCommand)
        }
    }

    // TODO: remove unwraps and return error
    fn read_command<S>(stream: &mut S) -> Result<MyCommand<T>, MyError>
    where
        S: Read,
    {
        let obj = Self::read_json_object(stream)?;
        let op_str = obj.get("op").unwrap().as_str().unwrap();

        unimplemented!()
    }

    fn read_ev_command<S>(stream: &mut S) -> Result<EventCommand, MyError>
    where
        S: Read,
    {
        unimplemented!()
    }

    fn parse_command(data: &[u8]) -> Result<MyCommand<T>, MyError> {
        unimplemented!()
    }

    fn parse_ev_command(data: &[u8]) -> Result<EventCommand, MyError> {
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
