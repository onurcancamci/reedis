use crate::common_traits::*;
use crate::data::{Data, DataType};
use crate::error::MyError;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct MockDatabase {
    table: Box<MockTable>,
}

impl Database<MockEvent> for MockDatabase {
    type Table = MockTable;
    type CommandResult = MockCommandResult;
    type Command = MockCommand;

    fn table(&self) -> &Self::Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Self::Table {
        &mut self.table
    }
}

impl MockDatabase {
    pub fn new() -> Self {
        MockDatabase {
            table: MockTable::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MockTable {
    map: HashMap<String, MockField>,
}

impl Table for MockTable {
    type Field = MockField;

    fn new() -> Box<Self> {
        Box::new(MockTable {
            map: HashMap::new(),
        })
    }

    fn get_field(&self, key: &str) -> Option<&Self::Field> {
        self.map.get(key)
    }

    fn get_field_mut(&mut self, key: &str) -> Option<&mut Self::Field> {
        self.map.get_mut(key)
    }

    fn set_field(&mut self, key: &str, field: Self::Field) -> Result<(), MyError> {
        if !self.map.contains_key(key) {
            self.map.insert(key.to_string(), field);
            Ok(())
        } else {
            Err(MyError::KeyAlreadyExists)
        }
    }

    fn keys_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a str> + 'a> {
        Box::new(self.map.keys().map(|e| e.as_str()))
    }
}

impl TableMethods<MockEvent> for MockTable {
    type Command = MockCommand;
    type CommandResult = MockCommandResult;
}

#[derive(Clone, Debug, PartialEq)]
pub struct MockField {
    data: Data<MockTable>,
    listeners: HashSet<usize>,
    child_listeners: usize,
}

impl Field for MockField {
    type Table = MockTable;

    fn create_with_data(data: Data<Self::Table>) -> Self {
        MockField {
            data,
            listeners: HashSet::new(),
            child_listeners: 0,
        }
    }

    fn get_data(&self) -> &Data<Self::Table> {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut Data<Self::Table> {
        &mut self.data
    }

    fn data_type(&self) -> DataType {
        self.data.data_type()
    }
}

#[derive(Debug)]
pub struct MockEvent {
    target: usize,
    content: MockEventContent,
}

impl Event for MockEvent {
    type Content = MockEventContent;

    fn new(path: &str, op: Operation, target: usize) -> Self {
        MockEvent {
            target,
            content: MockEventContent {
                path: path.to_string(),
                operation: op,
            },
        }
    }

    fn get_target(&self) -> usize {
        self.target
    }

    fn get_content(&self) -> &Self::Content {
        &self.content
    }
}

#[derive(Debug, Clone)]
pub struct MockEventContent {
    path: String,
    operation: Operation,
}

impl EventContent for MockEventContent {}

#[derive(Debug)]
pub struct MockCommand {
    path: Option<String>,
    terminate: bool,
    operation: Operation,
    mutator: bool,
    args: Vec<CommandArg<MockTable, Self>>,
}

impl Command for MockCommand {
    type Table = MockTable;

    fn new_with_vec(
        op: Operation,
        path: Option<String>,
        args: Vec<CommandArg<Self::Table, Self>>,
    ) -> Self {
        unreachable!()
    }

    fn get_path<'a>(&'a self) -> Option<&'a str> {
        self.path.as_deref()
    }

    fn is_mutator(&self) -> bool {
        self.mutator
    }

    fn is_terminate(&self) -> bool {
        self.terminate
    }

    fn get_operation(&self) -> Operation {
        self.operation.clone()
    }

    fn get_args_iter<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'a CommandArg<MockTable, Self>> + 'a> {
        Box::from(self.args.iter())
    }
}

impl MockCommand {
    pub fn new_terminate() -> Self {
        MockCommand {
            path: None,
            terminate: true,
            operation: Operation::Terminate,
            mutator: false,
            args: vec![],
        }
    }

    pub fn new_get(path: &str) -> Self {
        MockCommand {
            path: Some(path.to_string()),
            terminate: false,
            operation: Operation::Get,
            mutator: false,
            args: vec![],
        }
    }

    pub fn new_set(path: &str, data: Data<MockTable>) -> Self {
        MockCommand {
            path: Some(path.to_string()),
            terminate: false,
            operation: Operation::Set,
            mutator: false,
            args: vec![CommandArg::Data(data)],
        }
    }
}

#[derive(Debug)]
pub enum MockCommandResult {
    Ok(Vec<Data<MockTable>>, usize),
    Err(MyError, usize),
}

impl CommandResult for MockCommandResult {
    type Table = MockTable;

    fn modified_row_count(&self) -> usize {
        *match self {
            MockCommandResult::Ok(_, c) => c,
            MockCommandResult::Err(_, c) => c,
        }
    }

    fn results<'a>(
        &'a self,
    ) -> Result<Box<dyn Iterator<Item = &'a Data<Self::Table>> + 'a>, MyError> {
        match self {
            MockCommandResult::Ok(result, _) => Ok(Box::new(result.iter())),
            MockCommandResult::Err(err, _) => Err(err.clone()),
        }
    }

    fn new_data_result(data: impl Iterator<Item = Data<Self::Table>>, mod_count: usize) -> Self {
        let vec = data.collect();
        MockCommandResult::Ok(vec, mod_count)
    }

    fn new_error_result(err: MyError, mod_count: usize) -> Self {
        MockCommandResult::Err(err, mod_count)
    }
}

pub struct MockEventTable;

impl EventTable for MockEventTable {
    fn listen(&mut self, path: &str, listener: usize) {
        unreachable!();
    }

    fn unlisten(&mut self, path: &str, listener: usize) {
        unreachable!();
    }

    fn unlisten_listener(&mut self, listener: usize) {
        unreachable!();
    }

    fn lookup<'a>(&'a self, path: &str) -> Box<dyn Iterator<Item = usize> + 'a> {
        unreachable!();
    }
}

pub struct MockExecutionContext;

impl ExecutionContext<MockEvent> for MockExecutionContext {
    type EventTable = MockEventTable;

    fn tx_event(&self) -> &std::sync::mpsc::Sender<MockEvent> {
        unreachable!();
    }

    fn event_table(&self) -> &Self::EventTable {
        unreachable!();
    }
}
